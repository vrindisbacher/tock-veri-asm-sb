
#[flux_rs::trusted]
#[flux_rs::sig(fn (u32[@value], u32[@n]) -> u32[nth_bit(value, n)] requires n <= 31)]
pub fn get_nth_bit(value: u32, n: u32) -> u32 {
    value & (1 << n)
}

#[flux_rs::trusted]
#[flux_rs::sig(fn (u32[@val1], u32[@val2]) -> u32[and(val1, val2)])]
pub fn and(val1: u32, val2: u32) -> u32 {
    val1 & val2
}

#[flux_rs::trusted]
#[flux_rs::sig(fn (u32[@val], u32[@shift]) -> u32[lshr(val, shift)])]
pub fn shift_right(val: u32, shift: u32) -> u32 {
    val >> shift
}

#[flux_rs::trusted]
#[flux_rs::sig(fn (u32[@val], u32[@shift]) -> u32[lshl(val, shift)])]
pub fn shift_left(val: u32, shift: u32) -> u32 {
    val << shift
}

#[flux_rs::trusted]
#[flux_rs::sig(fn (u32[@val]) -> u32[negated(val)])]
pub fn negate(val: u32) -> u32 {
    !val
}

#[flux_rs::sig(fn (u32[@val1], u32[@val2], u32[@carry]) -> u32[wrapping_add_u32_with_carry(val1, negated(val2), carry)] requires carry == 0 || carry == 1)]
pub fn sub(val1: u32, val2: u32, carry: u32) -> u32 {
    val1.wrapping_add(negate(val2)).wrapping_add(carry)
}
