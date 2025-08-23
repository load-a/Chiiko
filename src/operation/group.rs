#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Group {
    Arithmetic(ArithmeticVariant),
    Logic(LogicVariant),
    Branch(BranchVariant),
    Subroutine(SubroutineVariant),
    Stack(StackVariant),
    Memory(MemoryVariant),
    InputOutput(InputOutputVariant),
    System(SystemVariant),
}

// TODO: Give each of these their own file

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArithmeticVariant {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Increment,
    Decrement,
    Random,
    Sum,
    Difference,
    Product,
    Quotient,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LogicVariant {
    LogicalAnd,
    InclusiveOr,
    ExclusiveOr,
    LogicalNot,
    LeftShift,
    RightShift,
    LeftRotate,
    RightRotate,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BranchVariant {
    Compare,
    Positive,
    Negative,
    Zero,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SubroutineVariant {
    Call,
    Return,
    Jump,
    JumpGreater,
    JumpGreaterEqual,
    JumpEqual,
    JumpLessEqual,
    JumpLess,
    JumpNotEqual,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StackVariant {
    Push,
    Pop,
    Dump,
    Restore,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MemoryVariant {
    Move,
    Load,
    Save,
    Swap,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InputOutputVariant {
    StringInput,
    NumericInput,
    PrintString,
    PrintNumber,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SystemVariant {
    Halt,
    Wait, // No-op instruction
}
