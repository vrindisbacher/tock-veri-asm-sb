// System Control Block (see table 10.5)
//
// Columns are:
// Address	Name	Type	Reset	Description
//
// Here are some reserved spaces:
// 0xE000ED40 - 0xE000ED7C	-	-	-	Reserved for CPUID registers, see The CPUID Scheme.
// 0xE000ED80 - 0xE000ED84	-	-	-	Reserved.
// 0xE000ED8C	-	-	-	Reserved.
//
// System Control ID Registers that aren't in the the system control block (see table 10.6)
//
// Here are some unimplemented blocks:
//
// 0xE000E000	-	RW	0x00000000	Master Control register, Reserved
// 0xE000E00C	-	-	-	Reserved
// 0xE000EF04 - 0xE000EF8C	-	-	-	Reserved
// 0xE000EF90 - 0xE000EFCC	...	...	...	implementation defined
// 0xE000EDF0 - 0xE000EEFC	-	-	-	See Debug register support in the SCS
//

use crate::flux_support::b32::from;
use crate::flux_support::b32::B32;

use super::flux_defs::sys_control_block_defs::*;
use super::flux_defs::sys_control_id_reg_defs::*;

// System Control Block Addresses
pub const CPUID_ADDR: B32 = from(0xE000ED00);
pub const ICSR_ADDR: B32 = from(0xE000ED04);
pub const VTOR_ADDR: B32 = from(0xE000ED08);
pub const AIRCR_ADDR: B32 = from(0xE000ED0C);
pub const SCR_ADDR: B32 = from(0xE000ED10);
pub const CCR_ADDR: B32 = from(0xE000ED14);
pub const SHPR1_ADDR: B32 = from(0xE000ED18);
pub const SHPR2_ADDR: B32 = from(0xE000ED1C);
pub const SHPR3_ADDR: B32 = from(0xE000ED20);
pub const SHCSR_ADDR: B32 = from(0xE000ED24);
pub const CFSR_ADDR: B32 = from(0xE000ED28);
pub const HFSR_ADDR: B32 = from(0xE000ED2C);
pub const DFSR_ADDR: B32 = from(0xE000ED30);
pub const MMFAR_ADDR: B32 = from(0xE000ED34);
pub const BFAR_ADDR: B32 = from(0xE000ED38);
pub const AFSR_ADDR: B32 = from(0xE000ED3C);
pub const CPACR_ADDR: B32 = from(0xE000ED88);

// ID Reg
pub const ICTR_ADDR: B32 = from(0xE000E004);
pub const ACTLR_ADDR: B32 = from(0xE000E008);
pub const STIR_ADDR: B32 = from(0xE000EF00);
pub const PID4_ADDR: B32 = from(0xE000EFD0);
pub const PID5_ADDR: B32 = from(0xE000EFD4);
pub const PID6_ADDR: B32 = from(0xE000EFD8);
pub const PID7_ADDR: B32 = from(0xE000EFDC);
pub const PID0_ADDR: B32 = from(0xE000EFE0);
pub const PID1_ADDR: B32 = from(0xE000EFE4);
pub const PID2_ADDR: B32 = from(0xE000EFE8);
pub const PID3_ADDR: B32 = from(0xE000EFEC);
pub const CID0_ADDR: B32 = from(0xE000EFF0);
pub const CID1_ADDR: B32 = from(0xE000EFF4);
pub const CID2_ADDR: B32 = from(0xE000EFF8);
pub const CID3_ADDR: B32 = from(0xE000EFFC);

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_sys_control_block_read_addr(addr)])]
fn is_valid_sys_control_block_read_addr(address: B32) -> bool {
    // all addresses are read
    address == CPUID_ADDR
        || address == ICSR_ADDR
        || address == VTOR_ADDR
        || address == AIRCR_ADDR
        || address == SCR_ADDR
        || address == CCR_ADDR
        || address == SHPR1_ADDR
        || address == SHPR2_ADDR
        || address == SHPR3_ADDR
        || address == SHCSR_ADDR
        || address == CFSR_ADDR
        || address == HFSR_ADDR
        || address == DFSR_ADDR
        || address == MMFAR_ADDR
        || address == BFAR_ADDR
        || address == AFSR_ADDR
        || address == CPACR_ADDR
}

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_sys_control_block_write_addr(addr)])]
fn is_valid_sys_control_block_write_addr(address: B32) -> bool {
    // all addresses but CPUID are write
    address == ICSR_ADDR
        || address == VTOR_ADDR
        || address == AIRCR_ADDR
        || address == SCR_ADDR
        || address == CCR_ADDR
        || address == SHPR1_ADDR
        || address == SHPR2_ADDR
        || address == SHPR3_ADDR
        || address == SHCSR_ADDR
        || address == CFSR_ADDR
        || address == HFSR_ADDR
        || address == DFSR_ADDR
        || address == MMFAR_ADDR
        || address == BFAR_ADDR
        || address == AFSR_ADDR
        || address == CPACR_ADDR
}

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_sys_control_id_reg_read_addr(addr)])]
fn is_valid_sys_control_id_reg_read_addr(address: B32) -> bool {
    // all but STIR are read
    address == ICTR_ADDR
        || address == ACTLR_ADDR
        || address == PID4_ADDR
        || address == PID5_ADDR
        || address == PID6_ADDR
        || address == PID7_ADDR
        || address == PID0_ADDR
        || address == PID1_ADDR
        || address == PID2_ADDR
        || address == PID3_ADDR
        || address == CID0_ADDR
        || address == CID1_ADDR
        || address == CID2_ADDR
        || address == CID3_ADDR
}

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_sys_control_id_reg_write_addr(addr)])]
fn is_valid_sys_control_id_reg_write_addr(address: B32) -> bool {
    // only actlr && stir are write
    address == ACTLR_ADDR || address == STIR_ADDR
}

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_sys_control_space_read_addr(addr)])]
pub fn is_valid_sys_control_space_read_addr(address: B32) -> bool {
    is_valid_sys_control_block_read_addr(address) || is_valid_sys_control_id_reg_read_addr(address)
}

#[flux_rs::sig(fn (B32[@addr]) -> bool[is_valid_sys_control_space_write_addr(addr)])]
pub fn is_valid_sys_control_space_write_addr(address: B32) -> bool {
    is_valid_sys_control_block_write_addr(address)
        || is_valid_sys_control_id_reg_write_addr(address)
}
