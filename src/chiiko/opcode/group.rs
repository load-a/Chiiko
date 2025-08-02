#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum ArithmeticVariant {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Increment,
    Decrement,
    Random,
}

#[derive(Debug, PartialEq)]
pub enum LogicVariant {
    LogicalAnd,
    LogicalNot,
    InclusiveOr,
    ExclusiveOr,
    LeftShift,
    RightShift,
    LeftRotate,
    RightRotate,
}

#[derive(Debug, PartialEq)]
pub enum BranchVariant {
    Compare,
    Positive,
    Negative,
    Zero,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum StackVariant {
    Push,
    Pop,
    Dump,
    Restore,
}

#[derive(Debug, PartialEq)]
pub enum MemoryVariant {
    Move,
    Load,
    Save,
    Swap,
}

#[derive(Debug, PartialEq)]
pub enum InputOutputVariant {
    StringInput,
    NumericInput,
    PrintString,
    PrintNumber,
    QueryKeyboard,
}

#[derive(Debug, PartialEq)]
pub enum SystemVariant {
    Halt,
    Wait, // No-op instruction
}
