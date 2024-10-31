use std::ops::{Add, BitAnd, BitOr, Not, Rem, Shl, Shr, Sub};

flux_rs::defs! {
    fn bv32(val: int) -> bitvec<32> {
        bv_int_to_bv32(val)
    }

    fn int(val: bitvec<32>) -> int {
        bv_bv32_to_int(val)
    }

    fn not(val: bitvec<32>) -> bitvec<32> {
        bv_not(val)
    }

    fn and(val1: bitvec<32>, val2: bitvec<32>) -> bitvec<32> {
        bv_and(val1, val2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[flux_rs::opaque]
#[flux_rs::refined_by(x: bitvec<32>)]
pub struct B32(u32);

#[flux_rs::trusted]
#[flux_rs::sig(fn (u32[@x]) -> B32[bv32(x)])]
pub const fn from(x: u32) -> B32 {
    B32(x)
}

impl B32 {
    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@x], B32[@y]) -> B32[bv_add(x, y)])]
    pub fn wrapping_add(self, other: B32) -> B32 {
        B32(self.0.wrapping_add(other.0))
    }
}

impl From<u32> for B32 {
    #[flux_rs::trusted]
    #[flux_rs::sig(fn (u32[@val]) -> B32[bv32(val)])]
    fn from(value: u32) -> B32 {
        B32(value)
    }
}

impl Into<u32> for B32 {
    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@val]) -> u32[int(val)])]
    fn into(self) -> u32 {
        self.0
    }
}

impl Not for B32 {
    type Output = B32;

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@x]) -> B32[bv_not(x)])]
    fn not(self) -> B32 {
        B32(!self.0)
    }
}

impl BitAnd for B32 {
    type Output = B32;

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@x], B32[@y]) -> B32[bv_and(x, y)])]
    fn bitand(self, rhs: Self) -> B32 {
        B32(self.0 & rhs.0)
    }
}

impl BitOr for B32 {
    type Output = B32;

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@x], B32[@y]) -> B32[bv_or(x, y)])]
    fn bitor(self, rhs: Self) -> B32 {
        B32(self.0 | rhs.0)
    }
}

impl Shl for B32 {
    type Output = B32;

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@x], B32[@y]) -> B32[bv_shl(x, y)])]
    fn shl(self, rhs: Self) -> B32 {
        B32(self.0 << rhs.0)
    }
}

impl Shr for B32 {
    type Output = B32;

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@x], B32[@y]) -> B32[bv_lshr(x, y)])]
    fn shr(self, rhs: Self) -> B32 {
        B32(self.0 >> rhs.0)
    }
}

impl Add for B32 {
    type Output = B32;

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@val1], B32[@val2]) -> B32[bv_add(val1, val2)])]
    fn add(self, rhs: Self) -> B32 {
        B32(self.0 + rhs.0)
    }
}

impl Sub for B32 {
    type Output = B32;

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@val1], B32[@val2]) -> B32[bv_sub(val1, val2)])]
    fn sub(self, rhs: Self) -> B32 {
        B32(self.0.wrapping_add(!rhs.0))
    }
}

impl Rem for B32 {
    type Output = B32;

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@val1], B32[@val2]) -> B32[bv_urem(val1, val2)])]
    fn rem(self, rhs: Self) -> B32 {
        B32(self.0 & rhs.0)
    }
}
