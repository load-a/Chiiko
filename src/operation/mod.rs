mod core;
mod error;
pub mod group;

#[cfg(test)]
mod test;

pub use core::Operation;
pub use error::OperationError;
