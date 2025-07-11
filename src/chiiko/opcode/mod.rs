mod core;
pub mod group;

#[cfg(test)]
mod test;

pub use core::Opcode;
pub use group::{
    Group,
    ArithmeticVariant,
    LogicVariant,
    BranchVariant,
    SubroutineVariant,
    StackVariant,
    MemoryVariant,
    InputOutputVariant,
    SystemVariant,
};