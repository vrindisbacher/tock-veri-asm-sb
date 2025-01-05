use crate::armv7m::{
    cpu::Armv7m,
    lang::{SpecialRegister, GPR},
};
use flux_rs::bitvec::BV32;

impl Armv7m {
    pub fn add_imm(&mut self, rd: GPR, rn: SpecialRegister, imm: BV32) {
        // adds rn and imm and stores the result in rd
        // VTOCK TODO: Flag Updates
        self.update_general_reg_with_b32(rd, self.get_value_from_special_reg(&rn) + imm);
    }
}
