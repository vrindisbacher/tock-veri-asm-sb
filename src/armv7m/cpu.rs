use super::{instr::{GeneralPurposeRegister, SpecialRegister, Value}, mem::Memory};

flux_rs::defs! {
    fn get_reg(reg: int, cpu: Armv7m) -> int {
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
            cpu.sp
        } else if reg == 14 { 
            cpu.lr
        } else {
            cpu.pc
        }
    }

    fn value_into_u32(value: Value, cpu: Armv7m) -> int {
        if value.is_reg {
            get_reg(value.val, cpu)
        } else {
            value.val
        }
    }
}


// The following is a struct that represents the CPU of the ARMv7m processor architecture
//
// There are thirteen general-purpose 32-bit registers, R0-R12, and an additional three 32-bit registers that have special
// names and usage models.
//
//
// Permissions are:
//      Read or write R0-R12, SP, and LR
//      Read the PC
//
// There are also special registers. These are:
//
//      APSR register: Program status is reported in the 32-bit Application Program Status Register
//      (APSR). The flags in this register are:
//
//      - N, bit[31] Negative condition flag. Set to bit[31] of the result of the instruction. If the result is regarded as
//      a two's complement signed integer, then N == 1 if the result is negative and N == 0 if it is positive
//      or zero.
//
//      - Z, bit[30] Zero condition flag. Set to 1 if the result of the instruction is zero, and to 0 otherwise. A result of
//      zero often indicates an equal result from a comparison.
//
//      - C, bit[29] Carry condition flag. Set to 1 if the instruction results in a carry condition, for example an
//      unsigned overflow on an addition.
//
//      - V, bit[28] Overflow condition flag. Set to 1 if the instruction results in an overflow condition, for example
//      a signed overflow on an addition.
//
//      - Q, bit[27] Set to 1 if a SSAT or USAT instruction changes the input value for the signed or unsigned range of
//      the result. In a processor that implements the DSP extension, the processor sets this bit to 1 to
//      indicate an overflow on some multiplies. Setting this bit to 1 is called saturation.
//
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
    sp: int,
    lr: int,
    pc: int,
    apsr: int,
    // Memory Layout
    code: int,
    sram: int,
    periph: int,
    ram: int,
    device: int,
    ppb: int,
    vendor_sys: int
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
    #[field(u32[sp])]
    sp: u32,
    #[field(u32[lr])]
    lr: u32,
    #[field(u32[pc])]
    pc: u32,
    #[field(u32[apsr])]
    apsr: u32,
    #[field(Memory[
        code, sram, periph, ram, device, ppb, vendor_sys
    ])]
    mem: Memory,
}

impl Armv7m {
    #[flux_rs::sig(fn (&Armv7m[@cpu], Value[@val]) -> u32[value_into_u32(val, cpu)])]
    fn value_into_u32(&self, value: Value) -> u32 {
        match value {
            Value::Register(register) => self.get_value_from_reg(&register),
            Value::Value(v) => v,
        }
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], u32[@new_val]) ensures self: Armv7m { new_cpu: get_reg(reg, new_cpu) == new_val })] 
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
        }
    }

    #[flux_rs::sig(fn (&Armv7m[@cpu], &GeneralPurposeRegister[@reg]) -> u32[get_reg(reg, cpu)])]
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
        }
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) 
            ensures self: Armv7m { new_cpu: new_cpu.pc == old_cpu.pc + 4 } 
    )]
    fn move_pc(&mut self) {
        // Moves the PC (i.e. r15 to the next instruction (i.e. 4 bytes down)
        self.pc += 4;
    }

    // VTOCK TODO: Check flag updates here

    // Mov
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], Value[@val]) 
        ensures self: Armv7m { 
            new_cpu: get_reg(reg, new_cpu) == value_into_u32(val, old_cpu) 
        }
    )]
    // Interesting note: incr the program counter is an issue for this refinement
    pub fn mov(&mut self, register: GeneralPurposeRegister, value: Value) {
        // Move immediate - writes a value into destination register
        // This does not cause a flag update
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);

        // self.move_pc();
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
        todo!()
        // let val = self.value_into_u32(value);
        // match register {
        //     SpecialRegister::Control => // self.control = val,
        //     // note this is a bunch of bits under the PSR register so we need to do fancy stuff
        //     SpecialRegister::IPSR => todo!(),
        // }
        // TODO: There are a bunch of flag updates here
        // self.move_pc();
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
        // let val = match value {
        //     SpecialRegister::Control => todo!() // self.control,
        //     // note this is a bunch of bits under the PSR register so we need to do fancy stuff
        //     SpecialRegister::IPSR => todo!(),
        // };
        // self.update_reg_with_u32(register, val);
        // self.move_pc();
        todo!()
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
