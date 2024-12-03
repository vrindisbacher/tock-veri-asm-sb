use crate::{
    armv7m::{
        cpu::Armv7m,
        lang::{SpecialRegister, GPR},
    },
    flux_support::bv32::BV32,
};

impl Armv7m {
    pub fn stmdb_no_wback(&mut self, rd: SpecialRegister, rm: GPR) {
        // this is basically identical to push because there is no write back
        // only difference is that it doesn't update sp directly
        let addr: u32 = self.get_value_from_special_reg(&rd).into();
        let addr = addr - 0x4;
        self.mem.write(addr, self.get_value_from_general_reg(&rm));
        self.update_special_reg_with_b32(rd, BV32::from(addr));
    }
}
