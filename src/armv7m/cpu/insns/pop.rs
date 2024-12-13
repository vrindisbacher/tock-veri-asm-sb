
use crate::{
    armv7m::{
        cpu::Armv7m,
        lang::{SpecialRegister, GPR},
    },
    flux_support::bv32::BV32,
};

impl Armv7m {
    // NOTE: these are split up because flux cannot reason about
    // contents of vectors, so pushes have to be done in sequence
    // rather than with a group of registers
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], GPR[@reg]) 
            requires is_valid_ram_addr(bv_add(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x4)))
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                    general_regs: set_gpr(reg, old_cpu, get_mem_addr(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), old_cpu.mem)),
                    sp: set_sp(old_cpu.sp, old_cpu.mode, old_cpu.control, bv_add(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x4))),
                    ..old_cpu
                }
            }
    )]
    pub fn pop_gpr(&mut self, reg: GPR) {
        let sp = self
            .get_value_from_special_reg(&SpecialRegister::sp());
        let val = self.mem.read(sp);
        self.update_general_reg_with_b32(reg, val);
        let sp = sp + BV32::from(0x4);
        self.update_special_reg_with_b32(SpecialRegister::sp(), BV32::from(sp));
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@reg]) 
            requires 
                is_valid_ram_addr(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control)) 
                && 
                !is_control(reg)
                &&
                !is_sp(reg)
                && 
                !is_psp(reg)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_pop_spr(old_cpu, reg) }
    )]
    pub fn pop_spr(&mut self, reg: SpecialRegister) {
        match reg {
            SpecialRegister::Sp |
            SpecialRegister::Control |
            SpecialRegister::PSP => return,
            _ => ()
        };
        let sp = self
            .get_value_from_special_reg(&SpecialRegister::sp());
        let new_sp = sp + BV32::from(0x4);
        let val = self.mem.read(sp);
        self.update_special_reg_with_b32(reg, val);
        self.update_special_reg_with_b32(SpecialRegister::sp(), new_sp);
    }

}
