use crate::armv7m::{
    cpu::Armv7m,
    lang::{SpecialRegister, GPR},
};
use flux_rs::bitvec::BV32;

flux_rs::defs! {}

impl Armv7m {
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@rd], GPR[@r1], GPR[@r2], GPR[@r3])
            requires
                is_valid_ram_addr(get_special_reg(rd, old_cpu))
                &&
                is_valid_ram_addr(bv_sub(get_special_reg(rd, old_cpu), bv32(0xc)))
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_stmdb_wback(old_cpu, rd, r1, r2, r3) }

    )]
    pub fn stmdb_wback(&mut self, rd: SpecialRegister, r1: GPR, r2: GPR, r3: GPR) {
        // this is identical to push - especially in this case because it is only
        // used with sp!
        let mut addr = self.get_value_from_special_reg(&rd) - BV32::from(0xc);

        // note - goes in order from lowest register but we can cheat a bit because we know
        // these are passed in order in our case
        let val1 = self.get_value_from_general_reg(&r1);
        self.mem.write(addr, val1);
        addr = addr + BV32::from(0x4);

        let val2 = self.get_value_from_general_reg(&r2);
        self.mem.write(addr, val2);
        addr = addr + BV32::from(0x4);

        let val3 = self.get_value_from_general_reg(&r3);
        self.mem.write(addr, val3);
        addr = addr + BV32::from(0x4);

        self.update_special_reg_with_b32(rd, addr - BV32::from(0xc));
    }
}
