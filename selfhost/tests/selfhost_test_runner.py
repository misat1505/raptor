"""End-to-end tests for Raptor compilers (the Rust one and the self-hosted one).

A test case is a black box: compiler + program + flags -> compile result, stdout,
stderr, exit code (optionally under valgrind). Both compilers share the same CLI,
so the same cases run against either of them.

    python3 ./selfhost/tests/selfhost_test_runner.py run    --compiler ./target/release/raptor                  # Rust compiler
    python3 ./selfhost/tests/selfhost_test_runner.py run    --compiler ./bootstrap/raptor --memory arc          # self-hosted
    python3 ./selfhost/tests/selfhost_test_runner.py run    --compiler ./bootstrap/raptor --memory leak -k 'memory/*'
    python3 ./selfhost/tests/selfhost_test_runner.py diff   --compiler ./target/release/raptor --compiler-b ./bootstrap/raptor examples/*.rp
    python3 ./selfhost/tests/selfhost_test_runner.py import --rust-tests tests --out selfhost/tests/cases            # one-off migration

Layout (written by `import`, edited by hand afterwards):

    cases/<group>/<name>.rp        the program
    cases/<group>/<name>.expect    what must happen

Format of `.expect` (every line is optional):

    flags: --overflow error                  extra compiler flags (shell syntax)
    compile: ok | fail                       must the compiler accept the program? (default ok)
    compile-stderr-contains: "text"          repeatable, JSON string or plain text
    exit: 1 | nonzero                        exit code of the program (default 0)
    valgrind: yes | no                       default: yes when exit is 0, otherwise no
    stderr: empty                            the program must not write to stderr
    stderr-contains: "text"                  repeatable
    stderr-contains-i: "text"                case-insensitive, repeatable
    stderr-contains-any: ["a", "b"]          at least one of the texts, repeatable
    stderr-contains-any-i: ["a", "b"]        same, case-insensitive
    --- stdout ---                           everything after this line is the exact stdout
    first line of expected output
    ...

Only the Python standard library is used.
"""

import argparse
import concurrent.futures as cf
import fnmatch
import json
import os
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import time

ANSI = re.compile(r"\x1b\[[0-9;]*m")
STDOUT_MARKER = "--- stdout ---"


def strip_ansi(text):
    return ANSI.sub("", text)


# ----------------------------------------------------------------------------
# .expect files
# ----------------------------------------------------------------------------


def parse_value(value):
    value = value.strip()
    if value.startswith('"'):
        return json.loads(value)
    return value


def parse_expect(path):
    exp = {
        "flags": [],
        "compile": "ok",
        "compile_contains": [],
        "exit": 0,
        "exit_nonzero": False,
        "valgrind": None,
        "stderr_empty": False,
        "stderr_contains": [],
        "stderr_contains_i": [],
        "stderr_any": [],
        "stdout": None,
    }
    with open(path, encoding="utf-8", newline="") as fh:
        text = fh.read()
    head, sep, body = text.partition(STDOUT_MARKER + "\n")
    if sep:
        exp["stdout"] = body
    for raw in head.splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        key, _, value = line.partition(":")
        key = key.strip()
        if key == "flags":
            exp["flags"] = shlex.split(value)
        elif key == "compile":
            exp["compile"] = value.strip()
        elif key == "compile-stderr-contains":
            exp["compile_contains"].append(parse_value(value))
        elif key == "exit":
            if value.strip() == "nonzero":
                exp["exit_nonzero"] = True
            else:
                exp["exit"] = int(value)
        elif key == "valgrind":
            exp["valgrind"] = value.strip() in ("yes", "true", "1")
        elif key == "stderr":
            exp["stderr_empty"] = value.strip() == "empty"
        elif key == "stderr-contains":
            exp["stderr_contains"].append(parse_value(value))
        elif key == "stderr-contains-i":
            exp["stderr_contains_i"].append(parse_value(value))
        elif key in ("stderr-contains-any", "stderr-contains-any-i"):
            exp["stderr_any"].append({"texts": json.loads(value), "ci": key.endswith("-i")})
        else:
            raise ValueError(f"{path}: unknown key {key!r}")
    if exp["valgrind"] is None:
        exp["valgrind"] = exp["exit"] == 0 and not exp["exit_nonzero"] and exp["compile"] == "ok"
    return exp


def write_expect(path, exp):
    lines = []
    if exp.get("flags"):
        lines.append("flags: " + " ".join(shlex.quote(f) for f in exp["flags"]))
    if exp.get("compile", "ok") != "ok":
        lines.append("compile: " + exp["compile"])
    if exp.get("exit_nonzero"):
        lines.append("exit: nonzero")
    elif exp.get("exit", 0) != 0:
        lines.append(f"exit: {exp['exit']}")
    default_valgrind = exp.get("exit", 0) == 0 and not exp.get("exit_nonzero")
    if exp.get("valgrind", default_valgrind) != default_valgrind:
        lines.append("valgrind: " + ("yes" if exp["valgrind"] else "no"))
    if exp.get("stderr_empty"):
        lines.append("stderr: empty")
    for needle in exp.get("stderr_contains", []):
        lines.append("stderr-contains: " + json.dumps(needle))
    for needle in exp.get("stderr_contains_i", []):
        lines.append("stderr-contains-i: " + json.dumps(needle))
    for group in exp.get("stderr_any", []):
        key = "stderr-contains-any-i" if group["ci"] else "stderr-contains-any"
        lines.append(key + ": " + json.dumps(group["texts"]))
    text = "\n".join(lines) + ("\n" if lines else "")
    if exp.get("stdout") is not None:
        text += STDOUT_MARKER + "\n" + exp["stdout"]
    with open(path, "w", encoding="utf-8", newline="") as fh:
        fh.write(text)


# ----------------------------------------------------------------------------
# running one program
# ----------------------------------------------------------------------------


class Outcome:
    def __init__(self):
        self.compile_ok = False
        self.compile_log = ""
        self.stdout = ""
        self.stderr = ""
        self.exit = None
        self.valgrind_failed = False
        self.workdir = None


def supports_memory_flag(compiler):
    try:
        r = subprocess.run([compiler, "-h"], capture_output=True, text=True, timeout=30)
    except (OSError, subprocess.TimeoutExpired):
        return False
    return "--memory" in r.stdout


def build_and_run(opts, rp_path, flags, use_valgrind, stdin_text=None):
    """Compile `rp_path` with opts.compiler and run it. Returns an Outcome."""
    out = Outcome()
    work = tempfile.mkdtemp(prefix="raptor-e2e-")
    out.workdir = work
    try:
        shutil.copy(rp_path, os.path.join(work, "t.rp"))
        cmd = [opts.compiler, "--compile", "-o", "./exe", "t.rp"] + list(flags)
        if opts.memory and opts.has_memory_flag:
            cmd += ["--memory", opts.memory]
        c = subprocess.run(
            cmd, cwd=work, capture_output=True, text=True, timeout=opts.timeout
        )
        out.compile_log = strip_ansi(c.stderr + c.stdout)
        out.compile_ok = c.returncode == 0 and os.path.exists(os.path.join(work, "exe"))
        if not out.compile_ok:
            return out
        run = [os.path.join(work, "exe")]
        if use_valgrind:
            leaks = "--leak-check=no" if opts.memory == "leak" else "--leak-check=full"
            run = [
                "valgrind",
                leaks,
                "--errors-for-leak-kinds=definite,indirect,possible",
                "--error-exitcode=99",
                "--quiet",
            ] + run
        r = subprocess.run(
            run,
            cwd=work,
            capture_output=True,
            text=True,
            timeout=opts.timeout,
            input=stdin_text if stdin_text is not None else "",
        )
        out.stdout = r.stdout
        out.stderr = strip_ansi(r.stderr)
        out.exit = r.returncode
        out.valgrind_failed = use_valgrind and r.returncode == 99
        return out
    except BaseException:
        shutil.rmtree(work, ignore_errors=True)
        raise


def cleanup(out, keep):
    if out.workdir and not keep:
        shutil.rmtree(out.workdir, ignore_errors=True)


# ----------------------------------------------------------------------------
# `run`
# ----------------------------------------------------------------------------


def check_case(opts, rp_path, exp):
    """Returns (status, detail) where status is 'ok' or 'FAIL'."""
    out = None
    try:
        out = build_and_run(
            opts, rp_path, exp["flags"], exp["valgrind"] and opts.valgrind
        )
        problems = []
        if exp["compile"] == "fail":
            if out.compile_ok:
                problems.append("the compiler accepted a program that must be rejected")
            for needle in exp["compile_contains"]:
                if needle not in out.compile_log:
                    problems.append(f"compiler output does not contain {needle!r}")
            if problems:
                problems.append("compiler output:\n" + out.compile_log[-400:])
        else:
            if not out.compile_ok:
                return "FAIL", "compilation failed:\n" + out.compile_log[-500:]
            if out.valgrind_failed:
                problems.append("valgrind: memory error or leak\n" + out.stderr[-500:])
            elif exp["exit_nonzero"]:
                if out.exit == 0:
                    problems.append("exit code 0, expected a non-zero exit code")
            elif out.exit != exp["exit"]:
                problems.append(f"exit code {out.exit}, expected {exp['exit']}")
            if exp["stdout"] is not None and out.stdout != exp["stdout"]:
                problems.append(
                    f"stdout differs\n  expected: {exp['stdout']!r}\n  actual:   {out.stdout!r}"
                )
            if exp["stderr_empty"] and out.stderr.strip():
                problems.append(f"stderr should be empty, got {out.stderr[:300]!r}")
            for needle in exp["stderr_contains"]:
                if needle not in out.stderr:
                    problems.append(
                        f"stderr does not contain {needle!r}\n  stderr: {out.stderr[:300]!r}"
                    )
            for needle in exp["stderr_contains_i"]:
                if needle.lower() not in out.stderr.lower():
                    problems.append(
                        f"stderr does not contain {needle!r} (ignoring case)\n  stderr: {out.stderr[:300]!r}"
                    )
            for group in exp["stderr_any"]:
                hay = out.stderr.lower() if group["ci"] else out.stderr
                if not any((t.lower() if group["ci"] else t) in hay for t in group["texts"]):
                    problems.append(
                        f"stderr contains none of {group['texts']!r}\n  stderr: {out.stderr[:300]!r}"
                    )
        if problems:
            return "FAIL", "\n".join(problems)
        return "ok", ""
    except subprocess.TimeoutExpired:
        return "FAIL", f"timeout after {opts.timeout}s"
    finally:
        if out is not None:
            cleanup(out, keep=False)


def discover(cases_dir, pattern):
    found = []
    for root, _, files in sorted(os.walk(cases_dir)):
        for f in sorted(files):
            if not f.endswith(".rp"):
                continue
            group = os.path.relpath(root, cases_dir).replace(os.sep, "/")
            name = f"{group}/{f[:-3]}"
            expect = os.path.join(root, f[:-3] + ".expect")
            if fnmatch.fnmatch(name, pattern) and os.path.exists(expect):
                found.append((name, os.path.join(root, f), expect))
    return found


def cmd_run(opts):
    opts.has_memory_flag = supports_memory_flag(opts.compiler)
    if opts.memory and not opts.has_memory_flag:
        print(
            f"note: {opts.compiler} has no --memory flag, ignoring --memory {opts.memory}",
            file=sys.stderr,
        )
    xfail = set()
    if opts.xfail:
        for line in open(opts.xfail, encoding="utf-8"):
            line = line.split("#")[0].strip()
            if line:
                xfail.add(line)
    cases = discover(opts.cases, opts.k)
    if not cases:
        print(f"no cases found in {opts.cases} matching {opts.k!r}", file=sys.stderr)
        return 2

    started = time.time()
    results = []
    with cf.ThreadPoolExecutor(opts.j) as pool:
        futures = {}
        for name, rp, expect in cases:
            futures[pool.submit(check_case, opts, rp, parse_expect(expect))] = name
        for fut in cf.as_completed(futures):
            status, detail = fut.result()
            results.append((futures[fut], status, detail))
            if opts.progress:
                print(("." if status == "ok" else "F"), end="", flush=True)
    if opts.progress:
        print()
    results.sort()

    failures, expected_failures, unexpected_passes = [], 0, []
    for name, status, detail in results:
        if status == "ok":
            if name in xfail:
                unexpected_passes.append(name)
            elif opts.v:
                print(f"  ok    {name}")
        elif name in xfail:
            expected_failures += 1
        else:
            failures.append((name, detail))
    for name, detail in failures:
        print(f"  FAIL  {name}")
        for line in detail.splitlines()[:12]:
            print("        " + line)
    passed = sum(1 for r in results if r[1] == "ok")
    label = os.path.basename(opts.compiler) + (
        f" --memory {opts.memory}" if opts.memory and opts.has_memory_flag else ""
    )
    print(
        f"\n{label}: {passed}/{len(results)} passed, {len(failures)} failed, "
        f"{expected_failures} expected failures, {len(unexpected_passes)} unexpectedly passing "
        f"({time.time() - started:.0f}s)"
    )
    for name in unexpected_passes:
        print(f"  XPASS (remove it from the xfail file): {name}")
    return 1 if failures or unexpected_passes else 0


# ----------------------------------------------------------------------------
# `diff`: run the same programs with two compilers and compare everything
# ----------------------------------------------------------------------------


def cmd_diff(opts):
    files = []
    for p in opts.files:
        if os.path.isdir(p):
            for root, _, names in sorted(os.walk(p)):
                files += [
                    os.path.join(root, n) for n in sorted(names) if n.endswith(".rp")
                ]
        else:
            files.append(p)
    flags = shlex.split(opts.flags or "")

    def one(path):
        results = []
        for compiler in (opts.compiler, opts.compiler_b):
            o = argparse.Namespace(**vars(opts))
            o.compiler = os.path.abspath(compiler)
            o.has_memory_flag = supports_memory_flag(o.compiler)
            results.append(build_and_run(o, path, flags, opts.valgrind))
        a, b = results
        try:
            if not a.compile_ok and not b.compile_ok:
                return path, "both reject", ""
            if a.compile_ok != b.compile_ok:
                who = "A" if not a.compile_ok else "B"
                return (
                    path,
                    "DIFF",
                    f"compiler {who} rejects it:\n"
                    + (a if who == "A" else b).compile_log[-300:],
                )
            diffs = []
            for field in ("exit", "stdout", "stderr"):
                x, y = getattr(a, field), getattr(b, field)
                if x != y:
                    diffs.append(f"{field}:\n  A: {x!r:.200}\n  B: {y!r:.200}")
            return path, ("DIFF" if diffs else "ok"), "\n".join(diffs)
        finally:
            cleanup(a, False)
            cleanup(b, False)

    bad = 0
    with cf.ThreadPoolExecutor(opts.j) as pool:
        for path, status, detail in pool.map(one, files):
            if status in ("ok", "both reject"):
                print(f"  {status:<11} {path}")
            else:
                bad += 1
                print(f"  {status:<11} {path}")
                for line in detail.splitlines()[:10]:
                    print("        " + line)
    print(f"\n{len(files) - bad}/{len(files)} identical")
    return 1 if bad else 0


# ----------------------------------------------------------------------------
# `import`: one-off migration of the Rust integration tests
# ----------------------------------------------------------------------------


def parse_rust_string(src, i):
    """Parse a Rust string literal at src[i]. Returns (value, end) or None."""
    m = re.compile(r'r(#*)"').match(src, i)
    if m:
        close = '"' + m.group(1)
        j = src.index(close, m.end())
        return src[m.end() : j], j + len(close)
    if src[i] != '"':
        return None
    out, j = [], i + 1
    while src[j] != '"':
        c = src[j]
        if c != "\\":
            out.append(c)
            j += 1
            continue
        n = src[j + 1]
        simple = {
            "n": "\n",
            "t": "\t",
            "r": "\r",
            "0": "\0",
            '"': '"',
            "\\": "\\",
            "'": "'",
        }
        if n in simple:
            out.append(simple[n])
            j += 2
        elif n == "\n":  # line continuation
            j += 2
            while src[j] in " \t\n":
                j += 1
        elif n == "u":
            k = src.index("}", j)
            out.append(chr(int(src[j + 3 : k], 16)))
            j = k + 1
        else:
            return None
    return "".join(out), j + 1


POLICY = {"Ignore": "ignore", "Warn": "warn", "Error": "error"}


def iter_assertions(body):
    """Yields (kind, args) for every assert!/assert_eq!/assert_ne! in a test body.

    `args` are the top-level, comma separated arguments as source text.
    """
    for m in re.finditer(r"\bassert(_eq|_ne)?!\(", body):
        j, depth, cur, args = m.end(), 1, [], []
        while True:
            c = body[j]
            if c == '"' or re.match(r'r#*"', body[j : j + 8]):
                lit = parse_rust_string(body, j)
                cur.append(body[j : lit[1]])
                j = lit[1]
                continue
            if c in "([{":
                depth += 1
            elif c in ")]}":
                depth -= 1
                if depth == 0:
                    break
            if c == "," and depth == 1:
                args.append("".join(cur).strip())
                cur = []
                j += 1
                continue
            cur.append(c)
            j += 1
        args.append("".join(cur).strip())
        yield m.group(1) or "", args


def split_top_level(text, sep):
    """Splits `text` on `sep`, ignoring separators inside strings and brackets."""
    parts, cur, depth, j = [], [], 0, 0
    while j < len(text):
        c = text[j]
        if c == '"' or re.match(r'r#*"', text[j : j + 8]):
            lit = parse_rust_string(text, j)
            cur.append(text[j : lit[1]])
            j = lit[1]
            continue
        if text.startswith(sep, j) and depth == 0:
            parts.append("".join(cur).strip())
            cur = []
            j += len(sep)
            continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        cur.append(c)
        j += 1
    parts.append("".join(cur).strip())
    return parts


def string_literal(text):
    lit = parse_rust_string(text.strip(), 0)
    if lit is None or lit[1] != len(text.strip()):
        return None
    return lit[0]


def apply_assertions(body, exp):
    """Fills `exp` from the assertions on stdout / stderr / exit_code.

    Returns None, or the reason why an assertion could not be translated (so a
    test is never silently imported with weaker expectations).
    """
    for kind, args in iter_assertions(body):
        mentioned = " ".join(args[:2])
        if not re.search(r"\b(stdout|stderr|exit_code)\b", mentioned):
            continue
        if kind == "_eq" and args[0] == "exit_code":
            exp["exit"] = int(args[1])
        elif kind == "_ne" and args[0] == "exit_code" and args[1] == "0":
            exp["exit_nonzero"] = True
        elif kind == "_eq" and args[0] == "stdout":
            value = string_literal(args[1])
            if value is None:
                return "stdout is not compared with a plain string literal"
            exp["stdout"] = value
        elif kind == "" and args[0] == "stdout.is_empty()":
            exp["stdout"] = ""
        elif kind == "" and args[0] == "stderr.is_empty()":
            exp["stderr_empty"] = True
        elif kind == "":
            atoms, ci_flags = [], []
            for atom in split_top_level(args[0], "||"):
                m = re.fullmatch(r"stderr\.(to_lowercase\(\)\.)?contains\((.*)\)", atom, re.S)
                text = string_literal(m.group(2)) if m else None
                if text is None:
                    return f"unsupported assertion: {args[0][:60]}"
                atoms.append(text)
                ci_flags.append(m.group(1) is not None)
            if len(set(ci_flags)) != 1:
                return "assertion mixes case-sensitive and case-insensitive stderr checks"
            if len(atoms) == 1:
                key = "stderr_contains_i" if ci_flags[0] else "stderr_contains"
                exp[key].append(atoms[0])
            else:
                exp["stderr_any"].append({"texts": atoms, "ci": ci_flags[0]})
        else:
            return f"unsupported assertion: {args[0][:60]}"
    return None


def extract_rust_test(body):
    """Returns (source, expectation) or (None, reason)."""
    prog = re.search(r'r(#+)"(.*?)"\1', body, re.S)
    if not prog:
        return None, "no raw program literal"
    source = prog.group(2)
    flags = ["--unsafe"] if "skip_typecheck" in body else []
    if (
        "interpreter" in body
        and "capture_compiled" not in body
        and "assert_same_output" not in body
    ):
        return None, "interpreter-only test"

    m = re.search(r"assert_same_output\(\s*\w+\s*,\s*", body)
    if m:
        lit = parse_rust_string(body, m.end())
        if lit is None:
            return None, "expected output is not a plain string literal"
        return source, {"flags": flags, "exit": 0, "valgrind": True, "stdout": lit[0]}

    m = re.search(
        r"capture_compiled_output_with_policy(_no_valgrind)?\(\s*&\w+\s*,\s*OverflowPolicy::(\w+)",
        body,
    )
    if m:
        exp = {
            "flags": flags + ["--overflow", POLICY[m.group(2)]],
            "valgrind": m.group(1) is None,
            "exit": 0,
            "exit_nonzero": False,
            "stdout": None,
            "stderr_empty": False,
            "stderr_contains": [],
            "stderr_contains_i": [],
            "stderr_any": [],
        }
        reason = apply_assertions(body, exp)
        if reason:
            return None, reason
        if exp["exit_nonzero"]:
            exp["valgrind"] = False
        return source, exp
    return None, "unrecognised test shape"


def cmd_import(opts):
    imported, skipped = 0, {}
    for root, _, files in sorted(os.walk(opts.rust_tests)):
        for f in sorted(files):
            if not f.endswith(".rs") or f == "mod.rs":
                continue
            group = os.path.relpath(os.path.join(root, f), opts.rust_tests)[
                :-3
            ].replace(os.sep, "/")
            src = open(os.path.join(root, f), encoding="utf-8").read()
            for part in re.split(r"(?m)^#\[test\]\s*$", src)[1:]:
                m = re.match(r"\s*fn\s+(\w+)\s*\(\)\s*\{", part)
                if not m:
                    continue
                source, exp = extract_rust_test(part)
                if source is None:
                    skipped.setdefault(exp, []).append(f"{group}::{m.group(1)}")
                    continue
                d = os.path.join(opts.out, group)
                os.makedirs(d, exist_ok=True)
                with open(
                    os.path.join(d, m.group(1) + ".rp"),
                    "w",
                    encoding="utf-8",
                    newline="",
                ) as fh:
                    fh.write(source)
                write_expect(os.path.join(d, m.group(1) + ".expect"), exp)
                imported += 1
    print(f"imported {imported} cases into {opts.out}")
    for why, names in sorted(skipped.items(), key=lambda kv: -len(kv[1])):
        print(f"skipped {len(names):>3}  {why}")
        for n in names[:3]:
            print(f"              e.g. {n}")
    return 0


# ----------------------------------------------------------------------------


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = ap.add_subparsers(dest="command", required=True)

    run = sub.add_parser("run", help="run the cases against one compiler")
    run.add_argument("--compiler", required=True)
    run.add_argument("--cases", default=os.path.join(here, "cases"))
    run.add_argument(
        "--memory",
        choices=["arc", "leak"],
        help="memory management (self-hosted compiler only)",
    )
    run.add_argument("--no-valgrind", dest="valgrind", action="store_false")
    run.add_argument("-k", default="*", help='glob on <group>/<name>, e.g. "memory/*"')
    run.add_argument(
        "--xfail", help="file with known failures, one <group>/<name> per line"
    )
    run.add_argument("-j", type=int, default=os.cpu_count() or 1)
    run.add_argument("--timeout", type=int, default=120)
    run.add_argument("--progress", action="store_true")
    run.add_argument("-v", action="store_true", help="also list the passing cases")
    run.set_defaults(func=cmd_run)

    diff = sub.add_parser("diff", help="run programs with two compilers and compare")
    diff.add_argument(
        "--compiler", required=True, help="compiler A (e.g. the Rust one)"
    )
    diff.add_argument(
        "--compiler-b", required=True, help="compiler B (e.g. the self-hosted one)"
    )
    diff.add_argument(
        "--memory",
        choices=["arc", "leak"],
        help="memory mode, applied to compilers that support it",
    )
    diff.add_argument(
        "--flags", help='extra flags for both compilers, e.g. "--overflow warn"'
    )
    diff.add_argument("--no-valgrind", dest="valgrind", action="store_false")
    diff.add_argument("-j", type=int, default=os.cpu_count() or 1)
    diff.add_argument("--timeout", type=int, default=120)
    diff.add_argument("files", nargs="+", help=".rp files or directories")
    diff.set_defaults(func=cmd_diff)

    imp = sub.add_parser(
        "import", help="convert the Rust integration tests into cases (one-off)"
    )
    imp.add_argument("--rust-tests", default="tests")
    imp.add_argument("--out", default=os.path.join(here, "cases"))
    imp.set_defaults(func=cmd_import)

    opts = ap.parse_args()
    if hasattr(opts, "compiler"):
        opts.compiler = os.path.abspath(opts.compiler)
    if hasattr(opts, "compiler_b"):
        opts.compiler_b = os.path.abspath(opts.compiler_b)
    sys.exit(opts.func(opts))


if __name__ == "__main__":
    main()
