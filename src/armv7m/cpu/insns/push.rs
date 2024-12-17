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
            requires is_valid_ram_addr(bv_sub(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x4)))
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                    mem: update_mem(bv_sub(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x4)), old_cpu.mem, get_gpr(reg, old_cpu)),
                    sp: set_sp(old_cpu.sp, old_cpu.mode, old_cpu.control, bv_sub(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x4))),
                    ..old_cpu
                }
            }
    )]
    pub fn push_gpr(&mut self, reg: GPR) {
        let sp = self
            .get_value_from_special_reg(&SpecialRegister::sp()) - BV32::from(0x4);
        let val = self.get_value_from_general_reg(&reg);
        self.mem.write(sp, val);
        self.update_special_reg_with_b32(SpecialRegister::sp(), BV32::from(sp));
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@reg]) 
            requires is_valid_ram_addr(bv_sub(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x4)))
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                    mem: update_mem(bv_sub(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x4)), old_cpu.mem, get_special_reg(reg, old_cpu)),
                    sp: set_sp(old_cpu.sp, old_cpu.mode, old_cpu.control, bv_sub(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x4))),
                    ..old_cpu
                }
            }
    )]
    pub fn push_spr(&mut self, reg: SpecialRegister) {
        // address = SP - 4*BitCount(registers);
        let sp = self
            .get_value_from_special_reg(&SpecialRegister::sp()) - BV32::from(0x4);
        let val = self.get_value_from_special_reg(&reg);
        self.mem.write(sp, val);
        self.update_special_reg_with_b32(SpecialRegister::sp(), BV32::from(sp));
    }
}
