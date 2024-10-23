use super::Armv7m;
use crate::armv7m::instr::{GeneralPurposeRegister, SpecialRegister, Value};

flux_rs::defs! {
    fn get_general_purpose_reg(reg: int, cpu: Armv7m) -> int {
        if reg == 0 {
            cpu.r0
        } else if reg == 1 {
            cpu.r1
        } else if reg == 2 {
            cpu.r2
        } else if reg == 3 {
            cpu.r3
        } else if reg == 4 {
            cpu.r4
        } else if reg == 5 {
            cpu.r5
        } else if reg == 6 {
            cpu.r6
        } else if reg == 7 {
            cpu.r7
        } else if reg == 8 {
            cpu.r8
        } else if reg == 9 {
            cpu.r9
        } else if reg == 10 {
            cpu.r10
        } else if reg == 11 {
            cpu.r11
        } else if reg == 12 {
            cpu.r12
        }  else if reg == 13 {
            cpu.sp
        } else if reg == 14 {
            cpu.lr
        } else  {
            // if reg == 15 {
            cpu.pc
        }
    }

    fn pc_moved(old_cpu: Armv7m, new_cpu: Armv7m) -> bool {
        new_cpu.pc == old_cpu.pc + 4
    }

    fn is_pc(reg: int) -> bool {
        reg == 15
    }

    fn is_sp(reg: int) -> bool {
        reg == 13
    }

    fn bv32(x:int) -> bitvec<32> { bv_int_to_bv32(x) }

    fn get_special_reg(reg: int, cpu: Armv7m) -> int {
        if reg == 16 {
            cpu.control
        } else if reg == 17 {
            cpu.psr
        } else {
            // if reg == 18 {
            // VTOCK TODO: Fix me
            cpu.psr
            // bv_bv32_to_int(bv_and(bv32(cpu.psr), bv32(0xff)))
        }
    }

    fn value_into_u32(value: Value, cpu: Armv7m) -> int {
        if value.is_reg && value.is_special {
            get_special_reg(value.val, cpu)
        } else if value.is_reg {
            get_general_purpose_reg(value.val, cpu)
        } else {
            value.val
        }
    }

    fn nth_bit(val: int, n: int) -> int {
        // val & (1 << n)
        bv_bv32_to_int(bv_and(bv32(val), bv_lshr(bv32(1), bv32(n))))
    }

    // 0 being the least significant bit, 31 the most significant
    fn nth_bit_is_set(val: int, n: int) -> bool {
        nth_bit(val, n) == 1
    }

    fn nth_bit_is_unset(val: int, n: int) -> bool {
        nth_bit(val, n) == 0
    }
}
