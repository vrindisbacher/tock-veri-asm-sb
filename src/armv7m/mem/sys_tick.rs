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
