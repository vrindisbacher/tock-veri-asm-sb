use flux_rs::bitvec::BV32;

// NVIC
//
// Some unimplemented blocks:
//
// 0xE000E380 -0xE000E3FC	-	-	-	Reserved
//
// 0xE000E7F0 -0xE000ECFC	-	-	-	Reserved
use super::flux_defs::nvic_defs::*;

#[flux_rs::constant(bv_int_to_bv32(0xE000E100))]
pub const ISER_START: BV32 = BV32::new(0xE000E100);
#[flux_rs::constant(bv_int_to_bv32(0xE000E13C))]
pub const ISER_END: BV32 = BV32::new(0xE000E13C);
//
#[flux_rs::constant(bv_int_to_bv32(0xE000E180))]
pub const ICER_START: BV32 = BV32::new(0xE000E180);
#[flux_rs::constant(bv_int_to_bv32(0xE000E1BC))]
pub const ICER_END: BV32 = BV32::new(0xE000E1BC);
//
#[flux_rs::constant(bv_int_to_bv32(0xE000E200))]
pub const ISPR_START: BV32 = BV32::new(0xE000E200);
#[flux_rs::constant(bv_int_to_bv32(0xE000E23C))]
pub const ISPR_END: BV32 = BV32::new(0xE000E23C);
//
#[flux_rs::constant(bv_int_to_bv32(0xE000E280))]
pub const ICPR_START: BV32 = BV32::new(0xE000E280);
#[flux_rs::constant(bv_int_to_bv32(0xE000E2BC))]
pub const ICPR_END: BV32 = BV32::new(0xE000E2BC);
//
#[flux_rs::constant(bv_int_to_bv32(0xE000E300))]
pub const IABR_START: BV32 = BV32::new(0xE000E300);
#[flux_rs::constant(bv_int_to_bv32(0xE000E37C))]
pub const IABR_END: BV32 = BV32::new(0xE000E37C);
//
#[flux_rs::constant(bv_int_to_bv32(0xE000E400))]
pub const IPR_START: BV32 = BV32::new(0xE000E400);
#[flux_rs::constant(bv_int_to_bv32(0xE000E7EC))]
pub const IPR_END: BV32 = BV32::new(0xE000E7EC);

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_nvic_addr(addr)])]
fn is_valid_nvic_addr(address: BV32) -> bool {
    let iser_start = ISER_START;
    let iser_end = ISER_END;
    let icer_start = ICER_START;
    let icer_end = ICER_END;
    let ispr_start = ISPR_START;
    let ispr_end = ISPR_END;
    let icpr_start = ICPR_START;
    let icpr_end = ICPR_END;
    let iabr_start = IABR_START;
    let iabr_end = IABR_END;
    let ipr_start = IPR_START;
    let ipr_end = IPR_END;
    if address >= iser_start && address <= iser_end {
        (address - iser_start) % BV32::from(4) == BV32::from(0)
    } else if address >= icer_start && address <= icer_end {
        (address - icer_start) % BV32::from(4) == BV32::from(0)
    } else if address >= ispr_start && address <= ispr_end {
        (address - ispr_start) % BV32::from(4) == BV32::from(0)
    } else if address >= icpr_start && address <= icpr_end {
        (address - icpr_start) % BV32::from(4) == BV32::from(0)
    } else if address >= iabr_start && address <= iabr_end {
        (address - iabr_start) % BV32::from(4) == BV32::from(0)
    } else if address >= ipr_start && address <= ipr_end {
        (address - ipr_start) % BV32::from(4) == BV32::from(0)
    } else {
        false
    }
}

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_nvic_read_addr(addr)])]
pub fn is_valid_nvic_read_addr(address: BV32) -> bool {
    // all read
    is_valid_nvic_addr(address)
}

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_nvic_write_addr(addr)])]
pub fn is_valid_nvic_write_addr(address: BV32) -> bool {
    // all write
    is_valid_nvic_addr(address)
}
