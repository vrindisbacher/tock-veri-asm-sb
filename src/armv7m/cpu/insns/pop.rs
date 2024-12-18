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
        let sp = self.get_value_from_special_reg(&SpecialRegister::sp());
        let val = self.mem.read(sp);
        let sp = sp + BV32::from(0x4);
        self.update_general_reg_with_b32(reg, val);
        self.update_special_reg_with_b32(SpecialRegister::sp(), sp);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu]) -> BV32[pop_spr_get_mem_addr_and_incr_ret_val(cpu)]
            requires pop_spr_get_mem_addr_and_incr_precondition(cpu)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_pop_spr_get_mem_addr_and_incr(cpu) }
    )]
    fn pop_spr_get_mem_addr_and_incr(&mut self) -> BV32 {
        let sp = self.get_value_from_special_reg(&SpecialRegister::sp());
        self.update_special_reg_with_b32(SpecialRegister::sp(), sp + BV32::from(0x4));
        self.mem.read(sp)
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], SpecialRegister[@reg], BV32[@val]) 
            requires pop_spr_update_reg_precondition(cpu, reg, val)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_pop_spr_update_reg(cpu, reg, val) }
    )]
    fn pop_spr_update_reg(&mut self, reg: SpecialRegister, val: BV32) {
        self.update_special_reg_with_b32(reg, val);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@reg]) 
            requires pop_spr_precondition(old_cpu, reg)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_pop_spr(old_cpu, reg) }
    )]
    pub fn pop_spr(&mut self, reg: SpecialRegister) {
        let val = self.pop_spr_get_mem_addr_and_incr();
        self.pop_spr_update_reg(reg, val);
    }
}
