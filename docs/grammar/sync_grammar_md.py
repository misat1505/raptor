#!/usr/bin/env python3
"""Keep the production rules in docs/grammar.md in sync with docs/grammar.ebnf.

grammar.md is hand-written prose plus blocks of the form

    <!-- ebnf: rule_a rule_b -->
    ...anything...
    <!-- /ebnf -->

This script replaces the body of every such block with the named rules copied
verbatim from grammar.ebnf (including the comment written directly above a
rule). Usage:

    python3 docs/sync_grammar_md.py           rewrite grammar.md in place
    python3 docs/sync_grammar_md.py --check   exit 1 if grammar.md is stale

It also fails when a block names an unknown rule, or when a rule of
grammar.ebnf is not shown in any block.
"""
import re
import sys
from pathlib import Path

DOCS = Path(__file__).resolve().parent
EBNF = DOCS / "grammar.ebnf"
MD = DOCS / "grammar.md"

BLOCK = re.compile(r"<!-- ebnf: ([a-z_ ]+?) -->\n.*?<!-- /ebnf -->", re.S)


def scan(src):
    """Return [(kind, name_or_None, start, end)] for comments and rules."""
    items, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if c.isspace():
            i += 1
        elif src.startswith("(*", i):
            j = src.index("*)", i + 2) + 2
            items.append(("comment", None, i, j))
            i = j
        elif c.isalpha():
            m = re.match(r"[a-z_]+", src[i:])
            name, start, j = m.group(0), i, i + m.end()
            while src[j] != ";":  # find the terminating ';' outside quotes/comments
                if src[j] in "\"'":
                    j = src.index(src[j], j + 1)
                elif src.startswith("(*", j):
                    j = src.index("*)", j + 2) + 1
                elif src[j] == "?":
                    j = src.index("?", j + 1)
                j += 1
            items.append(("rule", name, start, j + 1))
            i = j + 1
        else:
            raise SystemExit(f"grammar.ebnf: unexpected {c!r} at offset {i}")
    return items


def load_rules():
    src = EBNF.read_text()
    items = scan(src)
    rules = {}
    for k, (kind, name, start, end) in enumerate(items):
        if kind != "rule":
            continue
        text = src[start:end]
        prev = items[k - 1] if k else None
        # Attach the comment directly above (not section banners / the header).
        if prev and prev[0] == "comment" and not src[prev[2]:prev[3]].startswith("(* ="):
            if src[prev[3]:start].strip() == "":
                text = src[prev[2]:prev[3]] + "\n" + text
        rules[name] = text
    return rules


def init_skeleton(rules):
    """A minimal grammar.md with every rule in a single block."""
    names = " ".join(rules)
    return (
        "# Grammar\n\n"
        "Source of truth: [`grammar.ebnf`](grammar.ebnf). Edit the rules there, "
        "then run `python3 sync_grammar_md.py`.\n\n"
        f"<!-- ebnf: {names} -->\n<!-- /ebnf -->\n"
    )


def main():
    args = sys.argv[1:]
    if "-h" in args or "--help" in args:
        print(__doc__)
        print("    python3 docs/sync_grammar_md.py --init    create grammar.md "
              "skeleton if it does not exist")
        return 0
    check = "--check" in args
    rules = load_rules()
    used = set()

    if "--init" in args:
        if MD.exists():
            raise SystemExit(f"{MD} already exists; refusing to overwrite it")
        MD.write_text(init_skeleton(rules))
        print(f"created {MD}; now run the script again to fill in the rules")
        return 0
    if not MD.exists():
        raise SystemExit(
            f"{MD} not found. It holds the hand-written text and examples and is "
            "NOT generated from scratch - restore it from version control, or "
            "run with --init to create a bare skeleton."
        )

    def render(m):
        names = m.group(1).split()
        missing = [x for x in names if x not in rules]
        if missing:
            raise SystemExit(f"grammar.md references unknown rule(s): {', '.join(missing)}")
        used.update(names)
        body = "\n\n".join(rules[x] for x in names)
        return f"<!-- ebnf: {' '.join(names)} -->\n```ebnf\n{body}\n```\n<!-- /ebnf -->"

    old = MD.read_text()
    new = BLOCK.sub(render, old)

    unused = sorted(set(rules) - used)
    if unused:
        hint = ""
        if not used:
            hint = ("\n(grammar.md contains no '<!-- ebnf: ... -->' blocks - "
                    "is it an empty or wrong file?)")
        raise SystemExit(f"rules not shown in grammar.md: {', '.join(unused)}{hint}")

    if check:
        if new != old:
            print("docs/grammar.md is out of date; run: python3 docs/sync_grammar_md.py")
            return 1
        return 0
    MD.write_text(new)
    print(f"updated {MD.name}: {len(used)} rules in {len(BLOCK.findall(new))} blocks")
    return 0


if __name__ == "__main__":
    sys.exit(main())
