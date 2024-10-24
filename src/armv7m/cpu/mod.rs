mod insns;
mod psr;
mod flux_defs;

use insns::utils::get_nth_bit;
use super::mem::Memory;
use super::lang::{GeneralPurposeRegister, SpecialRegister, Value};
use flux_defs::*;

// The following is a struct that represents the CPU of the ARMv7m processor architecture
//
// There are thirteen general-purpose 32-bit registers, R0-R12, and an additional three 32-bit registers that have special
// names and usage models.
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
    psr: int,
    control: int,
    mem: Memory
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
    #[field(u32[psr])]
    psr: u32,
    #[field(u32[control])]
    control: u32,
    // Memory 
    #[field(Memory[mem])]
    mem: Memory,
}

impl Armv7m {

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (&Armv7m[@cpu], SpecialRegister[@reg]) -> u32[get_special_reg(reg, cpu)])]
    fn get_value_from_special_reg(&self, reg: SpecialRegister) -> u32 {
            match reg {
                SpecialRegister::Control => self.control,
                SpecialRegister::PSR => self.psr,
                // the last 8 bits of the PSR register 
                SpecialRegister::IPSR => self.psr & 0xff,
            }
    }


    #[flux_rs::sig(fn (&Armv7m[@cpu], Value[@val]) -> u32[value_into_u32(val, cpu)])]
    fn value_into_u32(&self, value: Value) -> u32 {
        match value {
            Value::SpecialRegister(register) => self.get_value_from_special_reg(register),
            Value::GeneralRegister(register) => self.get_value_from_general_reg(&register),
            Value::Value(v) => v,
        }
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], u32[@new_val]) 
            // no updates to PC or SP allowed
            requires !(is_pc(reg) || is_sp(reg))
            ensures self: Armv7m { 
                new_cpu: get_general_purpose_reg(reg, new_cpu) == new_val 
                            && new_cpu.sp == old_cpu.sp
                            && new_cpu.pc == old_cpu.pc
            }
    )] 
    fn update_general_reg_with_u32(&mut self, register: GeneralPurposeRegister, value: u32) {
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
            GeneralPurposeRegister::Lr => self.lr = value,
            GeneralPurposeRegister::Sp => panic!("Cannot update Stack Pointer in a direct manner"), // self.sp = value,
            GeneralPurposeRegister::Pc => panic!("Cannot update program counter in a direct manner"), 
        }
    }

    #[flux_rs::sig(fn (&Armv7m[@cpu], &GeneralPurposeRegister[@reg]) -> u32[get_general_purpose_reg(reg, cpu)])]
    fn get_value_from_general_reg(&self, register: &GeneralPurposeRegister) -> u32 {
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
            GeneralPurposeRegister::Sp => self.sp,
            GeneralPurposeRegister::Lr => self.lr,
            GeneralPurposeRegister::Pc => self.pc, 
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

    #[flux_rs::sig(fn (&Armv7m[@cpu]) -> bool[itstate_0_4_not_all_zero(cpu)] )]
    fn in_if_then_block(&self) -> bool {
        // See page B1-517 for where IT lies in EPSR register
        //
        // Use EPSR[26:25] EPSR[15:12] EPSR[11:10] Additional Information
        // IT  IT[1:0]      IT[7:4]    IT[3:2]     See ITSTATE on page A7-179
        //
        // See A7-180 for pseudo code for InItBlock
        let bit_0 = get_nth_bit(self.psr, 25) == 0;
        let bit_1 = get_nth_bit(self.psr, 26) == 0;
        let bit_2 = get_nth_bit(self.psr, 10) == 0;
        let bit_3 = get_nth_bit(self.psr, 11) == 0;
        !(bit_0 && bit_1 && bit_2 && bit_3)

    }
}
