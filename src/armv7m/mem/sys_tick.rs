use super::flux_defs::sys_tick_defs::*;

pub const SYST_CSR_ADDR: u32 = 0xE000E010;
pub const SYST_RVR_ADDR: u32 = 0xE000E014;
pub const SYST_CVR_ADDR: u32 = 0xE000E018;
pub const SYST_CALIB_ADDR: u32 = 0xE000E01C;

#[flux_rs::sig(fn (u32[@addr]) -> bool[is_valid_sys_tick_read_addr(addr)])]
pub fn is_valid_sys_tick_read_addr(address: u32) -> bool {
    // all addresses are read
    address == SYST_CSR_ADDR
        || address == SYST_RVR_ADDR
        || address == SYST_CVR_ADDR
        || address == SYST_CALIB_ADDR
}

#[flux_rs::sig(fn (u32[@addr]) -> bool[is_valid_sys_tick_write_addr(addr)])]
pub fn is_valid_sys_tick_write_addr(address: u32) -> bool {
    // all addresses but SYS_CALIB are write
    address == SYST_CSR_ADDR || address == SYST_RVR_ADDR || address == SYST_CVR_ADDR
}

// Sys Tick: https://developer.arm.com/documentation/ddi0403/d/System-Level-Architecture/System-Address-Map/The-system-timer--SysTick/System-timer-register-support-in-the-SCS?lang=en
//
// Here are some unimplemented blocks:
//
// 0xE000E020- 0xE000E0FC	-	-	-	Reserved
//
// Columns are:
//
// Address	Name	Type	Reset	Description
//
#[derive(Debug)]
#[flux_rs::refined_by(
    syst_csr: int,
    syst_rvr: int,
    syst_cvr: int,
    syst_calib: int
)]
pub struct SysTick {
    // 0xE000E010	SYST_CSR	RW	0x0000000x [a]	SysTick Control and Status Register, SYST_CSR
    #[field(u32[syst_csr])]
    syst_csr: u32,
    // 0xE000E014	SYST_RVR	RW	unknown	SysTick Reload Value Register, SYST_RVR
    #[field(u32[syst_rvr])]
    syst_rvr: u32,
    // 0xE000E018	SYST_CVR	RW	unknown	SysTick Current Value Register, SYST_CVR
    #[field(u32[syst_cvr])]
    syst_cvr: u32,
    // 0xE000E01C	SYST_CALIB	RO	IMP DEF	SysTick Calibration value Register, SYST_CALIB
    #[field(u32[syst_calib])]
    syst_calib: u32,
}

impl SysTick {
    #[flux_rs::sig(fn (&SysTick[@sys_tick],  u32[@addr]) -> u32[sys_tick_addr_into_reg(addr, sys_tick)] 
           requires is_valid_sys_tick_read_addr(addr)
    )]
    pub fn read(&self, address: u32) -> u32 {
        // 0xE000E010	SYST_CSR	RW	0x0000000x [a]	SysTick Control and Status Register, SYST_CSR
        // 0xE000E014	SYST_RVR	RW	unknown	SysTick Reload Value Register, SYST_RVR
        // 0xE000E018	SYST_CVR	RW	unknown	SysTick Current Value Register, SYST_CVR
        // 0xE000E01C	SYST_CALIB	RO	IMP DEF	SysTick Calibration value Register, SYST_CALIB
        match address {
            SYST_CSR_ADDR => self.syst_csr,
            SYST_RVR_ADDR => self.syst_rvr,
            SYST_CVR_ADDR => self.syst_cvr,
            SYST_CALIB_ADDR => self.syst_calib,
            // RESERVED
            // 0xE000E020- 0xE000E0FC	-	-	-	Reserved
            0xE000E020..=0xE000E0FC => panic!("Read of Reserved addr"),
            _ => panic!("Read of invalid addr"),
        }
    }

    #[flux_rs::sig(fn (self: &strg SysTick[@sys_tick],  u32[@addr], u32[@val])
           requires is_valid_sys_tick_write_addr(addr)
           ensures self: SysTick { new_sys_tick: sys_tick_addr_into_reg(addr, new_sys_tick) == val }
    )]
    pub fn write(&mut self, address: u32, value: u32) {
        // 0xE000E010	SYST_CSR	RW	0x0000000x [a]	SysTick Control and Status Register, SYST_CSR
        // 0xE000E014	SYST_RVR	RW	unknown	SysTick Reload Value Register, SYST_RVR
        // 0xE000E018	SYST_CVR	RW	unknown	SysTick Current Value Register, SYST_CVR
        // 0xE000E01C	SYST_CALIB	RO	IMP DEF	SysTick Calibration value Register, SYST_CALIB
        match address {
            SYST_CSR_ADDR => self.syst_csr = value,
            SYST_RVR_ADDR => self.syst_rvr = value,
            SYST_CVR_ADDR => self.syst_cvr = value,
            SYST_CALIB_ADDR => panic!("Write of read only reg"),
            // RESERVED
            // 0xE000E020- 0xE000E0FC	-	-	-	Reserved
            0xE000E020..=0xE000E0FC => panic!("Write to Reserved addr"),
            _ => panic!("Write to invalid addr"),
        };
    }
}
