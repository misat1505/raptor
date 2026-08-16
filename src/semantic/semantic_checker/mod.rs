pub mod checker;
pub mod expressions;
pub mod functions;
pub mod statements;
pub mod visitor;

#[cfg(test)]
mod tests;

pub use checker::SemanticChecker;
