flux_rs::defs! {
    fn int_to_reg(reg: int, cpu: Armv7m) -> int {
        if reg == 0 {
            cpu.r0
        } else if reg == 1 {
            cpu.r1
        } else if reg == 2 {
            cpu.r2
        } else if reg == 3 {
            cpu.r3
        } else if reg == 4 {
            cpu.r4
        } else if reg == 5 {
            cpu.r5
        } else if reg == 6 {
            cpu.r6
        } else if reg == 7 {
            cpu.r7
        } else if reg == 8 {
            cpu.r8
        } else if reg == 9 {
            cpu.r9
        } else if reg == 10 {
            cpu.r10
        } else if reg == 11 {
            cpu.r11
        } else if reg == 12 {
            cpu.r12
        } else if reg == 13 {
            cpu.r13
        } else if reg == 14 { 
            cpu.r14
        } else {
            cpu.r15
        }
    }

    fn value_into_u32(value: Value, cpu: Armv7m) -> int {
        if value.is_reg {
            int_to_reg(value.val, cpu)
        } else {
            value.val
        }
    }
}


#[flux_rs::refined_by(n: int)]
pub enum GeneralPurposeRegister {
    #[variant(GeneralPurposeRegister[0])]
    R0,
    #[variant(GeneralPurposeRegister[1])]
    R1,
    #[variant(GeneralPurposeRegister[2])]
    R2,
    #[variant(GeneralPurposeRegister[3])]
    R3,
    #[variant(GeneralPurposeRegister[4])]
    R4,
    #[variant(GeneralPurposeRegister[5])]
    R5,
    #[variant(GeneralPurposeRegister[6])]
    R6,
    #[variant(GeneralPurposeRegister[7])]
    R7,
    #[variant(GeneralPurposeRegister[8])]
    R8,
    #[variant(GeneralPurposeRegister[9])]
    R9,
    #[variant(GeneralPurposeRegister[10])]
    R10,
    #[variant(GeneralPurposeRegister[11])]
    R11,
    #[variant(GeneralPurposeRegister[12])]
    R12,
    #[variant(GeneralPurposeRegister[13])]
    R13,
    // Link Register
    #[variant(GeneralPurposeRegister[14])]
    R14,
    #[variant(GeneralPurposeRegister[15])]
    R15,
}

pub enum SpecialRegister {
    Control,
    IPSR,
}

#[flux_rs::refined_by(is_reg: bool, val: int)]
pub enum Value {
    #[variant({GeneralPurposeRegister[@n]} -> Value[true, n])]
    Register(GeneralPurposeRegister),
    #[variant({u32[@n]} -> Value[false, n])]
    Value(u32),
}

pub struct Mem {
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
    pub r0: u32,
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
            r14: 0,
            r15: 0,
            psr: 0,
            primask: 0,
            basepri: 0,
            faultmask: 0,
            control: 0,
        }
    }


    #[flux_rs::sig(fn (&Armv7m[@cpu], Value[@val]) -> u32[value_into_u32(val, cpu)])]
    fn value_into_u32(&self, value: Value) -> u32 {
        match value {
            Value::Register(register) => self.get_value_from_reg(&register),
            Value::Value(v) => v,
        }
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], u32[@new_val]) ensures self: Armv7m { new_cpu: int_to_reg(reg, new_cpu) == new_val })] 
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

    #[flux_rs::sig(fn (&Armv7m[@cpu], &GeneralPurposeRegister[@reg]) -> u32[int_to_reg(reg, cpu)])]
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

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) 
            ensures self: Armv7m { new_cpu: new_cpu.r15 == old_cpu.r15 + 4 } 
    )]
    fn move_pc(&mut self) {
        // Moves the PC (i.e. r15 to the next instruction (i.e. 4 bytes down)
        self.r15 += 4;
    }

    // VTOCK TODO: Check flag updates here

    // Mov
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], Value[@val]) 
        ensures self: Armv7m { new_cpu: 
            int_to_reg(reg, new_cpu) == value_into_u32(val, old_cpu) 
        }
    )]
    pub fn mov(&mut self, register: GeneralPurposeRegister, value: Value) {
        // Move immediate - writes a value into destination register
        // This does not cause a flag update
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);

        self.move_pc();
    }

    // Movs
    // Mov with an N and Z Flag update
    #[flux_rs::trusted]
    pub fn movs(&mut self, register: GeneralPurposeRegister, value: Value) {
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
        // TODO: Flag updates
        self.move_pc();
    }

    // MVN
    #[flux_rs::trusted]
    pub fn mvn(&mut self, register: GeneralPurposeRegister, value: Value) {
        self.move_pc();
        todo!()
    }

    // Msr
    #[flux_rs::trusted]
    pub fn msr(&mut self, register: SpecialRegister, value: Value) {
        let val = self.value_into_u32(value);
        match register {
            SpecialRegister::Control => self.control = val,
            // note this is a bunch of bits under the PSR register so we need to do fancy stuff
            SpecialRegister::IPSR => todo!(),
        }
        // TODO: There are a bunch of flag updates here
        self.move_pc();
    }

    // Isb
    #[flux_rs::trusted]
    pub fn isb(&mut self) {
        // do nothing
        self.move_pc();
    }

    // // Load a word
    // Ldr(InstrRegister, Value),
    #[flux_rs::trusted]
    pub fn ldr(&mut self, register: GeneralPurposeRegister, value: Value) {
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
        // TODO: There are a bunch of flag updates here
        self.move_pc();
    }

    // Mrs
    #[flux_rs::trusted]
    pub fn mrs(&mut self, register: GeneralPurposeRegister, value: SpecialRegister) {
        // Move to register from special register - for moving values from
        // special registers to general purpose registers
        let val = match value {
            SpecialRegister::Control => self.control,
            // note this is a bunch of bits under the PSR register so we need to do fancy stuff
            SpecialRegister::IPSR => todo!(),
        };
        self.update_reg_with_u32(register, val);
        self.move_pc();
    }

    // And
    #[flux_rs::trusted]
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
        self.move_pc();
    }

    // Sub
    #[flux_rs::trusted]
    pub fn sub(&mut self, register: GeneralPurposeRegister, value: Value) {
        // No flag updates here
        let val = self.value_into_u32(value);
        let register_val = self.get_value_from_reg(&register);
        self.update_reg_with_u32(register, register_val - val);
        self.move_pc();
    }

    // Logical Shift Right With a Flag Update
    // Lsrs
    #[flux_rs::trusted]
    pub fn lsrs(&mut self, register: GeneralPurposeRegister, value: Value, value1: Value) {
        let val = self.value_into_u32(value);
        let val1 = self.value_into_u32(value1);
        self.update_reg_with_u32(register, val >> val1);
        // TODO: There are a bunch of flag updates here
        self.move_pc();
    }

    // Lsl(InstrRegister, Value, Value),
    #[flux_rs::trusted]
    pub fn lsl(&mut self, register: GeneralPurposeRegister, value: Value, value1: Value) {
        // No flag updates here
        let val = self.value_into_u32(value);
        let val1 = self.value_into_u32(value1);
        self.update_reg_with_u32(register, val << val1);
        self.move_pc();
    }

    // Str
    #[flux_rs::trusted]
    pub fn str(&mut self, register: GeneralPurposeRegister, value_vec: Vec<Value>) {
        // NOTE: This is a pain - need to update Value to be another instruction
        self.move_pc();
        todo!()
    }

    // Bx
    #[flux_rs::trusted]
    pub fn bx(&mut self, register: GeneralPurposeRegister) {
        // VTOCK TODO: Do nothing but maybe we should make sure that this is the link register
        self.move_pc();
    }
}
