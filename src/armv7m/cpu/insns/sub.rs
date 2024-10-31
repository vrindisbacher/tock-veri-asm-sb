use crate::{armv7m::lang::GeneralPurposeRegister, flux_support::b32::B32};

use super::{super::Armv7m, utils::sub};
impl Armv7m {
    // Sub Immediate (see p. A7-402 of the manual)
    //
    // Subtract (immediate) subtracts an immediate value from a register value, and writes the result to the destination
    // register. It can optionally update the condition flags based on the result.
    //
    // Pseudo code provided by arm:
    // if ConditionPassed() then EncodingSpecificOperations();
    //  (result, carry, overflow) = AddWithCarry(R[n], NOT(imm32), '1');
    //  R[d] = result;
    //  if setflags then
    //      APSR.N = result<31>;
    //      APSR.Z = IsZeroBit(result);
    //      APSR.C = carry;
    //      APSR.V = overflow;

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], GeneralPurposeRegister[@val1], B32[@val2]) 
        ensures self: Armv7m 
            { 
                 new_cpu: 
                     general_purpose_register_updated(reg, old_cpu, new_cpu, 
                                                           //wrapping_add_B32_with_carry(get_general_purpose_reg(val1, old_cpu), negated(val2), 1))
                                                           wrapping_add_B32(get_general_purpose_reg(val1, old_cpu), negated(val2)))
                     &&
                     old_cpu.special_regs == new_cpu.special_regs
                     &&
                     old_cpu.mem == new_cpu.mem
            }
    )]
    pub fn subw_imm(
        &mut self,
        register: GeneralPurposeRegister,
        value1: GeneralPurposeRegister,
        value2: B32,
    ) {
        // Corresponds to encoding T3 of Sub immediate:
        //
        // Specific encoding ops are:
        // if Rd == '1111' && S == '1' then SEE CMP (immediate);
        // if Rn == '1101' then SEE SUB (SP minus immediate);
        // d = UInt(Rd); n = UInt(Rn); setflags = (S == '1'); imm32 = ThumbExpandImm(i:imm3:imm8);
        // if d == 13 || (d == 15 && S == '0') || n == 15 then UNPREDICTABLE;
        //
        // We already know d & n (register aboves) & that there is no S bit set so no SEE CMP
        // Also Rn == '1101' is the SP which we are not dealing with for now so no SEE SUB

        // VTOCK TODO: Inspect ThumbExpandImm (same as ThumbExpandImm_C ignoring the carry flag)
        let val1 = self.get_value_from_general_reg(&value1);
        let res = sub(val1, value2, B32::from(1));
        self.update_general_reg_with_b32(register, res);
    }
}
