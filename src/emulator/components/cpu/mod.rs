mod core;
// mod error;
mod fetch;
mod flags;
mod memory;
mod register;
mod stack;

#[cfg(test)]
mod test;

pub use core::Cpu;
