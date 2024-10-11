pub enum GeneralPurposeRegister {
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
    // Link Register
    R14,
    R15,
}

pub enum SpecialRegister {
    Control,
    ISPR,
}

pub enum Value {
    Register(GeneralPurposeRegister),
    Value(u32),
}

#[derive(Debug)]
#[flux_rs::refined_by(
    r0: int, 
    r1: int,
    r2: int, 
    r3: int,
    r4: int,
    r5: int,
    r6: int,
    r7: int,
    r8: int,
    r9: int,
    r10: int, 
    r11: int,
    r12: int,
    r13: int,
    r14: int,
    r15: int,
    psr: int,
    primask: int,
    basepri: int,
    faultmask: int,
    control: int,
)]
pub struct Armv7m {
    #[field(u32[r0])]
    r0: u32,
    #[field(u32[r1])]
    r1: u32,
    #[field(u32[r2])]
    r2: u32,
    #[field(u32[r3])]
    r3: u32,
    #[field(u32[r4])]
    r4: u32,
    #[field(u32[r5])]
    r5: u32,
    #[field(u32[r6])]
    r6: u32,
    #[field(u32[r7])]
    r7: u32,
    #[field(u32[r8])]
    r8: u32,
    #[field(u32[r9])]
    r9: u32,
    #[field(u32[r10])]
    r10: u32,
    #[field(u32[r11])]
    r11: u32,
    #[field(u32[r12])]
    r12: u32,
    // r13 is the stack pointer
    #[field(u32[r13])]
    r13: u32,
    // r14 is the link register
    #[field(u32[r14])]
    r14: u32,
    // r15 is the program counter
    #[field(u32[r15])]
    r15: u32,
    //
    // Special Registers below
    //
    // PSR has 3 sub registers:
    //
    // APSR, IPSR, EPSR
    #[field(u32[psr])]
    psr: u32,
    // Mask registers
    //
    // primask is 1 bit - the rest being reserved
    #[field(u32[primask])]
    primask: u32,
    // basepri is 8 bit - the rest being reserved
    #[field(u32[basepri])]
    basepri: u32,
    // faultmask is 1 bit - the rest being reserved
    #[field(u32[faultmask])]
    faultmask: u32,
    // Control register (2 bit or 3 bit) depending on the specific processor
    #[field(u32[control])]
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

    fn value_into_u32(&self, value: Value) -> u32 {
        match value {
            Value::Register(register) => self.get_value_from_reg(&register),
            Value::Value(v) => v,
        }
    }

    fn update_reg_with_u32(&mut self, register: GeneralPurposeRegister, value: u32) {
        match register {
            GeneralPurposeRegister::R0 => self.r0 = value,
            GeneralPurposeRegister::R1 => self.r1 = value,
            GeneralPurposeRegister::R2 => self.r2 = value,
            GeneralPurposeRegister::R3 => self.r3 = value,
            GeneralPurposeRegister::R4 => self.r4 = value,
            GeneralPurposeRegister::R5 => self.r5 = value,
            GeneralPurposeRegister::R6 => self.r6 = value,
            GeneralPurposeRegister::R7 => self.r7 = value,
            GeneralPurposeRegister::R8 => self.r8 = value,
            GeneralPurposeRegister::R9 => self.r9 = value,
            GeneralPurposeRegister::R10 => self.r10 = value,
            GeneralPurposeRegister::R11 => self.r11 = value,
            GeneralPurposeRegister::R12 => self.r12 = value,
            GeneralPurposeRegister::R13 => self.r13 = value,
            GeneralPurposeRegister::R14 => self.r14 = value,
            GeneralPurposeRegister::R15 => self.r15 = value,
        }
    }

    fn get_value_from_reg(&self, register: &GeneralPurposeRegister) -> u32 {
        match register {
            GeneralPurposeRegister::R0 => self.r0,
            GeneralPurposeRegister::R1 => self.r1,
            GeneralPurposeRegister::R2 => self.r2,
            GeneralPurposeRegister::R3 => self.r3,
            GeneralPurposeRegister::R4 => self.r4,
            GeneralPurposeRegister::R5 => self.r5,
            GeneralPurposeRegister::R6 => self.r6,
            GeneralPurposeRegister::R7 => self.r7,
            GeneralPurposeRegister::R8 => self.r8,
            GeneralPurposeRegister::R9 => self.r9,
            GeneralPurposeRegister::R10 => self.r10,
            GeneralPurposeRegister::R11 => self.r11,
            GeneralPurposeRegister::R12 => self.r12,
            GeneralPurposeRegister::R13 => self.r13,
            GeneralPurposeRegister::R14 => self.r14,
            GeneralPurposeRegister::R15 => self.r15,
        }
    }

    // VTOCK TODO: Check flag updates here

    // Mov
    pub fn mov(&mut self, register: GeneralPurposeRegister, value: Value) {
        // Move immediate - writes a value into destination register
        // This does not cause a flag update
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
    }

    // Movs
    // Mov with an N and Z Flag update
    pub fn movs(&mut self, register: GeneralPurposeRegister, value: Value) {
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
        // TODO: Flag updates
    }

    // Msr
    pub fn msr(&mut self, register: SpecialRegister, value: Value) {
        let val = self.value_into_u32(value);
        match register {
            SpecialRegister::Control => self.control = val,
            // note this is a bunch of bits under the PSR register so we need to do fancy stuff
            SpecialRegister::ISPR => todo!(),
        }
        // TODO: There are a bunch of flag updates here
    }

    // Isb
    pub fn isb(&mut self) {
        // do nothing
    }
    // // Load a word
    // Ldr(InstrRegister, Value),
    pub fn ldr(&mut self, register: GeneralPurposeRegister, value: Value) {
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
        // TODO: There are a bunch of flag updates here
    }

    // Mrs
    pub fn mrs(&mut self, register: GeneralPurposeRegister, value: SpecialRegister) {
        // Move to register from special register - for moving values from
        // special registers to general purpose registers
        let val = match value {
            SpecialRegister::Control => self.control,
            // note this is a bunch of bits under the PSR register so we need to do fancy stuff
            SpecialRegister::ISPR => todo!(),
        };
        self.update_reg_with_u32(register, val);
    }

    // And
    pub fn and(&mut self, register: GeneralPurposeRegister, value: Value, value1: Option<Value>) {
        // No flag updates
        let val = self.value_into_u32(value);
        match value1 {
            Some(val1) => {
                let val1 = self.value_into_u32(val1);
                self.update_reg_with_u32(register, val & val1);
            }
            None => {
                let reg_val = self.get_value_from_reg(&register);
                self.update_reg_with_u32(register, reg_val & val);
            }
        }
    }

    // Sub
    pub fn sub(&mut self, register: GeneralPurposeRegister, value: Value) {
        // No flag updates here
        let val = self.value_into_u32(value);
        let register_val = self.get_value_from_reg(&register);
        self.update_reg_with_u32(register, register_val - val);
    }

    // Logical Shift Right With a Flag Update
    // Lsrs
    pub fn lsrs(&mut self, register: GeneralPurposeRegister, value: Value, value1: Value) {
        let val = self.value_into_u32(value);
        let val1 = self.value_into_u32(value1);
        self.update_reg_with_u32(register, val >> val1);
        // TODO: There are a bunch of flag updates here
    }

    // Lsl(InstrRegister, Value, Value),
    pub fn lsl(&mut self, register: GeneralPurposeRegister, value: Value, value1: Value) {
        // No flag updates here
        let val = self.value_into_u32(value);
        let val1 = self.value_into_u32(value1);
        self.update_reg_with_u32(register, val << val1);
    }

    // Str
    pub fn str(&mut self, register: GeneralPurposeRegister, value_vec: Vec<Value>) {
        // NOTE: This is a pain - need to update Value to be another instruction
        todo!()
    }

    // Bx
    pub fn bx(&mut self, register: GeneralPurposeRegister) {
        // VTOCK TODO: DO nothing but maybe we should make sure that this is the link register
    }
}
