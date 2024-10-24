use crate::armv7m::{cpu::Armv7m, lang::GeneralPurposeRegister};

use super::utils::shift_left;

impl Armv7m {
    // Str (register) (w) with a LSL (see p. A7-388 in the manual)
    //
    // Store Register (register) calculates an address from a base register value and an offset register value, stores a word
    // from a register to memory. The offset register value can be shifted left by 0, 1, 2, or 3 bits. See Memory accesses on
    // page A7-184 for information about memory accesses.
    //
    // Pseudo code provided by arm:
    //
    // if ConditionPassed() then
    //  EncodingSpecificOperations();
    //  offset = Shift(R[m], shift_t, shift_n, APSR.C); address = R[n] + offset;
    //  MemU[address,4] = R[t];

    // NOTE: Dest cannot be LR, PC, or SP
    #[flux_rs::trusted]
    #[flux_rs::sig(fn (
            self: &strg Armv7m[@old_cpu], 
            GeneralPurposeRegister[@reg_to_store], 
            GeneralPurposeRegister[@reg_base], 
            GeneralPurposeRegister[@reg_offset], 
            u32[@shift]
        ) 
        requires 
            // reg base cannot be the PC
            !is_pc(reg_base)
            &&
            // ofset reg cannot be PC, SP, or LR allowed
            !(is_pc(reg_offset) || is_sp(reg_offset) || is_lr(reg_offset))
        ensures self: Armv7m { 
            new_cpu: check_mem_value_write(
                        get_general_purpose_reg(reg_base, old_cpu) + lshl(get_general_purpose_reg(reg_offset, old_cpu), shift), 
                        new_cpu.mem, 
                        get_general_purpose_reg(reg_to_store, old_cpu)
                     )
        }
    )]
    pub fn strw_lsl_reg(
        &mut self,
        register_to_str: GeneralPurposeRegister,
        base_reg: GeneralPurposeRegister,
        offset_reg: GeneralPurposeRegister,
        shift: u32,
    ) {
        // Corresponds to encoding T2 of Str (register)
        //
        // Specific encoding ops are:
        //
        //  if Rn == '1111' then UNDEFINED;
        //  t = UInt(Rt); n = UInt(Rn); m = UInt(Rm);
        //  index = TRUE; add = TRUE; wback = FALSE;
        //  (shift_t, shift_n) = (SRType_LSL, UInt(imm2));
        //  if t == 15 || m IN {13,15} then UNPREDICTABLE;
        if base_reg == GeneralPurposeRegister::Pc 
            || offset_reg == GeneralPurposeRegister::Sp
            || offset_reg == GeneralPurposeRegister::Lr
            || offset_reg == GeneralPurposeRegister::Pc
        {
            panic!("Preconditions for base and offset register not met")
        }
        let offset = shift_left(self.get_value_from_general_reg(&offset_reg), shift);
        let addr = self.get_value_from_general_reg(&base_reg) + offset;
        self.mem
            .write(addr, self.get_value_from_general_reg(&register_to_str))
    }
}
