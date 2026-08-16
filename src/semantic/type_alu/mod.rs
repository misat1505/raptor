
pub mod arithmetic;
pub mod cast;
pub mod comparison;
pub mod logical;
pub mod unary;

#[cfg(test)]
mod tests;

pub(in crate::semantic) struct TypeALU;
