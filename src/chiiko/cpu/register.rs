#[derive(Debug, Clone, Copy)]
pub enum Register {
    Accumulator,
    BRegister,
    CRegister,
    HRegister,
    LRegister,
    IRegister,
    JRegister,
    Reserved,
    ProgramCounter,
    StackPointer,
    StatusFlags,
}