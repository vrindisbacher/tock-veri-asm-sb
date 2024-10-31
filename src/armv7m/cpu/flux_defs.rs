use super::Armv7m;
use crate::armv7m::lang::{GeneralPurposeRegister, SpecialRegister};
use crate::flux_support::rmap::*;

const U32_MAX: u32 = std::u32::MAX;

flux_rs::defs! {
    fn get_general_purpose_reg(reg: int, cpu: Armv7m) -> int {
        map_get(cpu.general_regs, reg)
    }

    fn general_purpose_register_updated(reg: int, old_cpu: Armv7m, new_cpu: Armv7m, val: int) -> bool {
        map_set(old_cpu.general_regs, reg, val) == new_cpu.general_regs
    }

    fn get_special_reg(reg: int, cpu: Armv7m) -> int {
        if is_ipsr(reg) {
            bv_bv32_to_int(bv_and(bv32(map_get(cpu.special_regs, psr())), bv32(0xff)))
        } else {
            map_get(cpu.special_regs, reg)
        }
    }

    fn get_psr(cpu: Armv7m) ->  int {
        get_special_reg(psr(), cpu)
    }

    fn special_purpose_register_updated(reg: int, old_cpu: Armv7m, new_cpu: Armv7m, val: int) -> bool {
        map_set(old_cpu.special_regs, reg, val) == new_cpu.special_regs
    }

    fn is_ipsr(reg: int) -> bool {
        reg == 18
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

    fn r0() -> int {
        0
    }

    fn r1() -> int {
        1
    }

    fn r2() -> int {
        2
    }

    fn r3() -> int {
        3
    }

    fn r4() -> int {
        4
    }

    fn lr() -> int {
        14
    }

    fn control() -> int {
        16
    }

    fn psr() -> int {
        17
    }

    fn ipsr() -> int {
        18
    }

    fn bv32(x:int) -> bitvec<32> { bv_int_to_bv32(x) }

    fn nth_bit(val: int, n: int) -> int {
        // val & (1 << n)
        bv_bv32_to_int(bv_and(bv32(val), left_shift_bv32(1, n)))
    }

    fn right_shift_bv32(val: int, n: int) -> bitvec<32> {
        bv_lshr(bv32(val), bv32(n))
    }

    fn right_shift(val: int, n: int) -> int {
        // right shift
        bv_bv32_to_int(right_shift_bv32(val, n))
    }

    fn left_shift_bv32(val: int, n: int) -> bitvec<32> {
        bv_shl(bv32(val), bv32(n))
    }

    fn left_shift(val: int, n: int) -> int {
        // shift left
        bv_bv32_to_int(left_shift_bv32(val, n))
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

    fn or(val1: int, val2: int) -> int {
        bv_bv32_to_int(bv_or(bv32(val1), bv32(val2)))
    }

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

    // fn itstate_0_4_not_all_zero(cpu: Armv7m) -> bool {
    //     !(
    //         nth_bit(cpu.psr, 25) == 0
    //         &&
    //         nth_bit(cpu.psr, 26) == 0
    //         &&
    //         nth_bit(cpu.psr, 10) == 0
    //         &&
    //         nth_bit(cpu.psr, 11) == 0
    //     )
    // }

    // fn movs_flag_updates(cpu: Armv7m) -> bool {
    //     if !itstate_0_4_not_all_zero(cpu) {
    //         // flag updates
    //         // n flag and z flag are unset and set
    //         nth_bit_is_unset(cpu.psr, 31) && nth_bit_is_set(cpu.psr, 30)
    //     } else {
    //             // no flag updates
    //             true
    //     }
    // }

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

    // fn lsrs_imm_flag_updates(reg: GeneralPurposeRegister, old_cpu: Armv7m, new_cpu: Armv7m, shift: int) -> bool {
    //     if !itstate_0_4_not_all_zero(old_cpu) {
    //         // flag updates
    //         // n flag and z flag are unset and set and carry is computed
    //         nth_bit_is_unset(new_cpu.psr, 31)
    //         &&
    //         nth_bit_is_set(new_cpu.psr, 30)
    //         &&
    //         if right_shift_immediate_carry_flag(reg, old_cpu, shift) == 1 {
    //             nth_bit_is_set(new_cpu.psr, 29)
    //         } else if right_shift_immediate_carry_flag(reg, old_cpu, shift) == 0 {
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
}

#[flux_rs::extern_spec(std::u32)]
impl u32 {
    #[flux_rs::sig(fn (u32[@val1], u32[@val2]) -> u32[wrapping_add_u32(val1, val2)])]
    fn wrapping_add(self, other: u32) -> u32;
}
