#[derive(Debug, Clone, Copy)]
pub enum Register {
    Accumulator,
    BRegister,
    CRegister,
    HRegister,
    LRegister,
    IRegister,
    JRegister,
    ProgramCounterHigh,
    ProgramCounterLow,
    StackPointer,
    StatusFlags,
}