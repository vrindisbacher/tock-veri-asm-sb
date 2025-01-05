use super::super::flux_defs::*;
use super::super::Armv7m;
use crate::armv7m::lang::GPR;
use flux_rs::bitvec::BV32;

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
    //

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GPR[@rd], GPR[@rm])
       ensures self: Armv7m { new_cpu:
           new_cpu == Armv7m { general_regs: set_gpr(rd, old_cpu, get_gpr(rm, old_cpu)), ..old_cpu }
       }
    )]
    pub fn mov(&mut self, rd: GPR, rm: GPR) {
        // No flag updates - just moves rm to rd
        self.update_general_reg_with_b32(rd, self.get_value_from_general_reg(&rm));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GPR[@reg], BV32[@val])
        ensures self: Armv7m { new_cpu:
            new_cpu == Armv7m { general_regs: set_gpr(reg, old_cpu, val), ..old_cpu }
        }
    )]
    pub fn movw_imm(&mut self, register: GPR, value: BV32) {
        // Corresponds to encoding T2 of Mov immediate
        //
        // Specific encoding ops are:
        //      d = UInt(Rd);  setflags = (S == '1');  (imm32, carry) = ThumbExpandImm_C(i:imm3:imm8, APSR.C);
        //      if d IN {13,15} then UNPREDICTABLE;
        //
        // We already know d (register above), setflags is false because no S bit

        // VTOCK TODO:
        // Look at ThumbExpandImm_C
        self.update_general_reg_with_b32(register, value);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], GPR[@reg], BV32[@val])
            // TODO(VR): Flag Updates
            ensures self: Armv7m[{ general_regs: set_gpr(reg, old_cpu, val), ..old_cpu }]
    )]
    pub fn movs_imm(&mut self, register: GPR, value: BV32) {
        // Corresponds to encoding T1 of Mov immediate:
        //
        // Specific encoding ops are:
        // d = UInt(Rd);  setflags = !InITBlock();  imm32 = ZeroExtend(imm8, 32);  carry = APSR.C;
        //
        // We already know d (register above)
        self.update_general_reg_with_b32(register, value);
        // let set_flags = !self.in_if_then_block();
        // if set_flags {
        //     // VTOCK TODO: Actually deal with negative values
        //     self.unset_n_flag();
        //     self.set_z_flag();
        // }
    }
}
