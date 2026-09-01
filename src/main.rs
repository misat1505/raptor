use crate::{
    backend::{interpreter::interpreter::Interpreter, llvm::compiler::Compiler},
    common::errors::{ErrorSeverity, IError},
    frontend::{
        lexer::{
            lazy_stream_reader::LazyStreamReader,
            lexer::{Lexer, LexerOptions},
        },
        parser::{IParser, Parser},
    },
    semantic::semantic_checker::SemanticChecker,
};
use inkwell::{context::Context, OptimizationLevel};
use raptor_lib::common;
use raptor_lib::frontend;
use raptor_lib::semantic;
use raptor_lib::{
    backend::{self, llvm::OverflowPolicy},
    import_resolver::ImportResolver,
};
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{exit, Command},
    time::{Duration, Instant},
};

const LLVM_VERSION: &str = "18";

const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const DIM: &str = "\x1b[2m";

#[derive(Debug, Clone)]
struct CliOptions {
    path: String,
    verbose: bool,
    is_unsafe: bool,
    is_compile: bool,
    should_run: bool,
    output_path: Option<String>,
    link_objects: Vec<String>,
    opt_level: Option<OptimizationLevel>,
    overflow_policy: OverflowPolicy,
}

impl CliOptions {
    fn parse() -> Self {
        let args: Vec<String> = args().collect();
        let mut opts = Self {
            path: String::new(),
            verbose: false,
            is_unsafe: false,
            is_compile: false,
            should_run: false,
            output_path: None,
            link_objects: Vec::new(),
            opt_level: None,
            overflow_policy: OverflowPolicy::Ignore,
        };

        let mut iter = args.iter().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    usage();
                    exit(0);
                }
                "-v" | "--verbose" => opts.verbose = true,
                "--unsafe" => opts.is_unsafe = true,
                "--compile" => opts.is_compile = true,
                "--run" => {
                    opts.is_compile = true;
                    opts.should_run = true;
                }
                "-o" => {
                    let value = next_value(&mut iter, "-o");
                    if value.starts_with('-') {
                        fatal(&format!("-o requires a file, got '{value}'."));
                    }
                    opts.output_path = Some(value);
                }
                "--link" => {
                    let value = next_value(&mut iter, "--link");
                    opts.link_objects.push(value);
                }
                "-O0" => opts.opt_level = Some(OptimizationLevel::None),
                "-O1" => opts.opt_level = Some(OptimizationLevel::Less),
                "-O2" => opts.opt_level = Some(OptimizationLevel::Default),
                "-O3" => opts.opt_level = Some(OptimizationLevel::Aggressive),
                "--overflow" => {
                    let value = next_value(&mut iter, "--overflow");
                    opts.overflow_policy = match value.as_str() {
                        "ignore" => OverflowPolicy::Ignore,
                        "warn" => OverflowPolicy::Warn,
                        "error" => OverflowPolicy::Error,
                        _ => fatal(&format!("invalid overflow policy '{value}'. Expected: ignore, warn, error.")),
                    };
                }
                flag if flag.starts_with('-') => {
                    eprintln!("Error: unknown option '{flag}'.\n");
                    usage();
                    exit(1);
                }
                path => {
                    if !opts.path.is_empty() {
                        eprintln!("Error: multiple input files given.\n");
                        usage();
                        exit(1);
                    }
                    opts.path = path.to_string();
                }
            }
        }

        if opts.path.is_empty() {
            eprintln!("Error: path to file not given.\n");
            usage();
            exit(1);
        }

        if opts.output_path.is_some() && !opts.is_compile {
            fatal("-o can only be used with --compile or --run.");
        }

        opts
    }
}

fn next_value<'a>(iter: &mut impl Iterator<Item = &'a String>, flag: &str) -> String {
    match iter.next() {
        Some(value) => value.clone(),
        None => fatal(&format!("{flag} requires a value.")),
    }
}

fn fatal(msg: &str) -> ! {
    eprintln!("Error: {msg}");
    exit(1);
}

fn print_duration(phase: &str, duration: Duration) {
    let total_ms = duration.as_secs_f64() * 1000.0;

    let (value, unit) = if total_ms < 1_000.0 {
        (total_ms, "ms")
    } else if total_ms < 60_000.0 {
        (total_ms / 1_000.0, "s")
    } else if total_ms < 3_600_000.0 {
        (total_ms / 60_000.0, "min")
    } else {
        (total_ms / 3_600_000.0, "h")
    };

    println!("{CYAN}[time]{RESET}  {phase:<22} {DIM}│{RESET} {value:>9.3} {DIM}{unit}{RESET}",);
}

fn print_debug(message: &str) {
    println!("{YELLOW}[debug]{RESET} {message}");
}

fn on_warning(warning: Box<dyn IError>) {
    eprintln!("{}", warning.get_stderr_message());
}

fn usage() {
    println!(
        "\
Usage:
    program [OPTIONS] <FILE>

Options:
    -h, --help          Show this help message
    -v, --verbose       Show execution time of each phase
    --unsafe            Skip semantic checking
    --compile           Compile the source file instead of interpreting it
    --run               After compiling, build and run the resulting executable
                        (implies --compile)
    -o <FILE>           Set output executable path
    --link <FILE>       Link an additional object file (compilation mode only)
    -O0                 No optimization (default)
    -O1                 Basic optimization
    -O2                 Default optimization
    -O3                 Aggressive optimization
    --overflow <POLICY> Integer overflow policy: ignore, warn, error

Arguments:
    <FILE>              Path to the source file
"
    );
}

struct TimedPhase {
    phase: &'static str,
    start: Instant,
    enabled: bool,
}

impl TimedPhase {
    fn new(phase: &'static str, enabled: bool) -> Self {
        Self {
            phase,
            start: Instant::now(),
            enabled,
        }
    }
}

impl Drop for TimedPhase {
    fn drop(&mut self) {
        if self.enabled {
            print_duration(self.phase, self.start.elapsed());
        }
    }
}

struct BuildArtifacts {
    ir_path: String,
    obj_path: String,
    exe_path: String,
}

fn output_paths(input_path: &str, output_path: Option<&str>) -> BuildArtifacts {
    let input = Path::new(input_path);
    let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("output");

    let output_dir = Path::new("build");
    std::fs::create_dir_all(output_dir).expect("failed to create build directory");

    let ir_path = output_dir.join(format!("{stem}.ll"));
    let obj_path = output_dir.join(format!("{stem}.o"));

    let exe_path = match output_path {
        Some(path) => PathBuf::from(path),
        None => {
            #[cfg(windows)]
            {
                output_dir.join(format!("{stem}.exe"))
            }
            #[cfg(not(windows))]
            {
                output_dir.join(stem)
            }
        }
    };

    if let Some(parent) = exe_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("failed to create output directory");
        }
    }

    BuildArtifacts {
        ir_path: ir_path.to_string_lossy().into_owned(),
        obj_path: obj_path.to_string_lossy().into_owned(),
        exe_path: exe_path.to_string_lossy().into_owned(),
    }
}

fn run_command(program: &str, args: &[&str], step: &str) -> Result<(), String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|err| format!("Failed to invoke '{program}': {err}. Is it installed and on PATH?"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{step} failed:\n{stderr}"));
    }
    Ok(())
}

fn build_object_file(ir_path: &str, obj_path: &str) -> Result<(), String> {
    let llc = format!("llc-{LLVM_VERSION}");
    run_command(&llc, &[ir_path, "-filetype=obj", "-o", obj_path], "llc")
}

fn link_executable(obj_path: &str, exe_path: &str, ffi_objects: &[String]) -> Result<(), String> {
    let clang = format!("clang-{LLVM_VERSION}");
    let mut args = vec![obj_path.to_string(), "-o".to_string(), exe_path.to_string(), "-no-pie".to_string()];
    args.extend(ffi_objects.iter().cloned());

    let args_ref: Vec<&str> = args.iter().map(String::as_str).collect();
    run_command(&clang, &args_ref, "clang")
}

fn build_executable(ir_path: &str, obj_path: &str, exe_path: &str, ffi_objects: &[String]) -> Result<(), String> {
    build_object_file(ir_path, obj_path)?;
    link_executable(obj_path, exe_path, ffi_objects)?;
    Ok(())
}

struct Pipeline {
    opts: CliOptions,
    total_timer: Instant,
}

impl Pipeline {
    fn new(opts: CliOptions) -> Self {
        Self {
            opts,
            total_timer: Instant::now(),
        }
    }

    fn verbose(&self) -> bool {
        self.opts.verbose
    }

    fn debug(&self, message: &str) {
        if self.verbose() {
            print_debug(message);
        }
    }

    fn duration(&self, message: &str, duration: Duration) {
        if self.verbose() {
            print_duration(message, duration);
        }
    }

    fn timed(&self, phase: &'static str) -> TimedPhase {
        TimedPhase::new(phase, self.verbose())
    }

    fn run(self) {
        let lexer_options = LexerOptions {
            max_comment_length: 500,
            max_identifier_length: 100,
        };

        let (code, filename) = {
            let _t = self.timed("Source loading");
            self.load_source()
        };
        let reader = LazyStreamReader::new(code, Some(filename));

        let program = {
            let _t = self.timed("Lexer + Parser");
            self.run_frontend(reader, lexer_options.clone())
        };

        let program = {
            let _t = self.timed("Import Resolver");
            self.resolve_imports(filename, program, lexer_options)
        };

        self.run_semantic(&program);

        if self.opts.is_compile {
            self.compile_and_maybe_run(&program);
        } else {
            self.interpret(&program);
        }

        self.duration("total", self.total_timer.elapsed());
    }

    fn load_source(&self) -> (BufReader<File>, &'static str) {
        let path = &self.opts.path;
        let file = File::open(path).unwrap_or_else(|_| {
            eprintln!("File '{path}' not found.");
            exit(1);
        });
        let filename: &'static str = Box::leak(path.clone().into_boxed_str());
        (BufReader::new(file), filename)
    }

    fn run_frontend(&self, reader: LazyStreamReader<impl BufRead + 'static>, lexer_options: LexerOptions) -> frontend::ast::Program {
        let lexer = match Lexer::new(reader, lexer_options, on_warning) {
            Ok(lexer) => lexer,
            Err(err) => {
                eprintln!("{}", err.get_stderr_message());
                exit(1);
            }
        };
        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Ok(p) => p,
            Err(err) => {
                eprintln!("{}", err.get_stderr_message());
                exit(1);
            }
        }
    }

    fn resolve_imports(&self, filename: &'static str, program: frontend::ast::Program, lexer_options: LexerOptions) -> frontend::ast::Program {
        let mut import_resolver = ImportResolver::new(lexer_options, on_warning);
        match import_resolver.resolve(filename, program) {
            Ok(p) => p,
            Err(err) => {
                eprintln!("{}", err.get_stderr_message());
                exit(1);
            }
        }
    }

    fn run_semantic(&self, program: &frontend::ast::Program) {
        if self.opts.is_unsafe {
            if self.verbose() {
                println!("{CYAN}[time]{RESET}  {:<22} {DIM}│{RESET} skipped (--unsafe)", "semantic checker",);
            }
            return;
        }

        let _t = self.timed("Semantic checker");

        let mut checker = match SemanticChecker::new(program) {
            Ok(c) => c,
            Err(err) => {
                eprintln!("{}", err.get_stderr_message());
                exit(1);
            }
        };
        checker.check();

        if checker.errors.is_empty() {
            return;
        }

        let mut warnings = 0;
        let mut errors = 0;
        for error in &checker.errors {
            match error.get_severity() {
                ErrorSeverity::HIGH => errors += 1,
                ErrorSeverity::LOW => warnings += 1,
            }
            eprintln!("{}\n", error.get_stderr_message());
        }

        eprintln!("Static analysis finished with {errors} errors, {warnings} warnings.");
        if errors > 0 {
            exit(1);
        }
    }

    fn interpret(&self, program: &frontend::ast::Program) {
        let _t = self.timed("Interpreter");
        let mut interpreter = Interpreter::new(program);
        self.debug("Running interpreter...");
        if let Err(err) = interpreter.interpret() {
            eprintln!("{}", err.get_stderr_message());
            exit(1);
        }
        self.debug("Finished interpretation.");
    }

    fn compile_and_maybe_run(&self, program: &frontend::ast::Program) {
        let artifacts = output_paths(&self.opts.path, self.opts.output_path.as_deref());
        let context = Context::create();

        let compiler = {
            let _t = self.timed("Compiler");
            let mut c = Compiler::new(program, &context, self.opts.overflow_policy);
            if let Err(err) = c.compile() {
                eprintln!("{}", err.get_stderr_message());
                exit(1);
            }
            c
        };

        if let Some(level) = self.opts.opt_level {
            let _t = self.timed("Optimization");
            if let Err(err) = compiler.optimize(level) {
                eprintln!("{}", err.get_stderr_message());
                exit(1);
            }
        }

        {
            let _t = self.timed("Write LLVM IR");
            if let Err(err) = compiler.write_ir_to_file(&artifacts.ir_path) {
                eprintln!("{}", err.get_stderr_message());
                exit(1);
            }
        }
        self.debug(&format!("Wrote LLVM IR to '{}'.", artifacts.ir_path));

        {
            let _t = self.timed("LLVM -> exe");
            if let Err(err) = build_executable(&artifacts.ir_path, &artifacts.obj_path, &artifacts.exe_path, &self.opts.link_objects) {
                eprintln!("{err}");
                exit(1);
            }
        }
        self.debug(&format!("Built executable '{}'.", artifacts.exe_path));
        self.debug("Build successful.");

        if self.opts.should_run {
            let code = {
                let _t = self.timed("Program execution");
                match self.run_executable(&artifacts.exe_path) {
                    Ok(code) => code,
                    Err(err) => {
                        eprintln!("{err}");
                        exit(1);
                    }
                }
            };
            self.duration("Total", self.total_timer.elapsed());

            exit(code);
        }
    }

    fn run_executable(&self, exe_path: &str) -> Result<i32, String> {
        let path = if exe_path.starts_with('.') || exe_path.contains('/') || exe_path.contains('\\') {
            exe_path.to_string()
        } else {
            format!("./{exe_path}")
        };

        self.debug("Running executable...");

        let status = Command::new(&path).status().map_err(|err| format!("Failed to run '{path}': {err}"))?;

        self.debug("Finished running executable.");

        Ok(status.code().unwrap_or(1))
    }
}

fn main() {
    let opts = CliOptions::parse();
    Pipeline::new(opts).run();
}
