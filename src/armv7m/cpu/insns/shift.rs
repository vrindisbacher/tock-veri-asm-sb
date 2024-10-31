use crate::{armv7m::lang::GeneralPurposeRegister, flux_support::b32::B32};

use super::{
    super::Armv7m,
    utils::{get_nth_bit, shift_left, shift_right},
};

impl Armv7m {
    // LSR Immediate (see p. A7-284 of the manual)
    //
    // Pseudo code provided by arm:
    // if ConditionPassed() then
    //  EncodingSpecificOperations();
    //  (result, carry) = Shift_C(R[m], SRType_LSR, shift_n, APSR.C);
    //  R[d] = result;
    //  if setflags then
    //      APSR.N = result<31>;
    //      APSR.Z = IsZeroBit(result);
    //      APSR.C = carry;
    //      // APSR.V unchanged

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], GeneralPurposeRegister[@reg_val], B32[@shift]) 
        ensures self: Armv7m { 
            new_cpu: 
                general_purpose_register_updated(reg, old_cpu, new_cpu, right_shift(get_general_purpose_reg(reg_val, old_cpu), shift))
                &&
                old_cpu.special_regs == new_cpu.special_regs
                &&
                old_cpu.mem == new_cpu.mem
                // shift != 0 => general_purpose_register_updated(reg, new_cpu, right_shift_immediate_computation(reg_val, old_cpu, shift)) && lsrs_imm_flag_updates(reg_val, old_cpu, new_cpu, shift)
        }
    )]
    pub fn lsrs_imm(
        &mut self,
        register: GeneralPurposeRegister,
        value: GeneralPurposeRegister,
        shift: B32,
    ) {
        // Corresponds to encoding T1 of LSR
        //
        // Specific encoding ops are:
        //      d = UInt(Rdn); n = UInt(Rdn); m = UInt(Rm); setflags = !InITBlock();
        //
        //  We already know d & n (registers above)
        // if shift == 0 {
        //     return;
        // }
        // let value1 = self.get_value_from_general_reg(&value);
        // let res = shift_right(value1, shift);
        // let (res, carry) = if value1 > 0 && res == value1 {
        //     (0, get_nth_bit(value1, 31))
        // } else {
        //     (
        //         res,
        //         match shift {
        //             1..32 => get_nth_bit(value1, shift - 1),
        //             _ => 0,
        //         },
        //     )
        // };
        // self.update_general_reg_with_B32(register, res);
        // let set_flags = !self.in_if_then_block();
        // if set_flags {
        //     // VTOCK TODO: Actually deal with negative values
        //     self.unset_n_flag();
        //     self.set_z_flag();
        //     if carry == 1 {
        //         self.set_c_flag();
        //     } else {
        //         self.unset_c_flag();
        //     }
        // }
        let value1 = self.get_value_from_general_reg(&value);
        let res = value1 >> shift;
        self.update_general_reg_with_b32(register, res);
    }

    // LSL Register (see p. A7-283 of the manual)
    //
    // Pseudo code provided by arm:
    // if ConditionPassed() then
    //  EncodingSpecificOperations();
    //  (result, carry) = Shift_C(R[m], SRType_LSL, shift_n, APSR.C);
    //  R[d] = result;
    //  if setflags then
    //      APSR.N = result<31>;
    //      APSR.Z = IsZeroBit(result);
    //      APSR.C = carry;
    //      // APSR.V unchanged

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], GeneralPurposeRegister[@reg_val], GeneralPurposeRegister[@shift]) 
        ensures self: Armv7m { 
            new_cpu: 
                general_purpose_register_updated(reg, old_cpu, new_cpu, left_shift(get_general_purpose_reg(reg_val, old_cpu), get_general_purpose_reg(shift, old_cpu)))
                &&
                old_cpu.special_regs == new_cpu.special_regs
                &&
                old_cpu.mem == new_cpu.mem
                // get_general_purpose_reg(shift, old_cpu) != 0 
                //   => general_purpose_register_updated(reg, new_cpu, left_shift_reg_computation(reg_val, old_cpu, get_general_purpose_reg(shift, old_cpu))) && lslw_reg_flag_updates(reg_val, old_cpu, new_cpu, get_general_purpose_reg(shift, old_cpu))
        }
    )]
    pub fn lslw_reg(
        &mut self,
        register: GeneralPurposeRegister,
        value: GeneralPurposeRegister,
        shift: GeneralPurposeRegister,
    ) {
        // Corresponds to encoding T2 of LSL
        //
        // Specific encoding ops are:
        //
        // d = UInt(Rd); n = UInt(Rn); m = UInt(Rm); setflags = (S == '1');
        // if d IN {13,15} || n IN {13,15} || m IN {13,15} then UNPREDICTABLE;
        //
        //  We already know d & n (registers above)
        // let shift = self.get_value_from_general_reg(&shift);
        // if shift == 0 {
        //     return;
        // }
        // let value1 = self.get_value_from_general_reg(&value);
        // let res = shift_left(value1, shift);
        // let (res, carry) = if value1 > 0 && res == value1 {
        //     (0, get_nth_bit(value1, 31))
        // } else {
        //     (
        //         res,
        //         match shift {
        //             1..32 => get_nth_bit(value1, shift - 1),
        //             _ => 0,
        //         },
        //     )
        // };
        // self.update_general_reg_with_B32(register, res);
        // let set_flags = !self.in_if_then_block();
        // if set_flags {
        //     // VTOCK TODO: Actually deal with negative values
        //     self.unset_n_flag();
        //     self.set_z_flag();
        //     if carry == 1 {
        //         self.set_c_flag();
        //     } else {
        //         self.unset_c_flag();
        //     }
        // }
        let shift = self.get_value_from_general_reg(&shift);
        let value = self.get_value_from_general_reg(&value);
        let res = value << shift;
        self.update_general_reg_with_b32(register, res);
    }
}
