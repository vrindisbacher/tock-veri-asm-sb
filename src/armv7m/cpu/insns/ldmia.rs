use crate::{armv7m::{cpu::Armv7m, lang::{SpecialRegister, GPR}}, flux_support::bv32::BV32};


impl Armv7m {
    #[flux_rs::sig(
        fn (
            self: &strg Armv7m[@old_cpu],
            GPR[@rd],
            GPR[@rm1],
            GPR[@rm2],
            GPR[@rm3],
            GPR[@rm4],
            GPR[@rm5],
            GPR[@rm6],
            GPR[@rm7],
            GPR[@rm8],
        ) 
        requires 
            is_valid_ram_addr(get_gpr(rd, old_cpu))
            &&
            is_valid_ram_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x1c)))
        ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                general_regs: gprs_post_ldmia_w(old_cpu, rd, rm1, rm2, rm3, rm4, rm5, rm6, rm7, rm8),
                ..old_cpu
            }
        }

    )]
    pub fn ldmia_w(
        &mut self,
        rd: GPR,
        rm1: GPR,
        rm2: GPR,
        rm3: GPR,
        rm4: GPR,
        rm5: GPR,
        rm6: GPR,
        rm7: GPR,
        rm8: GPR,
    ) {
        let mut addr = self.get_value_from_general_reg(&rd); let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm1, val);
        addr = addr + BV32::from(0x4);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm2, val);
        addr = addr + BV32::from(0x4);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm3, val);
        addr = addr + BV32::from(0x4);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm4, val);
        addr = addr + BV32::from(0x4);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm5, val);
        addr = addr + BV32::from(0x4);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm6, val);
        addr = addr + BV32::from(0x4);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm7, val);
        addr = addr + BV32::from(0x4);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm8, val);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (
            self: &strg Armv7m[@old_cpu],
            SpecialRegister[@rd],
            GPR[@rm1],
            GPR[@rm2],
            GPR[@rm3],
        ) 
        requires 
            is_valid_ram_addr(get_special_reg(rd, old_cpu))
            &&
            is_valid_ram_addr(bv_add(get_special_reg(rd, old_cpu), bv32(0x8)))
        ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                general_regs: gprs_post_ldmia_w_special(old_cpu, rd, rm1, rm2, rm3),
                ..old_cpu
            }
        }

    )]
    pub fn ldmia_w_special(
        &mut self,
        rd: SpecialRegister,
        rm1: GPR,
        rm2: GPR,
        rm3: GPR,
    ) {
        let mut addr = self.get_value_from_special_reg(&rd);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm1, val);
        addr = addr + BV32::from(0x4);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm2, val);
        addr = addr + BV32::from(0x4);
        let val = self.mem.read(addr);
        self.update_general_reg_with_b32(rm3, val);
    }
}
