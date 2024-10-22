pub const SYST_CSR_ADDR: u32 = 0xE000E010;
pub const SYST_RVR_ADDR: u32 = 0xE000E014;
pub const SYST_CVR_ADDR: u32 = 0xE000E018;
pub const SYST_CALIB_ADDR: u32 = 0xE000E01C;

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

    pub fn write(&mut self, address: u32, value: u32) {
        // 0xE000E010	SYST_CSR	RW	0x0000000x [a]	SysTick Control and Status Register, SYST_CSR
        // 0xE000E014	SYST_RVR	RW	unknown	SysTick Reload Value Register, SYST_RVR
        // 0xE000E018	SYST_CVR	RW	unknown	SysTick Current Value Register, SYST_CVR
        // 0xE000E01C	SYST_CALIB	RO	IMP DEF	SysTick Calibration value Register, SYST_CALIB
        let reg = match address {
            SYST_CSR_ADDR => &mut self.syst_csr,
            SYST_RVR_ADDR => &mut self.syst_rvr,
            SYST_CVR_ADDR => &mut self.syst_cvr,
            SYST_CALIB_ADDR => panic!("Write of read only reg"),
            // RESERVED
            // 0xE000E020- 0xE000E0FC	-	-	-	Reserved
            0xE000E020..=0xE000E0FC => panic!("Write to Reserved addr"),
            _ => panic!("Write to invalid addr"),
        };
        *reg = value;
    }
}
