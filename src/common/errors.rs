use crate::{frontend::ast::Type, frontend::lexer::lazy_stream_reader::Position};
use std::fmt::Debug;

pub trait IError: Debug {
    fn message(&self) -> String;
    #[allow(dead_code)]
    fn set_message(&mut self, text: String);
    fn get_severity(&self) -> ErrorSeverity;
    fn expected_found(level: ErrorSeverity, summary: String, expected: String, found: String, position: Position) -> Self
    where
        Self: Sized;
    fn at(level: ErrorSeverity, summary: String, position: Position) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub enum ErrorSeverity {
    HIGH, // can't continue execution
    LOW,  // can continue execution
}

macro_rules! define_error {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            _message: String,
            _level: ErrorSeverity,
        }

        impl $name {
            pub fn new(level: ErrorSeverity, message: String) -> Self {
                $name {
                    _message: message,
                    _level: level,
                }
            }
        }

        impl IError for $name {
            fn message(&self) -> String {
                self._message.clone()
            }

            fn set_message(&mut self, text: String) {
                self._message = text;
            }

            fn get_severity(&self) -> ErrorSeverity {
                self._level.clone()
            }

            fn expected_found(level: ErrorSeverity, summary: String, expected: String, found: String, position: Position) -> Self {
                let message = format!(
                    "{}: {}\n  --> {}\n  expected: {}\n  found:    {}",
                    severity_to_string(&level),
                    summary,
                    position.location(),
                    expected,
                    found
                );
                $name::new(level, message)
            }

            fn at(level: ErrorSeverity, summary: String, position: Position) -> Self {
                let message = format!("{}: {}\n  --> {}", severity_to_string(&level), summary, position.location());

                $name::new(level, message)
            }
        }
    };
}

define_error!(LexerError);
define_error!(ParserError);
define_error!(SemanticCheckerError);
define_error!(InterpreterError);
define_error!(ComputationError);
define_error!(ScopeManagerError);
define_error!(StackOverflowError);
define_error!(StdFunctionError);
define_error!(CompilerError);

#[allow(dead_code)]
pub struct ErrorsManager;

impl ErrorsManager {
    #[allow(dead_code)]
    pub fn append_position(mut error: Box<dyn IError>, position: Position) -> Box<dyn IError> {
        error.set_message(format!("{}\nAt {:?}.", error.message(), position));
        error
    }
}

const RED: &str = "\x1b[1;31m";
const YELLOW: &str = "\x1b[1;33m";
const RESET: &str = "\x1b[0m";

pub fn severity_to_string(severity: &ErrorSeverity) -> String {
    match severity {
        ErrorSeverity::HIGH => format!("{}error{}", RED, RESET),
        ErrorSeverity::LOW => format!("{}warning{}", YELLOW, RESET),
    }
}

impl SemanticCheckerError {
    pub fn type_mismatch(level: ErrorSeverity, summary: String, expected: &Type, found: &Type, position: Position) -> Self {
        Self::expected_found(level, summary, format!("{:?}", expected), format!("{:?}", found), position)
    }
}
