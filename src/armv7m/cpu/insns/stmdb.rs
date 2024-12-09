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
            requires is_valid_write_addr(int(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control)) - 0x4)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_stmdb_no_wback(old_cpu, rd, rm) }
    )]
    pub fn stmdb_no_wback(&mut self, rd: SpecialRegister, rm: GPR) {
        // this is basically identical to push because there is no write back
        // only difference is that it doesn't update sp directly
        let addr: u32 = self.get_value_from_special_reg(&rd).into();
        let addr = addr - 0x4;
        let val = self.get_value_from_general_reg(&rm);
        self.mem.write(addr, val);
        self.update_special_reg_with_b32(rd, BV32::from(addr));
    }
}
