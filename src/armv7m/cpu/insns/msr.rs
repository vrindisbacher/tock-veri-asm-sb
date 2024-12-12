use crate::armv7m::lang::{SpecialRegister, GPR};

use super::super::Armv7m;

impl Armv7m {
    // Move to Special Register from Arm Register moves the value of a
    // general-purpose Arm register to the specified special-purpose register.
    // See p. A7-301 & p. B5-677 of the manual
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@reg], GPR[@val])
            requires !is_ipsr(reg) && is_sp(reg) => is_valid_ram_addr(get_gpr(val, old_cpu))
            ensures self: Armv7m { new_cpu: new_cpu == set_spr(reg, old_cpu, get_gpr(val, old_cpu)) }
    )]
    pub fn msr(&mut self, register: SpecialRegister, value: GPR) {
        self.update_special_reg_with_b32(register, self.get_value_from_general_reg(&value));
    }
}
