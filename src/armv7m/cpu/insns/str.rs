use crate::armv7m::{cpu::Armv7m, lang::GPR};
use flux_rs::bitvec::BV32;

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
    #[flux_rs::sig(fn (
            self: &strg Armv7m[@old_cpu],
            GPR[@reg_to_store],
            GPR[@reg_base],
            GPR[@reg_offset],
            BV32[@shift]
        )
        requires
            is_valid_write_addr(
                bv_add(
                    get_gpr(reg_base, old_cpu),
                    left_shift(get_gpr(reg_offset, old_cpu), shift)
                )
            )
        ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                mem: update_mem(
                        bv_add(
                            get_gpr(reg_base, old_cpu),
                            left_shift(get_gpr(reg_offset, old_cpu), shift)
                        ),
                        old_cpu.mem,
                        get_gpr(reg_to_store, old_cpu)
                ),
                ..old_cpu
            }
        }
    )]
    pub fn strw_lsl_reg(
        &mut self,
        register_to_str: GPR,
        base_reg: GPR,
        offset_reg: GPR,
        shift: BV32,
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
        let offset = self.get_value_from_general_reg(&offset_reg) << shift;
        let addr = (self.get_value_from_general_reg(&base_reg) + offset);
        let value = self.get_value_from_general_reg(&register_to_str);
        self.mem.write(addr, value);
    }

    #[flux_rs::sig(fn (
            self: &strg Armv7m[@old_cpu],
            GPR[@rt],
            GPR[@rn],
        )
        requires is_valid_write_addr(get_gpr(rn, old_cpu))
        ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                mem: update_mem(
                        get_gpr(rn, old_cpu),
                        old_cpu.mem,
                        get_gpr(rt, old_cpu)
                ),
                ..old_cpu
            }
        }
    )]
    pub fn str_no_wback(&mut self, rt: GPR, rn: GPR) {
        let addr = self.get_value_from_general_reg(&rn);
        let val = self.get_value_from_general_reg(&rt);
        self.mem.write(addr, val);
    }
}
