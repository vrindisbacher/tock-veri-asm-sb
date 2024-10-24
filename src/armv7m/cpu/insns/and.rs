use crate::armv7m::lang::GeneralPurposeRegister;

use super::super::Armv7m;

impl Armv7m {
    // And Immediate (see p. A7-200 of the manual)
    //
    // AND (immediate) performs a bitwise AND of a register value and
    // an immediate value, and writes the result to the destination
    // register
    //
    // Pseudo code provided by arm:
    //  if ConditionPassed() then
    //        EncodingSpecificOperations();
    //        result = R[n] AND imm32;
    //        R[d] = result;
    //        if setflags then
    //          APSR.N = result<31>;
    //          APSR.Z = IsZeroBit(result);
    //          APSR.C = carry;
    //          // APSR.V unchanged

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], u32[@val]) 
        // VTOCK TODO: Inspect this pre condition
        // no updates to PC or SP allowed
        requires !(is_pc(reg) || is_sp(reg))
        ensures self: Armv7m { 
            new_cpu: get_general_purpose_reg(reg, new_cpu) == bv_bv32_to_int(bv_and(bv32(get_general_purpose_reg(reg, old_cpu)), bv32(val)))
        }
    )]
    pub fn and_imm(
        &mut self,
        register: GeneralPurposeRegister,
        value: u32,
    ) {
        // Corresponds to encoding T1 of And immediate (VTOCK TODO: Inspect why there is no .W
        // option?)
        //
        // Specific encoding ops are:
        //  if Rd == '1111' && S == '1' then SEE TST (immediate);
        //  d = UInt(Rd); n = UInt(Rn); setflags = (S == '1');
        //  (imm32, carry) = ThumbExpandImm_C(i:imm3:imm8, APSR.C);
        //  if d == 13 || (d == 15 && S == '0') || n IN {13,15} then UNPREDICTABLE;
        //
        // We already know d & n (registers above), setflags is false because no S bit
        // So no SEE TST

        // VTOCK TODO:
        // Look at ThumbExpandImm_C
        let val1 = self.get_value_from_general_reg(&register);
        let res = val1 & value;
        self.update_general_reg_with_u32(register, res);
    }
}