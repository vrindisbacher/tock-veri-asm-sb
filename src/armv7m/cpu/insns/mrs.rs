use crate::armv7m::lang::{SpecialRegister, GPR};

use super::super::Armv7m;

impl Armv7m {
    // Move to Register from Special register moves the value from the
    // selected special-purpose register into a general-purpose Arm register.
    // See p. A7-300 & p. B5-675 of the manual
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GPR[@reg], SpecialRegister[@val]) 
        ensures self: Armv7m[{ general_regs: set_gpr(reg, old_cpu, get_special_reg(val, old_cpu)), ..old_cpu }] 
    )]
    pub fn mrs(&mut self, register: GPR, value: SpecialRegister) {
        let value = self.get_value_from_special_reg(&value);
        self.update_general_reg_with_b32(register, value);
    }
}
