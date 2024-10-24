use crate::armv7m::lang::GeneralPurposeRegister;
use super::super::Armv7m;
use super::super::flux_defs::*;

impl Armv7m {
    // Move Immediate (see p. A7-291 of the manual)
    //
    // Pseudo code provided by arm: 
    // if ConditionPassed() then
    //   EncodingSpecificOperations();
    //   result = imm32;
    //   R[d] = result;
    //   if setflags then
    //       APSR.N = result<31>;
    //       APSR.Z = IsZeroBit(result);
    //       APSR.C = carry;
    //       // APSR.V unchanged

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], u32[@val]) 
        // no updates to PC or SP allowed
        // VTOCK TODO: Inspect PC + SP precondition
        requires !(is_pc(reg) || is_sp(reg))
        ensures self: Armv7m { 
            new_cpu: get_general_purpose_reg(reg, new_cpu) == val
        }
    )]
    pub fn movw_imm(&mut self, register: GeneralPurposeRegister, value: u32) {
        // Corresponds to encoding T2 of Mov immediate
        //
        // Specific encoding ops are:
        //      d = UInt(Rd);  setflags = (S == '1');  (imm32, carry) = ThumbExpandImm_C(i:imm3:imm8, APSR.C);
        //      if d IN {13,15} then UNPREDICTABLE;
        //
        // We already know d (register above), setflags is false because no S bit

        // VTOCK TODO: 
        // Look at ThumbExpandImm_C
        self.update_general_reg_with_u32(register, value);
    }
  
    #[flux_rs::trusted]
    pub fn movs_imm(&mut self, register: GeneralPurposeRegister, value: u32) {
        // Corresponds to encoding T1 of Mov immediate: 
        //
        // Specific encoding ops are:
        // d = UInt(Rd);  setflags = !InITBlock();  imm32 = ZeroExtend(imm8, 32);  carry = APSR.C;
        //
        // We already know d (register above)
        self.update_general_reg_with_u32(register, value);
        let set_flags = !self.in_if_then_block();
        if set_flags {
            // VTOCK TODO: Actually deal with negative values
            self.unset_n_flag();
            self.set_z_flag();
        }
    }

}
