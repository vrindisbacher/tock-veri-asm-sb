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


// MPU
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

impl Mpu {
    #[flux_rs::sig(fn (&Mpu[@mpu], u32[@addr]) -> u32[mpu_addr_into_reg(addr, mpu)] requires is_valid_mpu_read_addr(addr))]
    pub fn read(&self, address: u32) -> u32 {
        // 0xE000ED90	MPU_TYPE	RO	implementation defined
        // MPU Type Register, MPU_TYPE
        // 0xE000ED94	MPU_CTRL	RW	0x00000000
        // MPU Control Register, MPU_CTRL
        // 0xE000ED98	MPU_RNR	RW	unknown
        // MPU Region Number Register, MPU_RNR
        // 0xE000ED9C	MPU_RBAR	RW	unknown
        // MPU Region Base Address Register, MPU_RBAR
        // 0xE000EDA0	MPU_RASR	RW	unknown
        // MPU Region Attribute and Size Register, MPU_RASR
        // 0xE000EDA4	MPU_RBAR_A1	RW	-
        // Alias 1 of MPU_RBAR, see MPU alias register support
        // 0xE000EDA8	MPU_RASR_A1	RW	-
        // Alias 1 of MPU_RASR, see MPU alias register support
        // 0xE000EDAC	MPU_RBAR_A2	RW	-
        // Alias 2 of MPU_RBAR, see MPU alias register support
        // 0xE000EDB0	MPU_RASR_A2	RW	-
        // Alias 2 of MPU_RASR, see MPU alias register support
        // 0xE000EDB4	MPU_RBAR_A3	RW	-
        // Alias 3 of MPU_RBAR, see MPU alias register support
        // 0xE000EDB8	MPU_RASR_A3	RW	-
        // Alias 3 of MPU_RASR, see MPU alias register support
        match address {
            MPU_TYPE_ADDR => self.mpu_type,
            MPU_CTRL_ADDR => self.mpu_ctrl,
            MPU_RNR_ADDR => self.mpu_rnr,
            MPU_RBAR_ADDR => self.mpu_rbar,
            MPU_RASR_ADDR => self.mpu_rasr,
            MPU_RBAR_A1_ADDR => self.mpu_rbar_a1,
            MPU_RASR_A1_ADDR => self.mpu_rasr_a1,
            MPU_RBAR_A2_ADDR => self.mpu_rbar_a2,
            MPU_RASR_A2_ADDR => self.mpu_rasr_a2,
            MPU_RBAR_A3_ADDR => self.mpu_rbar_a3,
            MPU_RASR_A3_ADDR => self.mpu_rasr_a3,
            // Reserved
            // 0xE000EDBC - 0xE000EDEC
            // -	...	-	Reserved.
            0xE000EDBC..=0xE000EDEC => panic!("Read of Reserved addr"),
            _ => panic!("Read of invalid addr"),
        }
    }

    #[flux_rs::sig(fn (self: &strg Mpu[@mpu], u32[@addr], u32[@val]) 
           requires is_valid_mpu_write_addr(addr) 
           ensures self: Mpu { new_mpu: mpu_addr_into_reg(addr, new_mpu) == val }
    )]
    pub fn write(&mut self, address: u32, value: u32) {
        // 0xE000ED90	MPU_TYPE	RO	implementation defined
        // MPU Type Register, MPU_TYPE
        // 0xE000ED94	MPU_CTRL	RW	0x00000000
        // MPU Control Register, MPU_CTRL
        // 0xE000ED98	MPU_RNR	RW	unknown
        // MPU Region Number Register, MPU_RNR
        // 0xE000ED9C	MPU_RBAR	RW	unknown
        // MPU Region Base Address Register, MPU_RBAR
        // 0xE000EDA0	MPU_RASR	RW	unknown
        // MPU Region Attribute and Size Register, MPU_RASR
        // 0xE000EDA4	MPU_RBAR_A1	RW	-
        // Alias 1 of MPU_RBAR, see MPU alias register support
        // 0xE000EDA8	MPU_RASR_A1	RW	-
        // Alias 1 of MPU_RASR, see MPU alias register support
        // 0xE000EDAC	MPU_RBAR_A2	RW	-
        // Alias 2 of MPU_RBAR, see MPU alias register support
        // 0xE000EDB0	MPU_RASR_A2	RW	-
        // Alias 2 of MPU_RASR, see MPU alias register support
        // 0xE000EDB4	MPU_RBAR_A3	RW	-
        // Alias 3 of MPU_RBAR, see MPU alias register support
        // 0xE000EDB8	MPU_RASR_A3	RW	-
        // Alias 3 of MPU_RASR, see MPU alias register support
        match address {
            MPU_TYPE_ADDR => panic!("Write to read only address"),
            MPU_CTRL_ADDR => self.mpu_ctrl = value,
            MPU_RNR_ADDR => self.mpu_rnr = value,
            MPU_RBAR_ADDR => self.mpu_rbar = value,
            MPU_RASR_ADDR => self.mpu_rasr = value,
            MPU_RBAR_A1_ADDR => self.mpu_rbar_a1 = value,
            MPU_RASR_A1_ADDR => self.mpu_rasr_a1 = value,
            MPU_RBAR_A2_ADDR => self.mpu_rbar_a2 = value,
            MPU_RASR_A2_ADDR => self.mpu_rasr_a2 = value,
            MPU_RBAR_A3_ADDR => self.mpu_rbar_a3 = value,
            MPU_RASR_A3_ADDR => self.mpu_rasr_a3 = value,
            // Reserved
            0xE000EDBC..=0xE000EDEC => panic!("Write to reserved addr"),
            _ => panic!("Write to invalid addr"),
        }
    }
}
