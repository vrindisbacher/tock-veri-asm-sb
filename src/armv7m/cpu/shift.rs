use crate::armv7m::instr::GeneralPurposeRegister;

use super::Armv7m;

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

    #[flux_rs::trusted]
    pub fn lsrs_imm(
        &mut self,
        register: GeneralPurposeRegister,
        value: GeneralPurposeRegister,
        shift: u32,
    ) {
        // Corresponds to encoding T1 of LSR
        //
        // Specific encoding ops are:
        //      d = UInt(Rdn); n = UInt(Rdn); m = UInt(Rm); setflags = !InITBlock();
        //
        //  We already know d & n (registers above)
        if shift == 0 {
            return;
        }
        let value1 = self.get_value_from_general_reg(&value);
        let res = value1 >> shift;
        let (res, carry) = if value1 > 0 && res == value1 {
            (0, Self::get_nth_bit(value1, 31))
        } else {
            (
                res,
                match shift {
                    1..32 => Self::get_nth_bit(value1, shift - 1),
                    _ => 0,
                },
            )
        };
        self.update_general_reg_with_u32(register, res);
        let set_flags = !self.in_if_then_block();
        if set_flags {
            // VTOCK TODO: Actually deal with negative values
            self.unset_n_flag();
            self.set_z_flag();
            if carry == 1 {
                self.set_c_flag();
            } else {
                self.unset_c_flag();
            }
        }
        // TODO: There are a bunch of flag updates here
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
    #[flux_rs::trusted]
    // VTOCK TODO: Actually can't be lr, sp, or pc as destination
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
        let shift = self.get_value_from_general_reg(&shift);
        if shift == 0 {
            return;
        }
        let value1 = self.get_value_from_general_reg(&value);
        let res = value1 << shift;
        let (res, carry) = if value1 > 0 && res == value1 {
            (0, Self::get_nth_bit(value1, 31))
        } else {
            (
                res,
                match shift {
                    1..32 => Self::get_nth_bit(value1, shift - 1),
                    _ => 0,
                },
            )
        };
        self.update_general_reg_with_u32(register, res);
        let set_flags = !self.in_if_then_block();
        if set_flags {
            // VTOCK TODO: Actually deal with negative values
            self.unset_n_flag();
            self.set_z_flag();
            if carry == 1 {
                self.set_c_flag();
            } else {
                self.unset_c_flag();
            }
        }
    }
}
