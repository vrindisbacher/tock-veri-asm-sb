use crate::{
    armv7m::{
        cpu::Armv7m,
        lang::{SpecialRegister, GPR},
    },
    flux_support::bv32::BV32,
};

impl Armv7m {
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@rd], GPR[@rm]) 
            requires is_valid_ram_addr(bv_sub(get_special_reg(rd, old_cpu), bv32(0x4)))
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_stmdb_no_wback(old_cpu, rd, rm) }
    )]
    pub fn stmdb_no_wback(&mut self, rd: SpecialRegister, rm: GPR) {
        // this is basically identical to push because there is no write back
        // only difference is that it doesn't update sp directly
        let addr = self.get_value_from_special_reg(&rd) - BV32::from(0x4);
        self.update_special_reg_with_b32(rd, addr);
        let val = self.get_value_from_general_reg(&rm);
        self.mem.write(addr, val);
    }
}
