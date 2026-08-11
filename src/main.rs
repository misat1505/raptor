use std::{env::args, eprintln, fs::File, io::BufReader, println};

use errors::IError;
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

Arguments:
    <FILE>          Path to the source file
"
    );
}

fn main() {
    let mut is_unsafe = false;
    let mut is_compile = false;

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
            return;
        }
    };

    let file = match File::open(path.as_str()) {
        Ok(f) => f,
        Err(_) => return eprintln!("File '{}' not found.", path),
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
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(lexer);

    let program = match parser.parse() {
        Ok(p) => p,
        Err(err) => return eprintln!("{}", err.message()),
    };

    if !is_unsafe {
        let mut semantic_checker = match SemanticChecker::new(&program) {
            Ok(checker) => checker,
            Err(err) => return eprintln!("{}", err.message()),
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

            return;
        }
    }

    if is_compile {
        let mut compiler = Compiler::new(&program);

        if let Err(err) = compiler.compile() {
            eprintln!("{}", err.message());
        }
    } else {
        let mut interpreter = Interpreter::new(&program);

        if let Err(err) = interpreter.interpret() {
            eprintln!("{}", err.message());
        }
    }
}
