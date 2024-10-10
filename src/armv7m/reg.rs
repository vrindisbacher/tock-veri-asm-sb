use super::instrs::{InstrRegister, Prgm, Value};

#[derive(Debug)]
pub struct Armv7m {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    // r13 is the stack pointer
    r13: u32,
    // r14 is the link register
    r14: u32,
    // r15 is the program counter
    r15: u32,
    //
    // Special Registers below
    //
    // PSR has 3 sub registers:
    //
    // APSR, IPSR, EPSR
    psr: u32,
    // Mask registers
    //
    // primask is 1 bit - the rest being reserved
    primask: u32,
    // basepri is 8 bit - the rest being reserved
    basepri: u32,
    // faultmask is 1 bit - the rest being reserved
    faultmask: u32,
    // Control register (2 bit or 3 bit) depending on the specific processor
    control: u32,
}

impl Armv7m {
    pub fn new() -> Self {
        Armv7m {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            // Link register is special - defaults to this value
            r14: 0xFFFFFFFF,
            r15: 0,
            psr: 0,
            primask: 0,
            basepri: 0,
            faultmask: 0,
            control: 0,
        }
    }

    fn update_reg_with_u32(&mut self, register: InstrRegister, value: u32) {
        match register {
            InstrRegister::R0 => self.r0 = value,
            InstrRegister::R1 => self.r1 = value,
            InstrRegister::R2 => self.r2 = value,
            InstrRegister::R3 => self.r3 = value,
            InstrRegister::R4 => self.r4 = value,
            InstrRegister::R5 => self.r5 = value,
            InstrRegister::R6 => self.r6 = value,
            InstrRegister::R7 => self.r7 = value,
            InstrRegister::R8 => self.r8 = value,
            InstrRegister::R9 => self.r9 = value,
            InstrRegister::R10 => self.r10 = value,
            InstrRegister::R11 => self.r11 = value,
            InstrRegister::R12 => self.r12 = value,
            InstrRegister::R13 => self.r13 = value,
            InstrRegister::Lr => self.r14 = value,
            InstrRegister::R15 => self.r15 = value,
            InstrRegister::Control => self.control = value,
            InstrRegister::Ipsr => todo!(),
        }
    }

    fn get_value_from_reg(&self, register: &InstrRegister) -> u32 {
        match register {
            InstrRegister::R0 => self.r0,
            InstrRegister::R1 => self.r1,
            InstrRegister::R2 => self.r2,
            InstrRegister::R3 => self.r3,
            InstrRegister::R4 => self.r4,
            InstrRegister::R5 => self.r5,
            InstrRegister::R6 => self.r6,
            InstrRegister::R7 => self.r7,
            InstrRegister::R8 => self.r8,
            InstrRegister::R9 => self.r9,
            InstrRegister::R10 => self.r10,
            InstrRegister::R11 => self.r11,
            InstrRegister::R12 => self.r12,
            InstrRegister::R13 => self.r13,
            InstrRegister::Lr => self.r14,
            InstrRegister::R15 => self.r15,
            InstrRegister::Control => self.control,
            InstrRegister::Ipsr => todo!(),
        }
    }

    fn value_into_u32(&self, value: Value) -> u32 {
        match value {
            Value::Register(register) => self.get_value_from_reg(&register),
            Value::Value(v) => v,
        }
    }

    fn move_instr(&mut self, register: InstrRegister, value: Value) {
        // Move immediate - writes a value into destination register
        // This does not cause a flag update
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
    }

    fn moves_instr(&mut self, register: InstrRegister, value: Value) {
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
        // TODO: There are a bunch of flag updates here
    }

    fn ldr_instr(&mut self, register: InstrRegister, value: Value) {
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
        // TODO: There are a bunch of flag updates here
    }

    fn msr_instr(&mut self, register: InstrRegister, value: Value) {
        // Move to register from special register - for moving values from
        // special registers to general purpose registers
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
    }

    fn and_instr(&mut self, register: InstrRegister, value: Value, value1: Value) {
        // Move to register from special register - for moving values from
        // special registers to general purpose registers
        //
        // No flag updates
        let val = self.value_into_u32(value);
        let val1 = self.value_into_u32(value1);
        self.update_reg_with_u32(register, val & val1);
    }

    fn sub_instr(&mut self, register: InstrRegister, value: Value) {
        // No flag updates here
        let val = self.value_into_u32(value);
        let register_val = self.get_value_from_reg(&register);
        self.update_reg_with_u32(register, register_val - val);
    }

    fn lsrs_instr(&mut self, register: InstrRegister, value: Value, value1: Value) {
        let val = self.value_into_u32(value);
        let val1 = self.value_into_u32(value1);
        self.update_reg_with_u32(register, val >> val1);
        // TODO: There are a bunch of flag updates here
    }

    fn lsl_instr(&mut self, register: InstrRegister, value: Value, value1: Value) {
        // No flag updates here
        let val = self.value_into_u32(value);
        let val1 = self.value_into_u32(value1);
        self.update_reg_with_u32(register, val << val1);
    }

    fn str_instr(&mut self, register: InstrRegister, value_vec: Vec<Value>) {
        // NOTE: This is a pain - need to update Value to be another instruction
        todo!()
    }

    pub fn execute(mut self, prgm: Prgm) -> Self {
        for instr in prgm {
            match instr {
                super::instrs::Instr::Mov(register, value) => {
                    // TODO: You can't move into a sepcial register so should probably validate
                    // that
                    self.move_instr(register, value);
                }
                super::instrs::Instr::Msr(register, value) => {
                    // TODO: Validate this is a special register
                    self.move_instr(register, value);
                }
                super::instrs::Instr::Isb => {
                    // This is mostly symbolic in our case
                    //
                    // Generally used to ensure a write to the control register is done
                    // before the next instruction is executed
                    //
                    // In our case, we don't care
                }
                super::instrs::Instr::Ldr(register, value) => {
                    self.ldr_instr(register, value);
                }
                super::instrs::Instr::Mrs(register, value) => {
                    self.msr_instr(register, value);
                }
                super::instrs::Instr::And(register, value, value1) => {
                    self.and_instr(register, value, value1);
                }
                super::instrs::Instr::Sub(register, value) => {
                    self.sub_instr(register, value);
                }
                super::instrs::Instr::Lsrs(register, value, value1) => {
                    self.lsrs_instr(register, value, value1);
                }
                super::instrs::Instr::Movs(register, value) => {
                    self.moves_instr(register, value);
                }
                super::instrs::Instr::Lsl(register, value, value1) => {
                    self.lsl_instr(register, value, value1);
                }
                super::instrs::Instr::Str(register, vec) => {
                    self.str_instr(register, vec);
                }
                super::instrs::Instr::Bx(_register) => {
                    // This instruction branches to an address or instruction set
                    // specified by a register
                    // In this case, do we actually care? Probably not
                }
            }
        }
        self
    }
}
