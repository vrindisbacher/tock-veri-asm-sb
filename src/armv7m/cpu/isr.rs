use crate::{armv7m::lang::{IsbOpt, SpecialRegister, GPR}, flux_support::bv32::BV32};

use super::{Armv7m, Control};

flux_rs::defs! {

    fn isr_bit_loc(old_cpu: Armv7m) -> BV32 {
        bv32((to_int(get_special_reg(ipsr(), old_cpu)) - 16) % 32)
    }

    fn isr_r0(old_cpu: Armv7m) -> BV32 {
        left_shift(
            bv32(1),
            isr_bit_loc(old_cpu)
        )
    }

    fn isr_r2(old_cpu: Armv7m) -> BV32 {
        bv32((to_int(get_special_reg(ipsr(), old_cpu)) - 16) / 32)
    }

    fn isr_offset(old_cpu: Armv7m) -> int {
        to_int(isr_r2(old_cpu)) * 4
    }
}

impl Armv7m {


    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) 
        requires to_int(get_special_reg(ipsr(), old_cpu)) >= 16 && mode_is_handler(old_cpu.mode)
        ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                mem: update_mem(
                     0xe000_e200 + isr_offset(old_cpu),
                     update_mem(
                         0xe000_e180 + isr_offset(old_cpu),
                         old_cpu.mem,
                         isr_r0(old_cpu)
                    ),
                    isr_r0(old_cpu)
                ),
                general_regs: map_set(
                    map_set(
                        map_set(
                            old_cpu.general_regs,
                            r0(),
                            isr_r0(old_cpu)
                        ),
                        r2(),
                        isr_r2(old_cpu)
                    ),
                    r3(),
                    bv32(0xe000_e200)
                ),
                control: Control { npriv: false, ..old_cpu.control },
                lr: bv32(0xFFFF_FFF9),
                ..old_cpu
            }
        }
    )]
    pub fn generic_isr(&mut self) {
        // r0 = 0
        self.movw_imm(GPR::R0, BV32::from(0));
        // control = r0 = 0
        self.msr(SpecialRegister::Control, GPR::R0);
        // isb
        self.isb(Some(IsbOpt::Sys));
        // NOTE: using pseudo instr here
        // lr = 0xFFFFFFF9
        self.pseudo_ldr_special(SpecialRegister::Lr, BV32::from(0xFFFFFFF9));
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
        self.bx(SpecialRegister::Lr);
    }

    fn svc_isr(&mut self) {}
    fn sys_tick_isr(&mut self) {}
}
