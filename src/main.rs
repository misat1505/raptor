use std::{env::args, eprintln, fs::File, io::BufReader, println, time::Instant};

use errors::IError;
use lexer::Lexer;
mod lazy_stream_reader;
use lazy_stream_reader::LazyStreamReader;

use crate::{
    interpreter::Interpreter,
    lexer::LexerOptions,
    parser::{IParser, Parser},
    semantic_checker::SemanticChecker,
};

mod alu;
mod ast;
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

Arguments:
    <FILE>          Path to the source file
"
    );
}

fn main() {
    let mut is_unsafe = false;
    let args: Vec<String> = args().collect();

    let path = match args.get(1).cloned() {
        Some(arg) if arg == "-h" || arg == "--help" => {
            usage();
            return;
        }
        Some(arg) if arg == "--unsafe" => {
            is_unsafe = true;

            match args.get(2).cloned() {
                Some(path) => path,
                None => {
                    eprintln!("Error: path to file not given.\n");
                    usage();
                    return;
                }
            }
        }
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

    // TODO: change unwrap to log the error
    let lexer = Lexer::new(reader, lexer_options, on_warning).unwrap();
    let mut parser = Parser::new(lexer);

    let start = Instant::now();
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

    let mut interpreter = Interpreter::new(&program);
    if let Err(err) = interpreter.interpret() {
        eprintln!("{}", err.message());
    };

    println!("\nExecution time: {:?}", Instant::now() - start);
}
