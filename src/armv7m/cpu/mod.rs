mod flux_defs;
mod insns;
mod psr;

use super::lang::{SpecialRegister, GPR};
use super::mem::Memory;
use crate::flux_support::bv32::BV32;
use crate::flux_support::rmap::Regs;
use flux_defs::*;

pub type ArmGeneralRegs = Regs<GPR, BV32>;
pub type ArmSpecialRegs = Regs<SpecialRegister, BV32>;

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
    general_regs: Map<GPR, BV32>,
    special_regs: Map<SpecialRegister, BV32>,
    mem: Memory
)]
pub struct Armv7m {
    // General Registers r0 - r11
    #[field(Regs<GPR, BV32>[general_regs])]
    pub general_regs: ArmGeneralRegs,
    // Special Registers
    #[field(Regs<SpecialRegister, BV32>[special_regs])]
    pub special_regs: ArmSpecialRegs,
    // Memory
    #[field(Memory[mem])]
    pub mem: Memory,
}

impl Armv7m {
    #[flux_rs::trusted]
    #[flux_rs::sig(fn (&Armv7m[@cpu], &SpecialRegister[@reg]) -> BV32[get_special_reg(reg, cpu)])]
    fn get_value_from_special_reg(&self, register: &SpecialRegister) -> BV32 {
        *self.special_regs.get(register).unwrap()
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@reg], BV32[@val])
            ensures self: Armv7m[{ special_regs: set_spr(reg, old_cpu, val), ..old_cpu }] 
    )]
    fn update_special_reg_with_b32(&mut self, register: SpecialRegister, value: BV32) {
        self.special_regs.set(register, value);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], GPR[@reg], BV32[@val]) 
            ensures self: Armv7m[{ general_regs: set_gpr(reg, old_cpu, val), ..old_cpu }] 
    )]
    fn update_general_reg_with_b32(&mut self, register: GPR, value: BV32) {
        self.general_regs.set(register, value);
    }

    #[flux_rs::sig(fn (&Armv7m[@cpu], &GPR[@reg]) -> BV32[get_gpr(reg, cpu)])]
    fn get_value_from_general_reg(&self, register: &GPR) -> BV32 {
        *self.general_regs.get(register).unwrap()
    }

    // #[flux_rs::sig(fn (&Armv7m[@cpu]) -> bool[itstate_0_4_not_all_zero(cpu)] )]
    // fn in_if_then_block(&self) -> bool {
    //     // See page B1-517 for where IT lies in EPSR register
    //     //
    //     // Use EPSR[26:25] EPSR[15:12] EPSR[11:10] Additional Information
    //     // IT  IT[1:0]      IT[7:4]    IT[3:2]     See ITSTATE on page A7-179
    //     //
    //     // See A7-180 for pseudo code for InItBlock
    //     let bit_0 = get_nth_bit(self.psr, 25) == 0;
    //     let bit_1 = get_nth_bit(self.psr, 26) == 0;
    //     let bit_2 = get_nth_bit(self.psr, 10) == 0;
    //     let bit_3 = get_nth_bit(self.psr, 11) == 0;
    //     !(bit_0 && bit_1 && bit_2 && bit_3)
    // }
}
