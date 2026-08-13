use std::{
    env::args,
    eprintln,
    fs::File,
    io::BufReader,
    path::Path,
    println,
    process::{exit, Command},
};

use errors::IError;
use inkwell::{context::Context, OptimizationLevel};
use lexer::Lexer;
mod lazy_stream_reader;
use lazy_stream_reader::LazyStreamReader;

use crate::{
    compiler::Compiler,
    interpreter::Interpreter,
    lexer::LexerOptions,
    parser::{IParser, Parser},
    semantic_checker::SemanticChecker,
};

mod alu;
mod ast;
mod compiler;
mod errors;
mod interpreter;
mod lexer;
mod libc_functions;
mod llvm_alu;
mod llvm_value;
mod parser;
mod scope_manager;
mod semantic_checker;
mod stack;
mod static_checker_stack;
mod std_functions;
mod tokens;
mod type_alu;
mod value;
mod visitor;

mod tests;

const LLVM_VERSION: &str = "18";

fn on_warning(warning: Box<dyn IError>) {
    eprintln!("{}", warning.message());
}

fn usage() {
    println!(
        "\
Usage:
    program [OPTIONS] <FILE>

Options:
    -h, --help      Show this help message
    --unsafe        Skip semantic checking
    --compile       Compile the source file instead of interpreting it
    --run           After compiling, build and run the resulting executable
                    (implies --compile)
    -O0             No optimization (default)
    -O1             Basic optimization
    -O2             Default optimization
    -O3             Aggressive optimization

Arguments:
    <FILE>          Path to the source file
"
    );
}

fn output_paths(input_path: &str) -> (String, String, String) {
    let path = Path::new(input_path);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");

    let output_dir = "build/";
    let ir_path = format!("{}{}.ll", output_dir, stem);
    let obj_path = format!("{}{}.o", output_dir, stem);

    #[cfg(windows)]
    let exe_path = format!("{}{}.exe", output_dir, stem);
    #[cfg(not(windows))]
    let exe_path = format!("{}{}", output_dir, stem);

    (ir_path, obj_path, exe_path)
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

fn link_executable(obj_path: &str, exe_path: &str) -> Result<(), String> {
    let clang = format!("clang-{}", LLVM_VERSION);
    run_command(&clang, &[obj_path, "-o", exe_path, "-no-pie"], "clang")
}

fn build_executable(ir_path: &str, obj_path: &str, exe_path: &str) -> Result<(), String> {
    build_object_file(ir_path, obj_path)?;
    link_executable(obj_path, exe_path)?;
    Ok(())
}

fn run_executable(exe_path: &str) -> Result<i32, String> {
    let path = if exe_path.starts_with('.') || exe_path.contains('/') || exe_path.contains('\\') {
        exe_path.to_string()
    } else {
        let command = format!("./{}", exe_path);
        command
    };

    let status = Command::new(&path).status().map_err(|err| format!("Failed to run '{}': {}", path, err))?;

    Ok(status.code().unwrap_or(1))
}

fn main() {
    let mut is_unsafe = false;
    let mut is_compile = false;
    let mut should_run = false;
    let mut opt_level: Option<OptimizationLevel> = None;

    let args: Vec<String> = args().collect();

    let mut path: Option<String> = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                usage();
                return;
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

    let lexer_options = LexerOptions {
        max_comment_length: 100,
        max_identifier_length: 20,
    };

    let lexer = match Lexer::new(reader, lexer_options, on_warning) {
        Ok(lexer) => lexer,
        Err(err) => {
            eprintln!("{}", err.message());
            exit(1);
        }
    };

    let mut parser = Parser::new(lexer);

    let program = match parser.parse() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{}", err.message());
            exit(1);
        }
    };

    if !is_unsafe {
        let mut semantic_checker = match SemanticChecker::new(&program) {
            Ok(checker) => checker,
            Err(err) => {
                eprintln!("{}", err.message());
                exit(1);
            }
        };

        semantic_checker.check();

        if semantic_checker.errors.len() > 0 {
            let mut warnings = 0;
            let mut errors = 0;

            for error in &semantic_checker.errors {
                match error.get_severity() {
                    errors::ErrorSeverity::HIGH => errors += 1,
                    errors::ErrorSeverity::LOW => warnings += 1,
                }

                eprintln!("{}\n", error.message());
            }

            eprintln!("Static analysis finished with {} errors, {} warnings.", errors, warnings);
            exit(1);
        }
    }

    if is_compile {
        let (ir_path, obj_path, exe_path) = output_paths(&path);

        let context = Context::create();
        let mut compiler = Compiler::new(&program, &context);
        if let Err(err) = compiler.compile() {
            eprintln!("{}", err.message());
            exit(1);
        }
        if let Some(level) = opt_level {
            if let Err(err) = compiler.optimize(level) {
                eprintln!("{}", err.message());
                exit(1);
            }
        }
        if let Err(err) = compiler.write_ir_to_file(&ir_path) {
            eprintln!("{}", err.message());
            exit(1);
        }

        println!("Wrote LLVM IR to '{}'.", ir_path);

        if let Err(err) = build_executable(&ir_path, &obj_path, &exe_path) {
            eprintln!("{}", err);
            exit(1);
        }

        println!("Built executable '{}'.", exe_path);

        if should_run {
            match run_executable(&exe_path) {
                Ok(code) => std::process::exit(code),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            }
        }
    } else {
        let mut interpreter = Interpreter::new(&program);

        if let Err(err) = interpreter.interpret() {
            eprintln!("{}", err.message());
            exit(1);
        }
    }
}
