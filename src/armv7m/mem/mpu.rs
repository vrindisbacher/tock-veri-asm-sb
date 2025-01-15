// MPU
//
// Here are some unimplemented blocks:
// 0xE000EDBC - 0xE000EDEC
// -	...	-	Reserved.

use super::flux_defs::mpu_defs::*;
use flux_rs::bitvec::BV32;

#[flux_rs::constant(bv_int_to_bv32(0xE000ED90))]
pub const MPU_TYPE_ADDR: BV32 = BV32::new(0xE000ED90);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED94))]
pub const MPU_CTRL_ADDR: BV32 = BV32::new(0xE000ED94);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED98))]
pub const MPU_RNR_ADDR: BV32 = BV32::new(0xE000ED98);
#[flux_rs::constant(bv_int_to_bv32(0xE000ED9C))]
pub const MPU_RBAR_ADDR: BV32 = BV32::new(0xE000ED9C);
#[flux_rs::constant(bv_int_to_bv32(0xE000EDA0))]
pub const MPU_RASR_ADDR: BV32 = BV32::new(0xE000EDA0);
#[flux_rs::constant(bv_int_to_bv32(0xE000EDA4))]
pub const MPU_RBAR_A1_ADDR: BV32 = BV32::new(0xE000EDA4);
#[flux_rs::constant(bv_int_to_bv32(0xE000EDA8))]
pub const MPU_RASR_A1_ADDR: BV32 = BV32::new(0xE000EDA8);
#[flux_rs::constant(bv_int_to_bv32(0xE000EDAC))]
pub const MPU_RBAR_A2_ADDR: BV32 = BV32::new(0xE000EDAC);
#[flux_rs::constant(bv_int_to_bv32(0xE000EDB0))]
pub const MPU_RASR_A2_ADDR: BV32 = BV32::new(0xE000EDB0);
#[flux_rs::constant(bv_int_to_bv32(0xE000EDB4))]
pub const MPU_RBAR_A3_ADDR: BV32 = BV32::new(0xE000EDB4);
#[flux_rs::constant(bv_int_to_bv32(0xE000EDB8))]
pub const MPU_RASR_A3_ADDR: BV32 = BV32::new(0xE000EDB8);

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_mpu_read_addr(addr)])]
pub fn is_valid_mpu_read_addr(address: BV32) -> bool {
    // all address are read
    let a1 = MPU_TYPE_ADDR;
    let a2 = MPU_CTRL_ADDR;
    let a3 = MPU_RNR_ADDR;
    let a4 = MPU_RBAR_ADDR;
    let a5 = MPU_RASR_ADDR;
    let a6 = MPU_RBAR_A1_ADDR;
    let a7 = MPU_RASR_A1_ADDR;
    let a8 = MPU_RBAR_A2_ADDR;
    let a9 = MPU_RASR_A2_ADDR;
    let a10 = MPU_RBAR_A3_ADDR;
    let a11 = MPU_RASR_A3_ADDR;
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
}

#[flux_rs::sig(fn (BV32[@addr]) -> bool[is_valid_mpu_write_addr(addr)])]
pub fn is_valid_mpu_write_addr(address: BV32) -> bool {
    // all address except MPU_TYPE are write
    let a1 = MPU_CTRL_ADDR;
    let a2 = MPU_RNR_ADDR;
    let a3 = MPU_RBAR_ADDR;
    let a4 = MPU_RASR_ADDR;
    let a5 = MPU_RBAR_A1_ADDR;
    let a6 = MPU_RASR_A1_ADDR;
    let a7 = MPU_RBAR_A2_ADDR;
    let a8 = MPU_RASR_A2_ADDR;
    let a9 = MPU_RBAR_A3_ADDR;
    let a10 = MPU_RASR_A3_ADDR;
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
}
