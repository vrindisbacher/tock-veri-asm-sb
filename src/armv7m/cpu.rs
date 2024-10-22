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
    // MEMORY
    /* System Control Space Start */
    // sys control id regs
    ictr: int,    
    actlr: int,    
    stir: int,    
    pid4: int,    
    pid5: int,    
    pid6: int,    
    pid7: int,    
    pid0: int,    
    pid1: int,    
    pid2: int,    
    pid3: int,    
    cid0: int,    
    cid1: int,    
    cid2: int,    
    cid3: int,
    // sys control blocks
    cpuid: int,    
    icsr: int,    
    vtor: int,    
    aircr: int,    
    scr: int,    
    ccr: int,    
    shpr1: int,    
    shpr2: int,    
    shpr3: int,    
    shcsr: int,    
    cfsr: int,    
    hfsr: int,    
    dfsr: int,    
    mmfar: int,    
    bfar: int,    
    afsr: int,    
    cpacr: int,
    /* System Control Space End */
    /* Sys Tick Start */
    syst_csr: int,
    syst_rvr: int,
    syst_cvr: int,
    syst_calib: int,
    /* Sys Tick End */
    /* NVIC START */
    iser0: int,
    iser1: int,
    iser2: int,
    iser3: int,
    iser4: int,
    iser5: int,
    iser6: int,
    iser7: int,
    iser8: int,
    iser9: int,
    iser10: int,
    iser11: int,
    iser12: int,
    iser13: int,
    iser14: int,
    iser15: int,
    icer0: int,
    icer1: int,
    icer2: int,
    icer3: int,
    icer4: int,
    icer5: int,
    icer6: int,
    icer7: int,
    icer8: int,
    icer9: int,
    icer10: int,
    icer11: int,
    icer12: int,
    icer13: int,
    icer14: int,
    icer15: int,
    ispr0: int,
    ispr1: int,
    ispr2: int,
    ispr3: int,
    ispr4: int,
    ispr5: int,
    ispr6: int,
    ispr7: int,
    ispr8: int,
    ispr9: int,
    ispr10: int,
    ispr11: int,
    ispr12: int,
    ispr13: int,
    ispr14: int,
    ispr15: int,
    icpr0: int,
    icpr1: int,
    icpr2: int,
    icpr3: int,
    icpr4: int,
    icpr5: int,
    icpr6: int,
    icpr7: int,
    icpr8: int,
    icpr9: int,
    icpr10: int,
    icpr11: int,
    icpr12: int,
    icpr13: int,
    icpr14: int,
    icpr15: int,
    iabr0: int,
    iabr1: int,
    iabr2: int,
    iabr3: int,
    iabr4: int,
    iabr5: int,
    iabr6: int,
    iabr7: int,
    iabr8: int,
    iabr9: int,
    iabr10: int,
    iabr11: int,
    iabr12: int,
    iabr13: int,
    iabr14: int,
    iabr15: int,
    ipr0: int,
    ipr1: int,
    ipr2: int,
    ipr3: int,
    ipr4: int,
    ipr5: int,
    ipr6: int,
    ipr7: int,
    ipr8: int,
    ipr9: int,
    ipr10: int,
    ipr11: int,
    ipr12: int,
    ipr13: int,
    ipr14: int,
    ipr15: int,
    ipr16: int,
    ipr17: int,
    ipr18: int,
    ipr19: int,
    ipr20: int,
    ipr21: int,
    ipr22: int,
    ipr23: int,
    ipr24: int,
    ipr25: int,
    ipr26: int,
    ipr27: int,
    ipr28: int,
    ipr29: int,
    ipr30: int,
    ipr31: int,
    ipr32: int,
    ipr33: int,
    ipr34: int,
    ipr35: int,
    ipr36: int,
    ipr37: int,
    ipr38: int,
    ipr39: int,
    ipr40: int,
    ipr41: int,
    ipr42: int,
    ipr43: int,
    ipr44: int,
    ipr45: int,
    ipr46: int,
    ipr47: int,
    ipr48: int,
    ipr49: int,
    ipr50: int,
    ipr51: int,
    ipr52: int,
    ipr53: int,
    ipr54: int,
    ipr55: int,
    ipr56: int,
    ipr57: int,
    ipr58: int,
    ipr59: int,
    ipr60: int,
    ipr61: int,
    ipr62: int,
    ipr63: int,
    ipr64: int,
    ipr65: int,
    ipr66: int,
    ipr67: int,
    ipr68: int,
    ipr69: int,
    ipr70: int,
    ipr71: int,
    ipr72: int,
    ipr73: int,
    ipr74: int,
    ipr75: int,
    ipr76: int,
    ipr77: int,
    ipr78: int,
    ipr79: int,
    ipr80: int,
    ipr81: int,
    ipr82: int,
    ipr83: int,
    ipr84: int,
    ipr85: int,
    ipr86: int,
    ipr87: int,
    ipr88: int,
    ipr89: int,
    ipr90: int,
    ipr91: int,
    ipr92: int,
    ipr93: int,
    ipr94: int,
    ipr95: int,
    ipr96: int,
    ipr97: int,
    ipr98: int,
    ipr99: int,
    ipr100: int,
    ipr101: int,
    ipr102: int,
    ipr103: int,
    ipr104: int,
    ipr105: int,
    ipr106: int,
    ipr107: int,
    ipr108: int,
    ipr109: int,
    ipr110: int,
    ipr111: int,
    ipr112: int,
    ipr113: int,
    ipr114: int,
    ipr115: int,
    ipr116: int,
    ipr117: int,
    ipr118: int,
    ipr119: int,
    ipr120: int,
    ipr121: int,
    ipr122: int,
    ipr123: int,
    /* NVIC END */
    /* MPU START */
    mpu_type: int,
    mpu_ctrl: int,
    mpu_rnr: int,
    mpu_rbar: int,
    mpu_rasr: int,
    mpu_rbar_a1: int,
    mpu_rasr_a1: int,
    mpu_rbar_a2: int,
    mpu_rasr_a2: int,
    mpu_rbar_a3: int,
    mpu_rasr_a3: int
    /* MPU END */
    
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
    // Memory 
    #[field(Memory[
        ictr,    
        actlr,    
        stir,    
        pid4,    
        pid5,    
        pid6,    
        pid7,    
        pid0,    
        pid1,    
        pid2,    
        pid3,    
        cid0,    
        cid1,    
        cid2,    
        cid3,
        cpuid,    
        icsr,    
        vtor,    
        aircr,    
        scr,    
        ccr,    
        shpr1,    
        shpr2,    
        shpr3,    
        shcsr,    
        cfsr,    
        hfsr,    
        dfsr,    
        mmfar,    
        bfar,    
        afsr,    
        cpacr,
        syst_csr,
        syst_rvr,
        syst_cvr,
        syst_calib,
        iser0,
        iser1,
        iser2,
        iser3,
        iser4,
        iser5,
        iser6,
        iser7,
        iser8,
        iser9,
        iser10,
        iser11,
        iser12,
        iser13,
        iser14,
        iser15,
        icer0,
        icer1,
        icer2,
        icer3,
        icer4,
        icer5,
        icer6,
        icer7,
        icer8,
        icer9,
        icer10,
        icer11,
        icer12,
        icer13,
        icer14,
        icer15,
        ispr0,
        ispr1,
        ispr2,
        ispr3,
        ispr4,
        ispr5,
        ispr6,
        ispr7,
        ispr8,
        ispr9,
        ispr10,
        ispr11,
        ispr12,
        ispr13,
        ispr14,
        ispr15,
        icpr0,
        icpr1,
        icpr2,
        icpr3,
        icpr4,
        icpr5,
        icpr6,
        icpr7,
        icpr8,
        icpr9,
        icpr10,
        icpr11,
        icpr12,
        icpr13,
        icpr14,
        icpr15,
        iabr0,
        iabr1,
        iabr2,
        iabr3,
        iabr4,
        iabr5,
        iabr6,
        iabr7,
        iabr8,
        iabr9,
        iabr10,
        iabr11,
        iabr12,
        iabr13,
        iabr14,
        iabr15,
        ipr0,
        ipr1,
        ipr2,
        ipr3,
        ipr4,
        ipr5,
        ipr6,
        ipr7,
        ipr8,
        ipr9,
        ipr10,
        ipr11,
        ipr12,
        ipr13,
        ipr14,
        ipr15,
        ipr16,
        ipr17,
        ipr18,
        ipr19,
        ipr20,
        ipr21,
        ipr22,
        ipr23,
        ipr24,
        ipr25,
        ipr26,
        ipr27,
        ipr28,
        ipr29,
        ipr30,
        ipr31,
        ipr32,
        ipr33,
        ipr34,
        ipr35,
        ipr36,
        ipr37,
        ipr38,
        ipr39,
        ipr40,
        ipr41,
        ipr42,
        ipr43,
        ipr44,
        ipr45,
        ipr46,
        ipr47,
        ipr48,
        ipr49,
        ipr50,
        ipr51,
        ipr52,
        ipr53,
        ipr54,
        ipr55,
        ipr56,
        ipr57,
        ipr58,
        ipr59,
        ipr60,
        ipr61,
        ipr62,
        ipr63,
        ipr64,
        ipr65,
        ipr66,
        ipr67,
        ipr68,
        ipr69,
        ipr70,
        ipr71,
        ipr72,
        ipr73,
        ipr74,
        ipr75,
        ipr76,
        ipr77,
        ipr78,
        ipr79,
        ipr80,
        ipr81,
        ipr82,
        ipr83,
        ipr84,
        ipr85,
        ipr86,
        ipr87,
        ipr88,
        ipr89,
        ipr90,
        ipr91,
        ipr92,
        ipr93,
        ipr94,
        ipr95,
        ipr96,
        ipr97,
        ipr98,
        ipr99,
        ipr100,
        ipr101,
        ipr102,
        ipr103,
        ipr104,
        ipr105,
        ipr106,
        ipr107,
        ipr108,
        ipr109,
        ipr110,
        ipr111,
        ipr112,
        ipr113,
        ipr114,
        ipr115,
        ipr116,
        ipr117,
        ipr118,
        ipr119,
        ipr120,
        ipr121,
        ipr122,
        ipr123,
        mpu_type,
        mpu_ctrl,
        mpu_rnr,
        mpu_rbar,
        mpu_rasr,
        mpu_rbar_a1,
        mpu_rasr_a1,
        mpu_rbar_a2,
        mpu_rasr_a2,
        mpu_rbar_a3,
        mpu_rasr_a3
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
