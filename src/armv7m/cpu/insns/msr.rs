use crate::armv7m::lang::{GeneralPurposeRegister, SpecialRegister};

use super::super::Armv7m;

impl Armv7m {
    // Move to Special Register from Arm Register moves the value of a
    // general-purpose Arm register to the specified special-purpose register.
    // See p. A7-301 & p. B5-677 of the manual
    //
    //
    // Pseudo code provided by ARM
    // if ConditionPassed() then
    //  EncodingSpecificOperations();
    //  case SYSm<7:3> of
    //      when ‘00000’ /* xPSR accesses */
    //       if SYSm<2> == ‘0’ then /* Include APSR */
    //          if mask<0> == ‘1’ then /* GE[3:0] bits */
    //              if !HaveDSPExt() then
    //               UNPREDICTABLE;
    //              else
    //               APSR<19:16> = R[n]<19:16>;
    //       if mask<1> == ‘1’ then /* N, Z, C, V, Q bits */
    //         APSR<31:27> = R[n]<31:27>;
    //      when ‘00001’ /* SP access */
    //        if CurrentModeIsPrivileged() then
    //          case SYSm<2:0> of
    //              when ‘000’
    //                  SP_main = R[n];
    //              when ‘001’
    //                  SP_process = R[n];
    //      when ‘00010’ /* Priority mask or CONTROL access */
    //          case SYSm<2:0> of
    //              when ‘000’
    //                  if CurrentModeIsPrivileged() then PRIMASK<0> = R[n]<0>;
    //              when ‘001’
    //                  if CurrentModeIsPrivileged() then BASEPRI<7:0> = R[n]<7:0>;
    //              when ‘010’
    //                  if CurrentModeIsPrivileged() &&
    //                  (R[n]<7:0> != ‘00000000’) &&
    //                  (UInt(R[n]<7:0>) < UInt(BASEPRI<7:0>) || BASEPRI<7:0> == ‘00000000’) then
    //                      BASEPRI<7:0> = R[n]<7:0>;
    //              when ‘011’
    //                  if CurrentModeIsPrivileged() &&
    //                      (ExecutionPriority() > -1) then
    //                      FAULTMASK<0> = R[n]<0>;
    //              when ‘100’
    //                  if CurrentModeIsPrivileged() then
    //                      CONTROL.nPRIV = R[n]<0>;
    //                  if CurrentMode == Mode_Thread then
    //                      CONTROL.SPSEL = R[n]<1>;
    //                  if HaveFPExt() then CONTROL.FPCA = R[n]<2>;
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], SpecialRegister[@reg], GeneralPurposeRegister[@val])
            // only updates to CONTROL right now
            requires is_control(reg)
            ensures self: Armv7m {
                new_cpu: 
                    special_purpose_register_updated(reg, old_cpu, new_cpu, get_general_purpose_reg(val, old_cpu))
                    &&
                    old_cpu.general_regs == new_cpu.general_regs
                    &&
                    old_cpu.mem == new_cpu.mem
            }
    )]
    pub fn msr(&mut self, register: SpecialRegister, value: GeneralPurposeRegister) {
        // This is a monster op
        self.update_special_reg_with_u32(register, self.get_value_from_general_reg(&value));
    }
}
