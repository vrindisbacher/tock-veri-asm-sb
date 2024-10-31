use crate::flux_support::b32::{from, B32};

// NVIC
//
// Some unimplemented blocks:
//
// 0xE000E380 -0xE000E3FC	-	-	-	Reserved
//
// 0xE000E7F0 -0xE000ECFC	-	-	-	Reserved
use super::flux_defs::nvic_defs::*;

pub const ISER_START: B32 = from(0xE000E100);
pub const ISER_END: B32 = from(0xE000E13C);
//
pub const ICER_START: B32 = from(0xE000E180);
pub const ICER_END: B32 = from(0xE000E1BC);
//
pub const ISPR_START: B32 = from(0xE000E200);
pub const ISPR_END: B32 = from(0xE000E23C);
//
pub const ICPR_START: B32 = from(0xE000E280);
pub const ICPR_END: B32 = from(0xE000E2BC);
//
pub const IABR_START: B32 = from(0xE000E300);
pub const IABR_END: B32 = from(0xE000E37C);
//
pub const IPR_START: B32 = from(0xE000E400);
pub const IPR_END: B32 = from(0xE000E7EC);

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_nvic_addr(addr)])]
fn is_valid_nvic_addr(address: B32) -> bool {
    if address >= ISER_START && address <= ISER_END {
        (address - ISER_START) % B32::from(4) == B32::from(0)
    } else if address >= ICER_START && address <= ICER_END {
        (address - ICER_START) % B32::from(4) == B32::from(0)
    } else if address >= ISPR_START && address <= ISPR_END {
        (address - ISPR_START) % B32::from(4) == B32::from(0)
    } else if address >= ICPR_START && address <= ICPR_END {
        (address - ICPR_START) % B32::from(4) == B32::from(0)
    } else if address >= IABR_START && address <= IABR_END {
        (address - IABR_START) % B32::from(4) == B32::from(0)
    } else if address >= IPR_START && address <= IPR_END {
        (address - IPR_START) % B32::from(4) == B32::from(0)
    } else {
        false
    }
}

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_nvic_read_addr(addr)])]
pub fn is_valid_nvic_read_addr(address: B32) -> bool {
    // all read
    is_valid_nvic_addr(address)
}

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_nvic_write_addr(addr)])]
pub fn is_valid_nvic_write_addr(address: B32) -> bool {
    // all write
    is_valid_nvic_addr(address)
}
