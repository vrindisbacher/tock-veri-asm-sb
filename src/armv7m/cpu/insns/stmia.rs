use crate::armv7m::{
    cpu::Armv7m,
    lang::{SpecialRegister, GPR},
};
use flux_rs::bitvec::BV32;

impl Armv7m {
    // flattening the list into 8 regs because we know this is r4 - r11
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
            is_valid_write_addr(get_gpr(rd, old_cpu))
            &&
            is_valid_write_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x4)))
            &&
            is_valid_write_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x8)))
            &&
            is_valid_write_addr(bv_add(get_gpr(rd, old_cpu), bv32(0xc)))
            &&
            is_valid_write_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x10)))
            &&
            is_valid_write_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x14)))
            &&
            is_valid_write_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x18)))
            &&
            is_valid_write_addr(bv_add(get_gpr(rd, old_cpu), bv32(0x1c)))
        ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                mem: mem_post_stmia_w(old_cpu, rd, rm1, rm2, rm3, rm4, rm5, rm6, rm7, rm8),
                ..old_cpu
            }
        }
    )]
    pub fn stmia_w(
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
        let val = self.get_value_from_general_reg(&rm1);
        self.mem.write(addr, val);
        addr = addr + BV32::from(0x4);
        let val = self.get_value_from_general_reg(&rm2);
        self.mem.write(addr, val);
        addr = addr + BV32::from(0x4);
        let val = self.get_value_from_general_reg(&rm3);
        self.mem.write(addr, val);
        addr = addr + BV32::from(0x4);
        let val = self.get_value_from_general_reg(&rm4);
        self.mem.write(addr, val);
        addr = addr + BV32::from(0x4);
        let val = self.get_value_from_general_reg(&rm5);
        self.mem.write(addr, val);
        addr = addr + BV32::from(0x4);
        let val = self.get_value_from_general_reg(&rm6);
        self.mem.write(addr, val);
        addr = addr + BV32::from(0x4);
        let val = self.get_value_from_general_reg(&rm7);
        self.mem.write(addr, val);
        addr = addr + BV32::from(0x4);
        let val = self.get_value_from_general_reg(&rm8);
        self.mem.write(addr, val);
    }
}
