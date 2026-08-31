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
    eprintln,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    println,
    process::{exit, Command},
    time::{Duration, Instant},
};

const LLVM_VERSION: &str = "18";

const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const DIM: &str = "\x1b[2m";

fn print_duration(phase: &str, duration: Duration) {
    println!(
        "{cyan}[time]{reset}  {phase:<22} {dim}│{reset} {time:>9.3} {dim}ms{reset}",
        cyan = CYAN,
        reset = RESET,
        phase = phase,
        dim = DIM,
        time = duration.as_secs_f64() * 1000.0,
    );
}

fn print_debug(message: &str) {
    println!("{yellow}[debug]{reset} {message}", yellow = YELLOW, reset = RESET, message = message,);
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
    -h, --help      Show this help message
    -v, --verbose   Show execution time of each phase
    --unsafe        Skip semantic checking
    --compile       Compile the source file instead of interpreting it
    --run           After compiling, build and run the resulting executable
                    (implies --compile)
    -o <FILE>       Set output executable path
    --link <FILE>   Link an additional object file (compilation mode only)
    -O0             No optimization (default)
    -O1             Basic optimization
    -O2             Default optimization
    -O3             Aggressive optimization
    --overflow <POLICY>
                    Integer overflow policy: ignore, warn, error
Arguments:
    <FILE>          Path to the source file
"
    );
}

fn output_paths(input_path: &str, output_path: Option<&str>) -> (String, String, String) {
    let input = Path::new(input_path);
    let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    let output_dir = Path::new("build");
    std::fs::create_dir_all(output_dir).expect("failed to create build directory");
    let ir_path = output_dir.join(format!("{}.ll", stem));
    let obj_path = output_dir.join(format!("{}.o", stem));
    let exe_path = match output_path {
        Some(path) => PathBuf::from(path),
        None => {
            #[cfg(windows)]
            {
                output_dir.join(format!("{}.exe", stem))
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
    (
        ir_path.to_string_lossy().into_owned(),
        obj_path.to_string_lossy().into_owned(),
        exe_path.to_string_lossy().into_owned(),
    )
}

fn run_command(program: &str, args: &[&str], step_description: &str) -> Result<(), String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|err| format!("Failed to invoke '{}': {}. Is it installed and on PATH?", program, err))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{} failed:\n{}", step_description, stderr));
    }
    Ok(())
}

fn build_object_file(ir_path: &str, obj_path: &str) -> Result<(), String> {
    let llc = format!("llc-{}", LLVM_VERSION);
    run_command(&llc, &[ir_path, "-filetype=obj", "-o", obj_path], "llc")
}

fn link_executable(obj_path: &str, exe_path: &str, ffi_objects: &[String]) -> Result<(), String> {
    let clang = format!("clang-{}", LLVM_VERSION);
    let mut args: Vec<String> = vec![obj_path.to_string(), "-o".to_string(), exe_path.to_string(), "-no-pie".to_string()];
    for ffi_object in ffi_objects {
        args.push(ffi_object.clone());
    }
    let args: Vec<&str> = args.iter().map(|arg| arg.as_str()).collect();
    run_command(&clang, &args, "clang")
}

fn build_executable(ir_path: &str, obj_path: &str, exe_path: &str, ffi_objects: &[String]) -> Result<(), String> {
    build_object_file(ir_path, obj_path)?;

    link_executable(obj_path, exe_path, ffi_objects)?;

    Ok(())
}

fn run_executable(exe_path: &str, verbose: bool) -> Result<i32, String> {
    let path = if exe_path.starts_with('.') || exe_path.contains('/') || exe_path.contains('\\') {
        exe_path.to_string()
    } else {
        format!("./{}", exe_path)
    };
    if verbose {
        print_debug("Running executable...");
    }
    let status = Command::new(&path).status().map_err(|err| format!("Failed to run '{}': {}", path, err))?;
    if verbose {
        print_debug("Finished running executable.");
    }
    Ok(status.code().unwrap_or(1))
}

fn main() {
    let total_timer = Instant::now();
    let mut is_unsafe = false;
    let mut is_compile = false;
    let mut should_run = false;
    let mut verbose = false;
    let mut overflow_policy = OverflowPolicy::Ignore;
    let mut opt_level: Option<OptimizationLevel> = None;
    let mut link_objects: Vec<String> = Vec::new();
    let args: Vec<String> = args().collect();
    let mut path: Option<String> = None;
    let mut output_path: Option<String> = None;
    let mut arg_iter = args.iter().skip(1);

    while let Some(arg) = arg_iter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                usage();
                return;
            }
            "-v" | "--verbose" => {
                verbose = true;
            }
            "--unsafe" => {
                is_unsafe = true;
            }
            "--compile" => {
                is_compile = true;
            }
            "--run" => {
                is_compile = true;
                should_run = true;
            }
            "-o" => {
                let output = match arg_iter.next() {
                    Some(output) => output,
                    None => {
                        eprintln!("Error: -o requires a file.");
                        exit(1);
                    }
                };
                if output.starts_with('-') {
                    eprintln!("Error: -o requires a file, got '{}'.", output);
                    exit(1);
                }
                output_path = Some(output.to_string());
            }
            "--link" => {
                let object = match arg_iter.next() {
                    Some(object) => object,
                    None => {
                        eprintln!("Error: --link requires a file.");
                        exit(1);
                    }
                };
                link_objects.push(object.to_string());
            }
            "-O0" => {
                opt_level = Some(OptimizationLevel::None);
            }
            "-O1" => {
                opt_level = Some(OptimizationLevel::Less);
            }
            "-O2" => {
                opt_level = Some(OptimizationLevel::Default);
            }
            "-O3" => {
                opt_level = Some(OptimizationLevel::Aggressive);
            }
            "--overflow" => {
                let policy = match arg_iter.next() {
                    Some(policy) => policy,
                    None => {
                        eprintln!(
                            "Error: --overflow requires one of: \
                             ignore, warn, error."
                        );
                        exit(1);
                    }
                };
                overflow_policy = match policy.as_str() {
                    "ignore" => OverflowPolicy::Ignore,
                    "warn" => OverflowPolicy::Warn,
                    "error" => OverflowPolicy::Error,
                    _ => {
                        eprintln!(
                            "Error: invalid overflow policy '{}'. \
                             Expected: ignore, warn, error.",
                            policy
                        );
                        exit(1);
                    }
                };
            }
            arg if arg.starts_with('-') => {
                eprintln!("Error: unknown option '{}'.\n", arg);
                usage();
                return;
            }
            _ => {
                if path.is_some() {
                    eprintln!("Error: multiple input files given.\n");
                    usage();
                    return;
                }
                path = Some(arg.to_string());
            }
        }
    }

    let path = match path {
        Some(path) => path,
        None => {
            eprintln!("Error: path to file not given.\n");
            usage();
            exit(1);
        }
    };

    if output_path.is_some() && !is_compile {
        eprintln!("Error: -o can only be used with --compile or --run.");
        exit(1);
    }

    let source_timer = Instant::now();
    let file = match File::open(path.as_str()) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("File '{}' not found.", path);
            exit(1);
        }
    };
    let code = BufReader::new(file);
    let filename: &'static str = Box::leak(path.clone().into_boxed_str());
    let reader = LazyStreamReader::new(code, Some(filename));
    if verbose {
        print_duration("source loading", source_timer.elapsed());
    }

    let frontend_timer = Instant::now();
    let lexer_options = LexerOptions {
        max_comment_length: 500,
        max_identifier_length: 100,
    };
    let lexer = match Lexer::new(reader, lexer_options.clone(), on_warning) {
        Ok(lexer) => lexer,
        Err(err) => {
            eprintln!("{}", err.get_stderr_message());
            exit(1);
        }
    };
    let mut parser = Parser::new(lexer);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{}", err.get_stderr_message());
            exit(1);
        }
    };
    if verbose {
        print_duration("lexer + parser", frontend_timer.elapsed());
    }

    let import_timer = Instant::now();
    let mut import_resolver = ImportResolver::new(lexer_options, on_warning);
    let import_resolved_program = match import_resolver.resolve(filename, program) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{}", err.get_stderr_message());
            exit(1);
        }
    };
    if verbose {
        print_duration("import resolver", import_timer.elapsed());
    }

    if !is_unsafe {
        let semantic_timer = Instant::now();
        let mut semantic_checker = match SemanticChecker::new(&import_resolved_program) {
            Ok(checker) => checker,
            Err(err) => {
                eprintln!("{}", err.get_stderr_message());
                exit(1);
            }
        };
        semantic_checker.check();
        if verbose {
            print_duration("semantic checker", semantic_timer.elapsed());
        }
        if !semantic_checker.errors.is_empty() {
            let mut warnings = 0;
            let mut errors = 0;
            for error in &semantic_checker.errors {
                match error.get_severity() {
                    ErrorSeverity::HIGH => errors += 1,
                    ErrorSeverity::LOW => warnings += 1,
                }
                eprintln!("{}\n", error.get_stderr_message());
            }
            eprintln!("Static analysis finished with {} errors, {} warnings.", errors, warnings);
            if errors > 0 {
                exit(1);
            }
        }
    } else if verbose {
        println!(
            "{cyan}[time]{reset}  {phase:<22} {dim}│{reset} skipped (--unsafe)",
            cyan = CYAN,
            reset = RESET,
            phase = "semantic checker",
            dim = DIM,
        );
    }

    if is_compile {
        let (ir_path, obj_path, exe_path) = output_paths(&path, output_path.as_deref());
        let context = Context::create();
        let compile_timer = Instant::now();
        let mut compiler = Compiler::new(&import_resolved_program, &context, overflow_policy);
        if let Err(err) = compiler.compile() {
            eprintln!("{}", err.get_stderr_message());
            exit(1);
        }
        if verbose {
            print_duration("compiler", compile_timer.elapsed());
        }

        if let Some(level) = opt_level {
            let optimization_timer = Instant::now();
            if let Err(err) = compiler.optimize(level) {
                eprintln!("{}", err.get_stderr_message());
                exit(1);
            }
            if verbose {
                print_duration("optimization", optimization_timer.elapsed());
            }
        }

        let ir_timer = Instant::now();
        if let Err(err) = compiler.write_ir_to_file(&ir_path) {
            eprintln!("{}", err.get_stderr_message());
            exit(1);
        }
        if verbose {
            print_duration("write LLVM IR", ir_timer.elapsed());
            print_debug(&format!("Wrote LLVM IR to '{}'.", ir_path));
        }

        let build_timer = Instant::now();
        if let Err(err) = build_executable(&ir_path, &obj_path, &exe_path, &link_objects) {
            eprintln!("{}", err);
            exit(1);
        }
        if verbose {
            print_duration("LLVM -> exe", build_timer.elapsed());
            print_debug(&format!("Built executable '{}'.", exe_path));
            print_debug(&format!("Build successful."));
        }

        if should_run {
            let run_timer = Instant::now();
            match run_executable(&exe_path, verbose) {
                Ok(code) => {
                    if verbose {
                        print_duration("program execution", run_timer.elapsed());
                        print_duration("total", total_timer.elapsed());
                    }
                    std::process::exit(code);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            }
        }
    } else {
        let interpreter_timer = Instant::now();
        let mut interpreter = Interpreter::new(&import_resolved_program);
        if let Err(err) = interpreter.interpret() {
            eprintln!("{}", err.get_stderr_message());
            exit(1);
        }
        if verbose {
            print_duration("interpreter", interpreter_timer.elapsed());
        }
    }

    if verbose {
        print_duration("total", total_timer.elapsed());
    }
}
