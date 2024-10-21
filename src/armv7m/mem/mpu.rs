// MPU: https://developer.arm.com/documentation/ddi0403/d/System-Level-Architecture/System-Address-Map/Protected-Memory-System-Architecture--PMSAv7/Register-support-for-PMSAv7-in-the-SCS?lang=en 
// 
// Here are some unimplemented blocks:
// 0xE000EDBC - 0xE000EDEC
// -	...	-	Reserved.
//
// Columns are:
// Address	Name	Type	Reset	Description
//
#[derive(Debug)]
#[flux_rs::refined_by(
    mpu_type: int,
    mpu_ctrl: int,
    mpu_rnr: int,
    mpu_rbar: int,
    mpu_rasr: int,
    mpu_rbar_a1: int,
    mpu_rasr_a1: int,
    mpu_rbar_a2: int,
    mpu_rasr_a2: int,
    mpu_rbar_a3: int,
    mpu_rasr_a3: int
)]
pub struct Mpu {
    // 0xE000ED90	MPU_TYPE	RO	implementation defined	
    // MPU Type Register, MPU_TYPE
    #[field(u32[mpu_type])]
    mpu_type: u32,
    // 0xE000ED94	MPU_CTRL	RW	0x00000000	
    // MPU Control Register, MPU_CTRL
    #[field(u32[mpu_ctrl])]
    mpu_ctrl: u32,
    // 0xE000ED98	MPU_RNR	RW	unknown	
    // MPU Region Number Register, MPU_RNR
    #[field(u32[mpu_rnr])]
    mpu_rnr: u32,
    // 0xE000ED9C	MPU_RBAR	RW	unknown	
    // MPU Region Base Address Register, MPU_RBAR
    #[field(u32[mpu_rbar])]
    mpu_rbar: u32,
    // 0xE000EDA0	MPU_RASR	RW	unknown	
    // MPU Region Attribute and Size Register, MPU_RASR
    #[field(u32[mpu_rasr])]
    mpu_rasr: u32,
    // 0xE000EDA4	MPU_RBAR_A1	RW	-	
    // Alias 1 of MPU_RBAR, see MPU alias register support
    #[field(u32[mpu_rbar_a1])]
    mpu_rbar_a1: u32,
    // 0xE000EDA8	MPU_RASR_A1	RW	-	
    // Alias 1 of MPU_RASR, see MPU alias register support
    #[field(u32[mpu_rasr_a1])]
    mpu_rasr_a1: u32,
    // 0xE000EDAC	MPU_RBAR_A2	RW	-	
    // Alias 2 of MPU_RBAR, see MPU alias register support
    #[field(u32[mpu_rbar_a2])]
    mpu_rbar_a2: u32,
    // 0xE000EDB0	MPU_RASR_A2	RW	-	
    // Alias 2 of MPU_RASR, see MPU alias register support
    #[field(u32[mpu_rasr_a2])]
    mpu_rasr_a2: u32,
    // 0xE000EDB4	MPU_RBAR_A3	RW	-	
    // Alias 3 of MPU_RBAR, see MPU alias register support
    #[field(u32[mpu_rbar_a3])]
    mpu_rbar_a3: u32,
    // 0xE000EDB8	MPU_RASR_A3	RW	-	
    // Alias 3 of MPU_RASR, see MPU alias register support
    #[field(u32[mpu_rasr_a3])]
    mpu_rasr_a3: u32,
}
