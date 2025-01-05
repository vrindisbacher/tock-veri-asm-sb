use crate::armv7m::{
    cpu::Armv7m,
    lang::{SpecialRegister, GPR},
};

use flux_rs::bitvec::BV32;
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
            is_valid_read_addr(get_gpr(rd, old_cpu))
            &&
            is_valid_read_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x4)))
            &&
            is_valid_read_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x8)))
            &&
            is_valid_read_addr(bv_add(get_gpr(rd, old_cpu), bv32(0xc)))
            &&
            is_valid_read_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x10)))
            &&
            is_valid_read_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x14)))
            &&
            is_valid_read_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x18)))
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
        let mut addr = self.get_value_from_general_reg(&rd);
        let val = self.mem.read(addr);
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

    #[flux_rs::sig(
        fn (
            self: &strg Armv7m[@old_cpu],
            SpecialRegister[@rd],
            GPR[@rm1],
            GPR[@rm2],
            GPR[@rm3],
        )
        requires
            is_valid_read_addr(get_special_reg(rd, old_cpu))
            &&
            is_valid_read_addr(bv_add(get_special_reg(rd, old_cpu), bv32(0x4)))
            &&
            is_valid_read_addr(bv_add(get_special_reg(rd, old_cpu), bv32(0x8)))
            &&
            is_valid_read_addr(bv_add(get_special_reg(rd, old_cpu), bv32(0xc)))
        ensures self: Armv7m { new_cpu: new_cpu == set_spr(
                rd,
                Armv7m {
                    general_regs: gprs_post_ldmia_w_special(old_cpu, rd, rm1, rm2, rm3),
                    ..old_cpu
                },
                bv_add(get_special_reg(rd, old_cpu), bv32(0xc))
            )
        }

    )]
    pub fn ldmia_w_special(&mut self, rd: SpecialRegister, rm1: GPR, rm2: GPR, rm3: GPR) {
        // NOTE: This is variant ldmia.w rd! { ... } so updates to rd are written back to the
        // register
        //
        // lowest memory values are written to lowest registers but we can cheat a bit here
        // because our use case passes regs in order
        let (val1, val2, val3) = self.ldmia_w_special_get_vals(rd);
        self.ldmia_w_special_update_gprs(rm1, val1, rm2, val2, rm3, val3);
    }

    #[flux_rs::sig(
        fn (
            self: &strg Armv7m[@old_cpu],
            SpecialRegister[@rd],
        ) -> (
            BV32[get_mem_addr(get_special_reg(rd, old_cpu), old_cpu.mem)],
            BV32[get_mem_addr(bv_add(get_special_reg(rd, old_cpu), bv32(0x4)), old_cpu.mem)],
            BV32[get_mem_addr(bv_add(get_special_reg(rd, old_cpu), bv32(0x8)), old_cpu.mem)],
        )
        requires
            is_valid_read_addr(get_special_reg(rd, old_cpu))
            &&
            is_valid_read_addr(bv_add(get_special_reg(rd, old_cpu), bv32(0x4)))
            &&
            is_valid_read_addr(bv_add(get_special_reg(rd, old_cpu), bv32(0x8)))
            &&
            is_valid_read_addr(bv_add(get_special_reg(rd, old_cpu), bv32(0xc)))
        ensures self: Armv7m { new_cpu: new_cpu == set_spr(rd, old_cpu, bv_add(get_special_reg(rd, old_cpu), bv32(0xc))) }
    )]
    fn ldmia_w_special_get_vals(&mut self, rd: SpecialRegister) -> (BV32, BV32, BV32) {
        let mut addr = self.get_value_from_special_reg(&rd);
        let val1 = self.mem.read(addr);
        addr = addr + BV32::from(0x4);
        let val2 = self.mem.read(addr);
        addr = addr + BV32::from(0x4);
        let val3 = self.mem.read(addr);
        addr = addr + BV32::from(0x4);
        self.update_special_reg_with_b32(rd, addr);
        (val1, val2, val3)
    }

    #[flux_rs::sig(fn
        (
            self: &strg Armv7m[@cpu],
            GPR[@rm1],
            BV32[@val1],
            GPR[@rm2],
            BV32[@val2],
            GPR[@rm3],
            BV32[@val3]
        )
        ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
            general_regs:  map_set(
                map_set(
                    map_set(
                        cpu.general_regs,
                        rm1,
                        val1
                    ),
                    rm2,
                    val2
                ),
                rm3,
                val3
            ),
            ..cpu
        } }
    )]
    fn ldmia_w_special_update_gprs(
        &mut self,
        rm1: GPR,
        val1: BV32,
        rm2: GPR,
        val2: BV32,
        rm3: GPR,
        val3: BV32,
    ) {
        self.update_general_reg_with_b32(rm1, val1);
        self.update_general_reg_with_b32(rm2, val2);
        self.update_general_reg_with_b32(rm3, val3);
    }
}
