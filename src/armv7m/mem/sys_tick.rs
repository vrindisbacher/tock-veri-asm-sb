// Sys Tick
//
// Here are some unimplemented blocks:
//
// 0xE000E020- 0xE000E0FC	-	-	-	Reserved
//

use flux_rs::bitvec::BV32;

use super::flux_defs::sys_tick_defs::*;

#[flux_rs::constant(bv_int_to_bv32(0xE000E010))]
pub const SYST_CSR_ADDR: BV32 = BV32::new(0xE000E010);
#[flux_rs::constant(bv_int_to_bv32(0xE000E014))]
pub const SYST_RVR_ADDR: BV32 = BV32::new(0xE000E014);
#[flux_rs::constant(bv_int_to_bv32(0xE000E018))]
pub const SYST_CVR_ADDR: BV32 = BV32::new(0xE000E018);
#[flux_rs::constant(bv_int_to_bv32(0xE000E01C))]
pub const SYST_CALIB_ADDR: BV32 = BV32::new(0xE000E01C);

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_sys_tick_read_addr(addr)])]
pub fn is_valid_sys_tick_read_addr(address: BV32) -> bool {
    // all addresses are read
    let a1 = SYST_CSR_ADDR;
    let a2 = SYST_RVR_ADDR;
    let a3 = SYST_CVR_ADDR;
    let a4 = SYST_CALIB_ADDR;
    address == a1 || address == a2 || address == a3 || address == a4
}

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_sys_tick_write_addr(addr)])]
pub fn is_valid_sys_tick_write_addr(address: BV32) -> bool {
    // all addresses but SYS_CALIB are write
    let a1 = SYST_CSR_ADDR;
    let a2 = SYST_RVR_ADDR;
    let a3 = SYST_CVR_ADDR;
    address == a1 || address == a2 || address == a3
}
