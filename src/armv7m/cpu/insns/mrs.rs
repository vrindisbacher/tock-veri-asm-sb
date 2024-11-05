use crate::armv7m::lang::{SpecialRegister, GPR};

use super::super::Armv7m;

impl Armv7m {
    // Move to Register from Special register moves the value from the
    // selected special-purpose register into a general-purpose Arm register.
    // See p. A7-300 & p. B5-675 of the manual
    //
    // Pseudo Code provided by ARM:
    //  if ConditionPassed() then
    //      EncodingSpecificOperations();
    //      R[d] = Zeros(32);
    //      case SYSm<7:3> of
    //          when ‘00000’ /* xPSR accesses */
    //              if SYSm<0> == ‘1’ then
    //                  R[d]<8:0> = IPSR<8:0>;
    //              if SYSm<1> == ‘1’ then
    //                  R[d]<26:24> = ‘000’; /* EPSR reads as zero */
    //                  R[d]<15:10> = ‘000000’;
    //              if SYSm<2> == ‘0’ then
    //                  R[d]<31:27> = APSR<31:27>;
    //                  if HaveDSPExt() then
    //                      R[d]<19:16> = APSR<19:16>;
    //          when ‘00001’ /* SP access */
    //              if CurrentModeIsPrivileged() then
    //                  case SYSm<2:0> of
    //                      when ‘000’
    //                          R[d] = SP_main;
    //                      when ‘001’
    //                          R[d] = SP_process;
    //          when ‘00010’ /* Priority mask or CONTROL access */
    //                  case SYSm<2:0> of
    //                  when ‘000’
    //                      R[d]<0> = if CurrentModeIsPrivileged() then PRIMASK<0> else ‘0’;
    //                  when ‘001’
    //                      R[d]<7:0> = if CurrentModeIsPrivileged() then BASEPRI<7:0> else ‘00000000’;
    //                  when ‘010’
    //                      R[d]<7:0> = if CurrentModeIsPrivileged() then BASEPRI<7:0> else ‘00000000’;
    //                  when ‘011’
    //                      R[d]<0> = if CurrentModeIsPrivileged() then FAULTMASK<0> else ‘0’;
    //                  when ‘100’
    //                      if HaveFPExt() then
    //                          R[d]<2:0> = CONTROL<2:0>;
    //                      else
    //                          R[d]<1:0> = CONTROL<1:0>;
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GPR[@reg], SpecialRegister[@val]) 
        ensures self: Armv7m { 
            new_cpu: 
                gpr_set(reg, old_cpu, new_cpu, get_special_reg(val, old_cpu)) 
                &&
                old_cpu.special_regs == new_cpu.special_regs
                &&
                old_cpu.mem == new_cpu.mem
        }
    )]
    pub fn mrs(&mut self, register: GPR, value: SpecialRegister) {
        // VTOCK TODO: monster op

        // for now just move the value
        let value = self.get_value_from_special_reg(&value);
        self.update_general_reg_with_b32(register, value);
    }
}
