pub enum InstrRegister {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    Lr,
    R15,
    Ipsr,
    // Psr,
    // Primask,
    // Basepri,
    // FaultMask,
    Control,
}

pub enum Value {
    Register(InstrRegister),
    Value(u32),
}

pub enum Instr {
    // Move
    Mov(InstrRegister, Value),
    // Write Special Register
    Msr(InstrRegister, Value),
    // Instruction Sync
    Isb,
    // Load a word
    Ldr(InstrRegister, Value),
    // Read Special Register - in this case Value is an address to be loaded into the
    // specified register
    Mrs(InstrRegister, Value),
    // And
    And(InstrRegister, Value, Value),
    // Sub
    Sub(InstrRegister, Value),
    // Logical Shift Right With a Flag Update
    Lsrs(InstrRegister, Value, Value),
    // Mov with an N and Z Flag update
    Movs(InstrRegister, Value),
    Lsl(InstrRegister, Value, Value),
    Str(InstrRegister, Vec<Value>),
    Bx(InstrRegister),
}

pub type Prgm = Vec<Instr>;
