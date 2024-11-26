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
#[flux_rs::invariant(is_valid_ram_addr(int(sp_main)))]
#[flux_rs::invariant(is_valid_ram_addr(int(sp_process)))]
pub struct SP {
    #[field({ BV32[sp_main] | is_valid_ram_addr(int(sp_main)) })]
    pub sp_main: BV32,
    #[field({ BV32[sp_process] | is_valid_ram_addr(int(sp_process)) })]
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
            CPUMode::Thread => false
        }
    }

    #[flux_rs::sig(fn (BV32[@val], BV32[@n]) -> bool[nth_bit_is_set(val, n)])]
    fn nth_bit_set(value: BV32, n: BV32) -> bool {
        (value & (BV32::from(1) << n)) != BV32::from(0)
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@reg], BV32[@val])
            requires is_sp(reg) => is_valid_ram_addr(int(val))
            ensures self: Armv7m[set_spr(reg, old_cpu, val)] 
    )]
    fn update_special_reg_with_b32(&mut self, register: SpecialRegister, value: BV32) {
        match register {
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
                let npriv_bit_set = Self::nth_bit_set(value, BV32::from(1));
                self.control.npriv = npriv_bit_set;
                if !self.mode_is_handler() {
                    let spsel_bit_set = Self::nth_bit_set(value, BV32::from(2));
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
            ensures self: Armv7m[{ general_regs: set_gpr(reg, old_cpu, val), ..old_cpu }] 
    )]
    fn update_general_reg_with_b32(&mut self, register: GPR, value: BV32) {
        self.general_regs.set(register, value);
    }

    #[flux_rs::sig(fn (&Armv7m[@cpu], &GPR[@reg]) -> BV32[get_gpr(reg, cpu)])]
    fn get_value_from_general_reg(&self, register: &GPR) -> BV32 {
        *self.general_regs.get(register).unwrap()
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu]) 
            requires sp_can_handle_exception_entry(cpu)
            ensures self: Armv7m[{ 
                sp: sp_post_exception_entry(cpu), 
                mem: mem_post_exception_entry(int(get_sp(sp_post_exception_entry(cpu), cpu.mode, cpu.control)), cpu),
                ..cpu 
            }] 
    )]
    fn push_stack(&mut self) {
        // Assuming 4 byte alignment for now 
        // but maybe this is something to revisit
        let frame_size = BV32::from(0x20);
        let frame_ptr = self.get_value_from_special_reg(&SpecialRegister::sp());
        let frame_ptr = (frame_ptr - frame_size) & !BV32::from(3);
        self.update_special_reg_with_b32(SpecialRegister::sp(), frame_ptr);
        let frame_ptr = frame_ptr.into();
         // MemA[frameptr,4] = R[0];
         // MemA[frameptr+0x4,4] = R[1];
         // MemA[frameptr+0x8,4] = R[2];
         // MemA[frameptr+0xC,4] = R[3];
         // MemA[frameptr+0x10,4] = R[12];
         // MemA[frameptr+0x14,4] = LR;
         // MemA[frameptr+0x18,4] = ReturnAddress(ExceptionType);
         // MemA[frameptr+0x1C,4] = (XPSR<31:10>:frameptralign:XPSR<8:0>);
        let r0 = self.get_value_from_general_reg(&GPR::r0());
        self.mem.write(frame_ptr, r0);
        let r1 = self.get_value_from_general_reg(&GPR::r1());
        self.mem.write(frame_ptr + 0x4, r1);
        let r2 = self.get_value_from_general_reg(&GPR::r2());
        self.mem.write(frame_ptr + 0x8, r2);
        let r3 = self.get_value_from_general_reg(&GPR::r3());
        self.mem.write(frame_ptr + 0xC, r3);
        let r12 = self.get_value_from_general_reg(&GPR::r12());
        self.mem.write(frame_ptr + 0x10, r12);
        let lr = self.get_value_from_special_reg(&SpecialRegister::lr());
        self.mem.write(frame_ptr + 0x14, lr);
        // putting a dummy value for ret addr
        self.mem.write(frame_ptr + 0x18, BV32::from(0));
        // TODO: Real implementation skips bit 9
        let psr = self.get_value_from_special_reg(&SpecialRegister::psr());
        self.mem.write(frame_ptr + 0x1C, psr);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], u8[@exception_num])
            ensures self: Armv7m[{ 
                mode: handler_mode(),
                control: control_post_exception_entry(old_cpu),
                psr: psr_post_exception_entry(old_cpu, exception_num),
                lr: lr_post_exception_entry(old_cpu, old_cpu.control),
                ..old_cpu
            }]
    )]
    fn exception_taken(&mut self, exception_number: u8) {
        // TODO: need to forget r0 - r3, r12 somehow 

        // set exception num in psr
        self.psr = (self.psr & !BV32::from(0xff)) |  BV32::from(exception_number as u32);

        // set link register
        self.lr = if self.mode_is_handler() {
            // From another exception
            BV32::from(0xFFFF_FFF1)
        } else if self.control.spsel {
            // from process stack
            BV32::from(0xFFFF_FFFD)
        } else {
            // from main stack
            BV32::from(0xFFFF_FFF9)
        };

        // stack = main
        self.mode = CPUMode::Handler;
        self.control.spsel = false;

        // TODO: There are other ops here but I don't think they 
        // matter 
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], u8[@exception_num]) 
            requires sp_can_handle_exception_entry(cpu)
            ensures self: Armv7m[{ 
                mode: handler_mode(),
                control: control_post_exception_entry(cpu),
                psr: psr_post_exception_entry(cpu, exception_num),
                lr: lr_post_exception_entry(cpu, cpu.control),
                sp: sp_post_exception_entry(cpu), 
                mem: mem_post_exception_entry(int(get_sp(sp_post_exception_entry(cpu), cpu.mode, cpu.control)), cpu),
                ..cpu
            }]
    )]
    fn exception_entry(&mut self, exception_number: u8) {
        self.push_stack();
        self.exception_taken(exception_number);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], BV32[@return_exec])
            requires return_exec == bv32(0xFFFF_FFF9) && return_exec == bv32(0xFFFF_FFFD)
            ensures self: Armv7m[{
                control: Control { spsel: return_exec == bv32(0xFFFF_FFF9), ..cpu.control },
                general_regs: gprs_post_exception_exit(get_sp_from_isr_ret(cpu.sp, return_exec), cpu),
                lr: get_mem_addr(get_sp_from_isr_ret(cpu.sp, return_exec) + 0x14, cpu.mem),
                psr: get_mem_addr(get_sp_from_isr_ret(cpu.sp, return_exec) + 0x1C, cpu.mem),
                ..cpu
            }]
    )]
    fn exception_exit(&mut self, return_exec: BV32) {
        let frame_ptr = if return_exec == BV32::from(0xFFFF_FFF9) {
                self.control.spsel = false;
                self.sp.sp_main.into()
        } else {
                self.control.spsel = true;
                self.sp.sp_process.into()
        };
        // R[0] = MemA[frameptr,4];
        // R[1] = MemA[frameptr+0x4,4];
        // R[2] = MemA[frameptr+0x8,4];
        // R[3] = MemA[frameptr+0xC,4];
        // R[12] = MemA[frameptr+0x10,4];
        // LR = MemA[frameptr+0x14,4];
        // BranchTo(MemA[frameptr+0x18,4]); // UNPREDICTABLE if the new PC not halfword aligned
        // psr = MemA[frameptr+0x1C,4];
        self.update_general_reg_with_b32(GPR::r0(), self.mem.read(frame_ptr));
        self.update_general_reg_with_b32(GPR::r1(), self.mem.read(frame_ptr + 0x4));
        self.update_general_reg_with_b32(GPR::r2(), self.mem.read(frame_ptr + 0x8));
        self.update_general_reg_with_b32(GPR::r3(), self.mem.read(frame_ptr + 0xC));
        self.update_general_reg_with_b32(GPR::r12(), self.mem.read(frame_ptr + 0x10));
        self.update_special_reg_with_b32(SpecialRegister::lr(), self.mem.read(frame_ptr + 0x14));
        self.update_special_reg_with_b32(SpecialRegister::psr(), self.mem.read(frame_ptr + 0x1C));
    }

    #[flux_rs::sig(
        fn (&mut Armv7m[@cpu], u8[@exception_num], _, _) 
            requires sp_can_handle_exception_entry(cpu)
    )]
    fn exception(&mut self, exception_number: u8, isr: fn(&mut Armv7m) -> BV32, ret_to: fn(&mut Armv7m) -> ()) {
        self.exception_entry(exception_number);
        let ret = isr(self);
        self.exception_exit(ret);
        // branch to return address
        ret_to(self)
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
