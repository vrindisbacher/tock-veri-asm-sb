// MPU
//
// Here are some unimplemented blocks:
// 0xE000EDBC - 0xE000EDEC
// -	...	-	Reserved.

use super::flux_defs::mpu_defs::*;

pub const MPU_TYPE_ADDR: u32 = 0xE000ED90;
pub const MPU_CTRL_ADDR: u32 = 0xE000ED94;
pub const MPU_RNR_ADDR: u32 = 0xE000ED98;
pub const MPU_RBAR_ADDR: u32 = 0xE000ED9C;
pub const MPU_RASR_ADDR: u32 = 0xE000EDA0;
pub const MPU_RBAR_A1_ADDR: u32 = 0xE000EDA4;
pub const MPU_RASR_A1_ADDR: u32 = 0xE000EDA8;
pub const MPU_RBAR_A2_ADDR: u32 = 0xE000EDAC;
pub const MPU_RASR_A2_ADDR: u32 = 0xE000EDB0;
pub const MPU_RBAR_A3_ADDR: u32 = 0xE000EDB4;
pub const MPU_RASR_A3_ADDR: u32 = 0xE000EDB8;

#[flux_rs::sig(fn (u32[@addr]) -> bool[is_valid_mpu_read_addr(addr)])]
pub fn is_valid_mpu_read_addr(address: u32) -> bool {
    // all address are read
    address == MPU_TYPE_ADDR
        || address == MPU_CTRL_ADDR
        || address == MPU_RNR_ADDR
        || address == MPU_RBAR_ADDR
        || address == MPU_RASR_ADDR
        || address == MPU_RBAR_A1_ADDR
        || address == MPU_RASR_A1_ADDR
        || address == MPU_RBAR_A2_ADDR
        || address == MPU_RASR_A2_ADDR
        || address == MPU_RBAR_A3_ADDR
        || address == MPU_RASR_A3_ADDR
}

#[flux_rs::sig(fn (u32[@addr]) -> bool[is_valid_mpu_write_addr(addr)])]
pub fn is_valid_mpu_write_addr(address: u32) -> bool {
    // all address except MPU_TYPE are write
    address == MPU_CTRL_ADDR
        || address == MPU_RNR_ADDR
        || address == MPU_RBAR_ADDR
        || address == MPU_RASR_ADDR
        || address == MPU_RBAR_A1_ADDR
        || address == MPU_RASR_A1_ADDR
        || address == MPU_RBAR_A2_ADDR
        || address == MPU_RASR_A2_ADDR
        || address == MPU_RBAR_A3_ADDR
        || address == MPU_RASR_A3_ADDR
}
