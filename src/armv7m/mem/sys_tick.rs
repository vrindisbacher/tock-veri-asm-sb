// Sys Tick
//
// Here are some unimplemented blocks:
//
// 0xE000E020- 0xE000E0FC	-	-	-	Reserved
//

use crate::flux_support::b32::{from, B32};

use super::flux_defs::sys_tick_defs::*;

pub const SYST_CSR_ADDR: B32 = from(0xE000E010);
pub const SYST_RVR_ADDR: B32 = from(0xE000E014);
pub const SYST_CVR_ADDR: B32 = from(0xE000E018);
pub const SYST_CALIB_ADDR: B32 = from(0xE000E01C);

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_sys_tick_read_addr(addr)])]
pub fn is_valid_sys_tick_read_addr(address: B32) -> bool {
    // all addresses are read
    address == SYST_CSR_ADDR
        || address == SYST_RVR_ADDR
        || address == SYST_CVR_ADDR
        || address == SYST_CALIB_ADDR
}

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_sys_tick_write_addr(addr)])]
pub fn is_valid_sys_tick_write_addr(address: B32) -> bool {
    // all addresses but SYS_CALIB are write
    address == SYST_CSR_ADDR || address == SYST_RVR_ADDR || address == SYST_CVR_ADDR
}
