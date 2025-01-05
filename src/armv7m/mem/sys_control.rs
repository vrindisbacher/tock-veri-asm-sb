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

use flux_rs::bitvec::BV32;

use super::flux_defs::sys_control_block_defs::*;
use super::flux_defs::sys_control_id_reg_defs::*;

// System Control Block Addresses
#[flux_rs::constant(bv_int_to_bv32(0xE000ED00))]
pub const CPUID_ADDR: BV32 = BV32::new(0xE000ED00);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED04))]
pub const ICSR_ADDR: BV32 = BV32::new(0xE000ED04);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED08))]
pub const VTOR_ADDR: BV32 = BV32::new(0xE000ED08);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED0C))]
pub const AIRCR_ADDR: BV32 = BV32::new(0xE000ED0C);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED10))]
pub const SCR_ADDR: BV32 = BV32::new(0xE000ED10);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED14))]
pub const CCR_ADDR: BV32 = BV32::new(0xE000ED14);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED18))]
pub const SHPR1_ADDR: BV32 = BV32::new(0xE000ED18);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED1C))]
pub const SHPR2_ADDR: BV32 = BV32::new(0xE000ED1C);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED20))]
pub const SHPR3_ADDR: BV32 = BV32::new(0xE000ED20);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED24))]
pub const SHCSR_ADDR: BV32 = BV32::new(0xE000ED24);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED28))]
pub const CFSR_ADDR: BV32 = BV32::new(0xE000ED28);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED2C))]
pub const HFSR_ADDR: BV32 = BV32::new(0xE000ED2C);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED30))]
pub const DFSR_ADDR: BV32 = BV32::new(0xE000ED30);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED34))]
pub const MMFAR_ADDR: BV32 = BV32::new(0xE000ED34);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED34))]
pub const BFAR_ADDR: BV32 = BV32::new(0xE000ED38);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED3C))]
pub const AFSR_ADDR: BV32 = BV32::new(0xE000ED3C);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED88))]
pub const CPACR_ADDR: BV32 = BV32::new(0xE000ED88);

// ID Reg
#[flux_rs::constant(bv_int_to_bv32(0xE000E004))]
pub const ICTR_ADDR: BV32 = BV32::new(0xE000E004);
#[flux_rs::constant(bv_int_to_bv32(0xE000E004))]
pub const ACTLR_ADDR: BV32 = BV32::new(0xE000E008);
#[flux_rs::constant(bv_int_to_bv32(0xE000EF00))]
pub const STIR_ADDR: BV32 = BV32::new(0xE000EF00);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFD0))]
pub const PID4_ADDR: BV32 = BV32::new(0xE000EFD0);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFD4))]
pub const PID5_ADDR: BV32 = BV32::new(0xE000EFD4);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFD8))]
pub const PID6_ADDR: BV32 = BV32::new(0xE000EFD8);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFDC))]
pub const PID7_ADDR: BV32 = BV32::new(0xE000EFDC);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFE0))]
pub const PID0_ADDR: BV32 = BV32::new(0xE000EFE0);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFE4))]
pub const PID1_ADDR: BV32 = BV32::new(0xE000EFE4);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFE8))]
pub const PID2_ADDR: BV32 = BV32::new(0xE000EFE8);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFEC))]
pub const PID3_ADDR: BV32 = BV32::new(0xE000EFEC);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFF0))]
pub const CID0_ADDR: BV32 = BV32::new(0xE000EFF0);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFF4))]
pub const CID1_ADDR: BV32 = BV32::new(0xE000EFF4);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFF8))]
pub const CID2_ADDR: BV32 = BV32::new(0xE000EFF8);
#[flux_rs::constant(bv_int_to_bv32(0xE000EFFC))]
pub const CID3_ADDR: BV32 = BV32::new(0xE000EFFC);

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_sys_control_block_read_addr(addr)])]
fn is_valid_sys_control_block_read_addr(address: BV32) -> bool {
    // all addresses are read
    let a1 = CPUID_ADDR;
    let a2 = ICSR_ADDR;
    let a3 = VTOR_ADDR;
    let a4 = AIRCR_ADDR;
    let a5 = SCR_ADDR;
    let a6 = CCR_ADDR;
    let a7 = SHPR1_ADDR;
    let a8 = SHPR2_ADDR;
    let a9 = SHPR3_ADDR;
    let a10 = SHCSR_ADDR;
    let a11 = CFSR_ADDR;
    let a12 = HFSR_ADDR;
    let a13 = DFSR_ADDR;
    let a14 = MMFAR_ADDR;
    let a15 = BFAR_ADDR;
    let a16 = AFSR_ADDR;
    let a17 = CPACR_ADDR;

    address == a1
        || address == a2
        || address == a3
        || address == a4
        || address == a5
        || address == a6
        || address == a7
        || address == a8
        || address == a9
        || address == a10
        || address == a11
        || address == a12
        || address == a13
        || address == a14
        || address == a15
        || address == a16
        || address == a17
}

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_sys_control_block_write_addr(addr)])]
fn is_valid_sys_control_block_write_addr(address: BV32) -> bool {
    // all addresses but CPUID are write
    let a2 = ICSR_ADDR;
    let a3 = VTOR_ADDR;
    let a4 = AIRCR_ADDR;
    let a5 = SCR_ADDR;
    let a6 = CCR_ADDR;
    let a7 = SHPR1_ADDR;
    let a8 = SHPR2_ADDR;
    let a9 = SHPR3_ADDR;
    let a10 = SHCSR_ADDR;
    let a11 = CFSR_ADDR;
    let a12 = HFSR_ADDR;
    let a13 = DFSR_ADDR;
    let a14 = MMFAR_ADDR;
    let a15 = BFAR_ADDR;
    let a16 = AFSR_ADDR;
    let a17 = CPACR_ADDR;
    address == a2
        || address == a3
        || address == a4
        || address == a5
        || address == a6
        || address == a7
        || address == a8
        || address == a9
        || address == a10
        || address == a11
        || address == a12
        || address == a13
        || address == a14
        || address == a15
        || address == a16
        || address == a17
}

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_sys_control_id_reg_read_addr(addr)])]
fn is_valid_sys_control_id_reg_read_addr(address: BV32) -> bool {
    // all but STIR are read
    let a1 = ICTR_ADDR;
    let a2 = ACTLR_ADDR;
    let a3 = PID4_ADDR;
    let a4 = PID5_ADDR;
    let a5 = PID6_ADDR;
    let a6 = PID7_ADDR;
    let a7 = PID0_ADDR;
    let a8 = PID1_ADDR;
    let a9 = PID2_ADDR;
    let a10 = PID3_ADDR;
    let a11 = CID0_ADDR;
    let a12 = CID1_ADDR;
    let a13 = CID2_ADDR;
    let a14 = CID3_ADDR;
    address == a1
        || address == a2
        || address == a3
        || address == a4
        || address == a5
        || address == a6
        || address == a7
        || address == a8
        || address == a9
        || address == a10
        || address == a11
        || address == a12
        || address == a13
        || address == a14
}

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_sys_control_id_reg_write_addr(addr)])]
fn is_valid_sys_control_id_reg_write_addr(address: BV32) -> bool {
    // only actlr && stir are write
    let actlr_addr = ACTLR_ADDR;
    let stir_addr = STIR_ADDR;
    address == actlr_addr || address == stir_addr
}

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_sys_control_space_read_addr(addr)])]
pub fn is_valid_sys_control_space_read_addr(address: BV32) -> bool {
    is_valid_sys_control_block_read_addr(address) || is_valid_sys_control_id_reg_read_addr(address)
}

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_sys_control_space_write_addr(addr)])]
pub fn is_valid_sys_control_space_write_addr(address: BV32) -> bool {
    is_valid_sys_control_block_write_addr(address)
        || is_valid_sys_control_id_reg_write_addr(address)
}
