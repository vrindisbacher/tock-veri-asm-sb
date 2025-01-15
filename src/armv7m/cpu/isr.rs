use crate::armv7m::lang::{IsbOpt, SpecialRegister, GPR};
use flux_rs::bitvec::BV32;

use super::{Armv7m, Control};

flux_rs::defs! {}

impl Armv7m {
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) -> BV32[bv32(0xFFFF_FFF9)]
        requires
            bv_uge(get_special_reg(ipsr(), old_cpu), bv32(16))
            &&
            mode_is_handler(old_cpu.mode)
        ensures self: Armv7m { new_cpu: new_cpu == cpu_post_generic_isr(old_cpu) }
    )]
    pub fn generic_isr(&mut self) -> BV32 {
        // r0 = 0
        self.movw_imm(GPR::R0, BV32::from(0));
        // control = r0 = 0
        self.msr(SpecialRegister::Control, GPR::R0);
        // isb
        self.isb(Some(IsbOpt::Sys));
        // NOTE: using pseudo instr here
        // lr = 0xFFFFFFF9
        self.pseudo_ldr_special(SpecialRegister::lr(), BV32::from(0xFFFFFFF9));
        // r0 = ipsr
        self.mrs(GPR::R0, SpecialRegister::IPSR);
        // Note: this seems to be a useless instruction?
        self.and_imm(GPR::R0, BV32::from(0xff));
        // r0 = ipsr - 16
        self.subw_imm(GPR::R0, GPR::R0, BV32::from(16));
        // r2 = r0 >> 5 ---> (ipsr - 16 / 32)
        self.lsrs_imm(GPR::R2, GPR::R0, BV32::from(5));
        // r3 = 1
        self.movs_imm(GPR::R3, BV32::from(1));
        // r0 = r0 & 31
        self.and_imm(GPR::R0, BV32::from(31));
        // r0 = r3 << r0
        //      -     -
        //      1     (ipsr - 16 & 31)
        self.lslw_reg(GPR::R0, GPR::R3, GPR::R0);
        // Note: Ignoring the dissasembled version of this because dealing with program counter is
        // annoying
        //
        // Gonna encode this as a pseudo instruction for now
        self.pseudo_ldr(GPR::R3, BV32::from(0xe000_e180));
        // r0 = 1 << (ipsr - 16 & 31)
        // r3 = 0xe000_e180
        // r2 = (ipsr - 16 >> 5)
        self.strw_lsl_reg(GPR::R0, GPR::R3, GPR::R2, BV32::from(2));
        // Note: Ignoring the dissasembled version of this because dealing with program counter is
        // annoying
        //
        // Gonna encode this as a pseudo instruction for now
        self.pseudo_ldr(GPR::R3, BV32::from(0xe000_e200));
        // r0 = 1 << (ipsr - 16 & 31)
        // r3 = 0xe000_e200
        // r2 = (ipsr - 16 >> 5) << 2
        //
        // mem[0xe000_e200 + ((ipsr - 16 >> 5) << 2)] = (1 << ipsr - 16 & 31) i.e. "bit for the ipsr # is set"
        self.strw_lsl_reg(GPR::R0, GPR::R3, GPR::R2, BV32::from(2));
        // self.bx(SpecialRegister::Lr);
        return self.get_value_from_special_reg(&SpecialRegister::lr());
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) -> BV32[bv32(0xFFFF_FFF9)]
            requires mode_is_handler(old_cpu.mode)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_svc_to_kernel_isr(old_cpu) }
    )]
    fn svc_isr_to_kernel(&mut self) -> BV32 {
        // sys call fired is a pub static mut so it has some location -
        // giving it an arbitrary ram addr
        //
        // basically we just want to set SYSCALL FIRED pub static mut to 1
        self.pseudo_ldr(GPR::R0, BV32::from(0x8000_0000));
        self.movw_imm(GPR::R1, BV32::from(1));
        self.str_no_wback(GPR::R1, GPR::R0);
        // now do everything else
        self.movw_imm(GPR::R0, BV32::from(0));
        self.msr(SpecialRegister::Control, GPR::R0);
        self.isb(Some(IsbOpt::Sys));
        self.pseudo_ldr_special(SpecialRegister::lr(), BV32::from(0xFFFF_FFF9));
        // self.bx(SpecialRegister::Lr);
        return self.get_value_from_special_reg(&SpecialRegister::lr());
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) -> BV32[bv32(0xFFFF_FFFD)]
            requires mode_is_handler(old_cpu.mode)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_svc_to_app_isr(old_cpu) }
    )]
    fn svc_isr_to_app(&mut self) -> BV32 {
        self.movw_imm(GPR::R0, BV32::from(1));
        self.msr(SpecialRegister::Control, GPR::R0);
        self.isb(Some(IsbOpt::Sys));
        self.pseudo_ldr_special(SpecialRegister::lr(), BV32::from(0xFFFF_FFFD));
        return self.get_value_from_special_reg(&SpecialRegister::lr());
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) -> BV32[svc_isr_ret_val(old_cpu)]
            requires mode_is_handler(old_cpu.mode)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_svc_isr(old_cpu) }
    )]
    pub fn svc_isr(&mut self) -> BV32 {
        // TODO: should really be a cmp & bne but tough to model that so using ite for now
        if self.get_value_from_special_reg(&SpecialRegister::lr()) == BV32::from(0xFFFF_FFF9) {
            return self.svc_isr_to_app();
        } else {
            return self.svc_isr_to_kernel();
        }
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) -> BV32[bv32(0xFFFF_FFF9)]
            requires mode_is_handler(old_cpu.mode)
            ensures self: Armv7m { new_cpu: new_cpu ==  cpu_post_sys_tick_isr(old_cpu) }
    )]
    pub fn sys_tick_isr(&mut self) -> BV32 {
        self.movw_imm(GPR::R0, BV32::from(0));
        self.msr(SpecialRegister::Control, GPR::R0);
        self.isb(Some(IsbOpt::Sys));
        self.pseudo_ldr_special(SpecialRegister::lr(), BV32::from(0xFFFF_FFF9));
        // self.bx(SpecialRegister::Lr);
        return self.get_value_from_special_reg(&SpecialRegister::lr());
    }
}
