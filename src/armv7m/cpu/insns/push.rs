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
    pub fn push_gpr(&mut self, reg: GPR) {
        let sp: u32 = self
            .get_value_from_special_reg(&SpecialRegister::sp())
            .into();
        let sp = sp - 0x4;
        let val = self.get_value_from_general_reg(&reg);
        self.mem.write(sp, val);
        self.update_special_reg_with_b32(SpecialRegister::sp(), BV32::from(sp));
    }

    pub fn push_spr(&mut self, reg: SpecialRegister) {
        // address = SP - 4*BitCount(registers);
        let sp: u32 = self
            .get_value_from_special_reg(&SpecialRegister::sp())
            .into();
        let sp = sp - 0x4;
        let val = self.get_value_from_special_reg(&reg);
        self.mem.write(sp, val);
        self.update_special_reg_with_b32(SpecialRegister::sp(), BV32::from(sp));
    }
}
