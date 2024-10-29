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
    // TOCK TODO: Seems like there's a bug here
    #[flux_rs::trusted]
    #[flux_rs::sig(fn (
            self: &strg Armv7m[@old_cpu], 
            GeneralPurposeRegister[@reg_to_store], 
            GeneralPurposeRegister[@reg_base], 
            GeneralPurposeRegister[@reg_offset], 
            u32[@shift]
        ) 
        requires 
            is_valid_write_addr(
                get_general_purpose_reg(reg_base, old_cpu) + left_shift(get_general_purpose_reg(reg_offset, old_cpu), shift), 
            )
        ensures self: Armv7m { 
            new_cpu: mem_value_updated(
                        get_general_purpose_reg(reg_base, old_cpu) + left_shift(get_general_purpose_reg(reg_offset, old_cpu), shift), 
                        old_cpu.mem,
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
        let offset = shift_left(self.get_value_from_general_reg(&offset_reg), shift);
        let addr = self.get_value_from_general_reg(&base_reg) + offset;
        self.mem
            .write(addr, self.get_value_from_general_reg(&register_to_str))
    }

    #[flux_rs::sig(fn (
            self: &strg Armv7m[@old_cpu], 
            u32[@val],
            GeneralPurposeRegister[@reg_base], 
        ) 
        requires is_valid_write_addr(get_general_purpose_reg(reg_base, old_cpu))
        ensures self: Armv7m { 
            new_cpu: mem_value_updated(
                        get_general_purpose_reg(reg_base, old_cpu),
                        old_cpu.mem,
                        new_cpu.mem, 
                        val
                     )
        }
    )]
    pub fn str_direct(&mut self, value: u32, addr: GeneralPurposeRegister) {
        let addr = self.get_value_from_general_reg(&addr);
        self.mem.write(addr, value);
    }

    #[flux_rs::sig(fn (
            self: &strg Armv7m[@old_cpu], 
            u32[@val],
            u32[@reg_base], 
        ) 
        requires is_valid_write_addr(reg_base)
        ensures self: Armv7m { 
            new_cpu: mem_value_updated(
                        reg_base,
                        old_cpu.mem,
                        new_cpu.mem, 
                        val
                     )
        }
    )]
    pub fn str_super_direct(&mut self, value: u32, addr: u32) {
        self.mem.write(addr, value);
    }
}
