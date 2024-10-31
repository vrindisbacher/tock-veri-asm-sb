use crate::flux_support::b32::B32;

#[flux_rs::trusted]
#[flux_rs::sig(fn (B32[@value], B32[@n]) -> B32[nth_bit(value, n)] requires n <= 31)]
pub fn get_nth_bit(value: B32, n: B32) -> B32 {
    value & (B32::from(1) << n)
}

#[flux_rs::trusted]
#[flux_rs::sig(fn (B32[@val1], B32[@val2]) -> B32[and(val1, val2)])]
pub fn and(val1: B32, val2: B32) -> B32 {
    val1 & val2
}

#[flux_rs::trusted]
#[flux_rs::sig(fn (B32[@val], B32[@shift]) -> B32[right_shift(val, shift)])]
pub fn shift_right(val: B32, shift: B32) -> B32 {
    val >> shift
}

#[flux_rs::trusted]
#[flux_rs::sig(fn (B32[@val], B32[@shift]) -> B32[left_shift(val, shift)])]
pub fn shift_left(val: B32, shift: B32) -> B32 {
    val << shift
}

#[flux_rs::trusted]
#[flux_rs::sig(fn (B32[@val]) -> B32[negated(val)])]
pub fn negate(val: B32) -> B32 {
    !val
}

#[flux_rs::sig(fn (B32[@val1], B32[@val2], B32[@carry]) -> B32[wrapping_add_u32(val1, negated(val2))])]
// B32[wrapping_add_B32_with_carry(val1, negated(val2), carry)] requires carry == 0 || carry == 1)]
pub fn sub(val1: B32, val2: B32, _carry: B32) -> B32 {
    val1.wrapping_add(!val2)
    // .wrapping_add(carry)
    // val1 - val2
}
