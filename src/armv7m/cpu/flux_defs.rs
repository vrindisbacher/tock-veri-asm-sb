use super::Armv7m;
use crate::armv7m::lang::{GeneralPurposeRegister, SpecialRegister, Value};

const U32_MAX: u32 = std::u32::MAX;

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

    fn is_lr(reg: int) -> bool {
        reg == 14
    }

    fn is_sp(reg: int) -> bool {
        reg == 13
    }

    fn is_control(reg: int) -> bool {
        reg == 16
    }

    fn bv32(x:int) -> bitvec<32> { bv_int_to_bv32(x) }

    fn get_ipsr(cpu: Armv7m) -> int {
            bv_bv32_to_int(bv_and(bv32(cpu.psr), bv32(0xff)))
    }

    fn get_special_reg(reg: int, cpu: Armv7m) -> int {
        if reg == 16 {
            cpu.control
        } else if reg == 17 {
            cpu.psr
        } else {
            // if reg == 18 {
            get_ipsr(cpu)
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
        bv_bv32_to_int(bv_and(bv32(val), lshr_bv32(1, n)))
    }

    fn lshr_bv32(val: int, n: int) -> bitvec<32> {
        // right shift
        bv_lshr(bv32(val), bv32(n))
    }

    fn right_shift(val: int, n: int) -> int {
        // right shift
        bv_bv32_to_int(lshr_bv32(val, n))
    }

    fn left_shift(val: int, n: int) -> int {
        // shift left
        bv_bv32_to_int(bv_shl(bv32(val), bv32(n)))
    }

    // 0 being the least significant bit, 31 the most significant
    fn nth_bit_is_set(val: int, n: int) -> bool {
        nth_bit(val, n) == 1
    }

    fn nth_bit_is_unset(val: int, n: int) -> bool {
        nth_bit(val, n) == 0
    }

    fn negated(val: int) -> int {
        bv_bv32_to_int(bv_not(bv32(val)))
    }

    fn and(val1: int, val2: int) -> int {
        bv_bv32_to_int(bv_and(bv32(val1), bv32(val2)))
    }

    fn itstate_0_4_not_all_zero(cpu: Armv7m) -> bool {
        !(
            nth_bit(cpu.psr, 25) == 0
            &&
            nth_bit(cpu.psr, 26) == 0
            &&
            nth_bit(cpu.psr, 10) == 0
            &&
            nth_bit(cpu.psr, 11) == 0
        )
    }

    fn movs_flag_updates(cpu: Armv7m) -> bool {
        if !itstate_0_4_not_all_zero(cpu) {
            // flag updates
            // n flag and z flag are unset and set
            nth_bit_is_unset(cpu.psr, 31) && nth_bit_is_set(cpu.psr, 30)
        } else {
                // no flag updates
                true
        }
    }

    fn general_purpose_register_updated(reg: GeneralPurposeRegister, cpu: Armv7m, val: int) -> bool {
        get_general_purpose_reg(reg, cpu) == val
    }

    // fn right_shift_immediate_computation(reg: GeneralPurposeRegister, old_cpu: Armv7m, shift: int) -> int {
    //     if (
    //         get_general_purpose_reg(reg, old_cpu) > 0
    //         &&
    //         lshr(get_general_purpose_reg(reg, old_cpu), shift) == get_general_purpose_reg(reg, old_cpu)
    //     ) {
    //         0
    //     } else {
    //         lshr(get_general_purpose_reg(reg, old_cpu), shift)
    //     }
    // }

    // fn right_shift_immediate_carry_flag(reg: GeneralPurposeRegister, old_cpu: Armv7m, shift: int) -> int {
    //     if (
    //         get_general_purpose_reg(reg, old_cpu) > 0
    //         &&
    //         lshr(get_general_purpose_reg(reg, old_cpu), shift) == get_general_purpose_reg(reg, old_cpu)
    //     ) {
    //         nth_bit(get_general_purpose_reg(reg, old_cpu), 31)
    //     } else {
    //         if shift >= 1 && shift <= 31 {
    //             nth_bit(get_general_purpose_reg(reg, old_cpu), shift - 1)
    //         } else {
    //             0
    //         }
    //     }
    // }

    fn lsrs_imm_flag_updates(reg: GeneralPurposeRegister, old_cpu: Armv7m, new_cpu: Armv7m, shift: int) -> bool {
        if !itstate_0_4_not_all_zero(old_cpu) {
            // flag updates
            // n flag and z flag are unset and set and carry is computed
            nth_bit_is_unset(new_cpu.psr, 31)
            &&
            nth_bit_is_set(new_cpu.psr, 30)
            &&
            if right_shift_immediate_carry_flag(reg, old_cpu, shift) == 1 {
                nth_bit_is_set(new_cpu.psr, 29)
            } else if right_shift_immediate_carry_flag(reg, old_cpu, shift) == 0 {
                nth_bit_is_unset(new_cpu.psr, 29)
            } else {
                // that's wrong :)
                false
            }
        } else {
                // no flag updates
                true
        }
    }

    // fn left_shift_reg_computation(reg: GeneralPurposeRegister, old_cpu: Armv7m, shift: int) -> int {
    //     if (
    //         get_general_purpose_reg(reg, old_cpu) > 0
    //         &&
    //         lshl(get_general_purpose_reg(reg, old_cpu), shift) == get_general_purpose_reg(reg, old_cpu)
    //     ) {
    //         0
    //     } else {
    //         lshl(get_general_purpose_reg(reg, old_cpu), shift)
    //     }
    // }

    // fn left_shift_reg_carry_flag(reg: GeneralPurposeRegister, old_cpu: Armv7m, shift: int) -> int {
    //     if (
    //         get_general_purpose_reg(reg, old_cpu) > 0
    //         &&
    //         lshl(get_general_purpose_reg(reg, old_cpu), shift) == get_general_purpose_reg(reg, old_cpu)
    //     ) {
    //         nth_bit(get_general_purpose_reg(reg, old_cpu), 31)
    //     } else {
    //         if shift >= 1 && shift <= 31 {
    //             nth_bit(get_general_purpose_reg(reg, old_cpu), shift - 1)
    //         } else {
    //             0
    //         }
    //     }
    // }

    // fn lslw_reg_flag_updates(reg: GeneralPurposeRegister, old_cpu: Armv7m, new_cpu: Armv7m, shift: int) -> bool {
    //     if !itstate_0_4_not_all_zero(old_cpu) {
    //         // flag updates
    //         // n flag and z flag are unset and set and carry is computed
    //         nth_bit_is_unset(new_cpu.psr, 31)
    //         &&
    //         nth_bit_is_set(new_cpu.psr, 30)
    //         &&
    //         if left_shift_reg_carry_flag(reg, old_cpu, shift) == 1 {
    //             nth_bit_is_set(new_cpu.psr, 29)
    //         } else if left_shift_reg_carry_flag(reg, old_cpu, shift) == 0 {
    //             nth_bit_is_unset(new_cpu.psr, 29)
    //         } else {
    //             // that's wrong :)
    //             false
    //         }
    //     } else {
    //             // no flag updates
    //             true
    //     }
    // }

    fn wrapping_add_u32(val1: int, val2: int) -> int {
        if val1 + val2 > U32_MAX {
            val1 + val2 % U32_MAX
        } else {
            val1 + val2
        }
    }

    fn wrapping_add_u32_with_carry(val1: int, val2: int, val3: int) -> int {
        wrapping_add_u32(wrapping_add_u32(val1, val2), val3)
    }
}

#[flux_rs::extern_spec(std::u32)]
impl u32 {
    #[flux_rs::sig(fn (u32[@val1], u32[@val2]) -> u32[wrapping_add_u32(val1, val2)])]
    fn wrapping_add(self, other: u32) -> u32;
}
