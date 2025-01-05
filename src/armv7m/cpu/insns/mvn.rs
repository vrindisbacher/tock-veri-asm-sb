use crate::armv7m::lang::GPR;

use super::super::Armv7m;

use flux_rs::bitvec::BV32;

impl Armv7m {
    // Move (not) Immediate (word) (see p. A7-304 of the manual)
    // Bitwise NOT (immediate) writes the bitwise inverse of an immediate value to the destination register. It can
    // optionally update the condition flags based on the value.
    //
    // Pseudo code provided by arm:
    // if ConditionPassed() then
    //  EncodingSpecificOperations();
    //  result = NOT(imm32);
    //  R[d] = result;
    //  if setflags then
    //   APSR.N = result<31>;
    //   APSR.Z = IsZeroBit(result);
    //   APSR.C = carry;
    //   APSR.V unchanged

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GPR[@reg], BV32[@val])
        ensures self: Armv7m { new_cpu:
            new_cpu == Armv7m { general_regs: set_gpr(reg, old_cpu, bv_not(val)), ..old_cpu }
        }
    )]
    pub fn mvn_imm(&mut self, register: GPR, value: BV32) {
        // Corresponds to encoding T1 of Mvn Immediate
        //
        // Specific encoding ops are:
        // d = UInt(Rd); setflags = (S == '1');
        // (imm32, carry) = ThumbExpandImm_C(i:imm3:imm8, APSR.C);
        // if d IN {13,15} then UNPREDICTABLE;
        //
        //
        // We already know d (register above), setflags is false because no S bit

        // VTOCK TODO: Look at ThumbExpandImm_C
        self.update_general_reg_with_b32(register, !value);
    }
}
