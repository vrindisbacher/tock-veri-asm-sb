mod exception;
mod flux_defs;
mod insns;
mod isr;
mod psr;

use super::lang::{SpecialRegister, GPR};
use super::mem::Memory;
use crate::flux_support::rmap::Regs;
use flux_defs::*;
use flux_rs::bitvec::BV32;

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
#[derive(Debug)]
#[flux_rs::refined_by(mode: int)]
pub enum CPUMode {
    #[variant(CPUMode[0])]
    Handler,
    #[variant(CPUMode[1])]
    Thread,
}

#[derive(Debug)]
#[flux_rs::refined_by(sp_main: BV32, sp_process: BV32)]
#[flux_rs::invariant(is_valid_ram_addr(sp_main))]
#[flux_rs::invariant(is_valid_ram_addr(sp_process))]
pub struct SP {
    #[field({ BV32[sp_main] | is_valid_ram_addr(sp_main) })]
    pub sp_main: BV32,
    #[field({ BV32[sp_process] | is_valid_ram_addr(sp_process) })]
    pub sp_process: BV32,
}

#[derive(Debug)]
#[flux_rs::refined_by(npriv: bool, spsel: bool)]
pub struct Control {
    // both of these are either 0 or 1 so using bools
    // 0 - Thread mode has privileged access
    // 1 - Thread mode has unprivileged access
    #[field(bool[npriv])]
    pub npriv: bool,
    // 0 use sp_main
    // 1 In Thread mode, use SP_process as the current stack. In Handler mode, this value is reserved.
    #[field(bool[spsel])]
    pub spsel: bool,
}

#[derive(Debug)]
#[flux_rs::refined_by(
    general_regs: Map<GPR, BV32>,
    sp: SP,
    control: Control,
    lr: BV32,
    pc: BV32,
    psr: BV32,
    mem: Memory,
    mode: CPUMode
)]
pub struct Armv7m {
    // General Registers r0 - r11
    #[field(Regs<GPR, BV32>[general_regs])]
    pub general_regs: ArmGeneralRegs,
    // Stack Pointer
    #[field(SP[sp])]
    pub sp: SP,
    // Control register
    #[field(Control[control])]
    pub control: Control,
    // Program Counter
    #[field(BV32[pc])]
    pub pc: BV32,
    // Link register
    #[field(BV32[lr])]
    pub lr: BV32,
    // program status register
    #[field(BV32[psr])]
    pub psr: BV32,
    // Memory
    #[field(Memory[mem])]
    pub mem: Memory,
    // current CPU mode
    #[field(CPUMode[mode])]
    pub mode: CPUMode,
}

impl Armv7m {
    #[flux_rs::sig(fn (&Armv7m[@cpu], &SpecialRegister[@reg]) -> BV32[get_special_reg(reg, cpu)])]
    fn get_value_from_special_reg(&self, register: &SpecialRegister) -> BV32 {
        match register {
            SpecialRegister::PSP => self.sp.sp_process,
            SpecialRegister::Sp => {
                // Thread mode: Main, else
                // check spsel
                // 0 use sp_main
                // 1 In Thread mode, use SP_process as the current stack. In Handler mode, this value is reserved
                if self.mode_is_handler() || !self.control.spsel {
                    self.sp.sp_main
                } else {
                    self.sp.sp_process
                }
            }
            SpecialRegister::Lr => self.lr,
            SpecialRegister::Pc => self.pc,
            SpecialRegister::Control => {
                if self.control.npriv && self.control.spsel {
                    BV32::from(3)
                } else if self.control.npriv {
                    // first bit is 1 - i.e. 01
                    BV32::from(1)
                } else if self.control.spsel {
                    // second bit is 1 - i.e. 10
                    BV32::from(2)
                } else {
                    BV32::from(0)
                }
            }
            SpecialRegister::PSR => self.psr,
            SpecialRegister::IPSR => self.psr & BV32::from(0xff),
        }
    }

    #[flux_rs::sig(fn (&Armv7m[@cpu]) -> bool[mode_is_handler(cpu.mode)])]
    fn mode_is_handler(&self) -> bool {
        match self.mode {
            CPUMode::Handler => true,
            CPUMode::Thread => false,
        }
    }

    #[flux_rs::sig(fn (BV32[@val], BV32[@n]) -> bool[nth_bit_is_set(val, n)])]
    fn nth_bit_set(value: BV32, n: BV32) -> bool {
        (value & (BV32::from(1) << n)) != BV32::from(0)
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@reg], BV32[@val])
            requires
            (is_sp(reg) || is_psp(reg)) => is_valid_ram_addr(val)
            ensures self: Armv7m { new_cpu: new_cpu == set_spr(reg, old_cpu, val) }
    )]
    fn update_special_reg_with_b32(&mut self, register: SpecialRegister, value: BV32) {
        match register {
            SpecialRegister::PSP => {
                self.sp.sp_process = value;
            }
            SpecialRegister::Sp => {
                if self.mode_is_handler() || !self.control.spsel {
                    // updates sp_main
                    self.sp.sp_main = value;
                } else {
                    self.sp.sp_process = value;
                }
            }
            SpecialRegister::Lr => {
                self.lr = value;
            }
            SpecialRegister::Pc => {
                self.pc = value;
            }
            SpecialRegister::Control => {
                // TODO: This is only ok in privileged mode
                let npriv_bit_set = Self::nth_bit_set(value, BV32::from(0));
                self.control.npriv = npriv_bit_set;
                if !self.mode_is_handler() {
                    let spsel_bit_set = Self::nth_bit_set(value, BV32::from(1));
                    self.control.spsel = spsel_bit_set;
                }
            }
            SpecialRegister::PSR => self.psr = value,
            // IPSR updates do nothing
            _ => {}
        }
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], GPR[@reg], BV32[@val])
        ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                general_regs: set_gpr(reg, old_cpu, val), ..old_cpu
            }
        }
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
