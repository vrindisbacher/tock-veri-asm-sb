// System Control Block (see table 10.5)
//
// Columns are:
// Address	Name	Type	Reset	Description
//
// Here are some reserved spaces:
// 0xE000ED40 - 0xE000ED7C	-	-	-	Reserved for CPUID registers, see The CPUID Scheme.
// 0xE000ED80 - 0xE000ED84	-	-	-	Reserved.
// 0xE000ED8C	-	-	-	Reserved.
#[derive(Debug)]
#[flux_rs::refined_by(
    cpuid: int,    
    icsr: int,    
    vtor: int,    
    aircr: int,    
    scr: int,    
    ccr: int,    
    shpr1: int,    
    shpr2: int,    
    shpr3: int,    
    shcsr: int,    
    cfsr: int,    
    hfsr: int,    
    dfsr: int,    
    mmfar: int,    
    bfar: int,    
    afsr: int,    
    cpacr: int
)]
pub struct SysControlBlock {
    // 0xE000ED00	CPUID	RO	implementation defined	CPUID Base Register.
    #[field(u32[cpuid])]
    cpuid: u32,
    // 0xE000ED04	ICSR	RW	0x00000000	Interrupt Control and State Register, ICSR.
    #[field(u32[icsr])]
    icsr: u32,
    // 0xE000ED08	VTOR	RW	0x00000000[a]	Vector Table Offset Register, VTOR.
    #[field(u32[vtor])]
    vtor: u32,
    // 0xE000ED0C	AIRCR	RW	[b]	Application Interrupt and Reset Control Register, AIRCR.
    #[field(u32[aircr])]
    aircr: u32,
    // 0xE000ED10	SCR	RW	0x00000000	System Control Register, SCR.
    #[field(u32[scr])]
    scr: u32,
    // 0xE000ED14	CCR	RW	0x00000000	Configuration and Control Register, CCR.
    #[field(u32[ccr])]
    ccr: u32,
    // 0xE000ED18	SHPR1	RW	0x00000000	System Handler Priority Register 1, SHPR1.
    #[field(u32[shpr1])]
    shpr1: u32,
    // 0xE000ED1C	SHPR2	RW	0x00000000	System Handler Priority Register 2, SHPR2.
    #[field(u32[shpr2])]
    shpr2: u32,
    // 0xE000ED20	SHPR3	RW	0x00000000	System Handler Priority Register 3, SHPR3.
    #[field(u32[shpr3])]
    shpr3: u32,
    // 0xE000ED24	SHCSR	RW	0x00000000	System Handler Control and State Register, SHCSR.
    #[field(u32[shcsr])]
    shcsr: u32,
    // 0xE000ED28	CFSR	RW	0x00000000
    // Configurable Fault Status Register, CFSR.
    //
    // The following describe the subregisters of the CFSR:
    //
    // MemManage Status Register, MMFSR
    //
    // BusFault Status Register, BFSR
    //
    // UsageFault Status Register, UFSR.
    #[field(u32[cfsr])]
    cfsr: u32,
    // 0xE000ED2C	HFSR	RW	0x00000000	HardFault Status Register, HFSR.
    #[field(u32[hfsr])]
    hfsr: u32,
    // 0xE000ED30	DFSR	RW	0x00000000[c]	Debug Fault Status Register, DFSR.
    #[field(u32[dfsr])]
    dfsr: u32,
    // 0xE000ED34	MMFAR	RW	unknown	MemManage Fault Address Register, MMFAR.
    #[field(u32[mmfar])]
    mmfar: u32,
    // 0xE000ED38	BFAR	RW	unknown	BusFault Address Register, BFAR.
    #[field(u32[bfar])]
    bfar: u32,
    // 0xE000ED3C	AFSR	RW	unknown	Auxiliary Fault Status Register, AFSR, implementation defined.
    #[field(u32[afsr])]
    afsr: u32,
    // 0xE000ED88	CPACR	RW	unknown	Coprocessor Access Control Register, CPACR.
    #[field(u32[cpacr])]
    cpacr: u32,
}

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
// Columns are:
// Address	Name	Type	Reset	Description
#[derive(Debug)]
#[flux_rs::refined_by(
    ictr: int,    
    actlr: int,    
    stir: int,    
    pid4: int,    
    pid5: int,    
    pid6: int,    
    pid7: int,    
    pid0: int,    
    pid1: int,    
    pid2: int,    
    pid3: int,    
    cid0: int,    
    cid1: int,    
    cid2: int,    
    cid3: int,
)]
pub struct SysControlIDReg {
    // 0xE000E004	ICTR	RO	implementation defined	Interrupt Controller Type Register, ICTR
    #[field(u32[ictr])]
    ictr: u32,
    // 0xE000E008	ACTLR	RW	implementation defined	Auxiliary Control Register, ACTLR
    #[field(u32[actlr])]
    actlr: u32,
    // 0xE000EF00	STIR	WO	-	Software Triggered Interrupt Register, STIR
    #[field(u32[stir])]
    stir: u32,
    // 0xE000EFD0	PID4	RO	[a]	Peripheral Identification Registers, see The CPUID Scheme
    #[field(u32[pid4])]
    pid4: u32,
    // 0xE000EFD4	PID5	RO
    #[field(u32[pid5])]
    pid5: u32,
    // 0xE000EFD8	PID6	RO
    #[field(u32[pid6])]
    pid6: u32,
    // 0xE000EFDC	PID7	RO
    #[field(u32[pid7])]
    pid7: u32,
    // 0xE000EFE0	PID0	RO
    #[field(u32[pid0])]
    pid0: u32,
    // 0xE000EFE4	PID1	RO
    #[field(u32[pid1])]
    pid1: u32,
    // 0xE000EFE8	PID2	RO
    #[field(u32[pid2])]
    pid2: u32,
    // 0xE000EFEC	PID3	RO
    #[field(u32[pid3])]
    pid3: u32,
    // 0xE000EFF0	CID0	RO	[a]	Component Identification Registers, see The CPUID Scheme
    #[field(u32[cid0])]
    cid0: u32,
    // 0xE000EFF4	CID1	RO
    #[field(u32[cid1])]
    cid1: u32,
    // 0xE000EFF8	CID2	RO
    #[field(u32[cid2])]
    cid2: u32,
    // 0xE000EFFC	CID3	RO
    #[field(u32[cid3])]
    cid3: u32
}


// System Control Space: https://developer.arm.com/documentation/ddi0403/d/System-Level-Architecture/System-Address-Map/System-Control-Space--SCS-/System-control-and-ID-registers?lang=en
#[derive(Debug)]
#[flux_rs::refined_by(
    // sys control id regs
    ictr: int,    
    actlr: int,    
    stir: int,    
    pid4: int,    
    pid5: int,    
    pid6: int,    
    pid7: int,    
    pid0: int,    
    pid1: int,    
    pid2: int,    
    pid3: int,    
    cid0: int,    
    cid1: int,    
    cid2: int,    
    cid3: int,
    // sys control blocks
    cpuid: int,    
    icsr: int,    
    vtor: int,    
    aircr: int,    
    scr: int,    
    ccr: int,    
    shpr1: int,    
    shpr2: int,    
    shpr3: int,    
    shcsr: int,    
    cfsr: int,    
    hfsr: int,    
    dfsr: int,    
    mmfar: int,    
    bfar: int,    
    afsr: int,    
    cpacr: int,
)]
pub struct SysControlSpace {
    #[field(SysControlIDReg[
        ictr,    
        actlr,    
        stir,    
        pid4,    
        pid5,    
        pid6,    
        pid7,    
        pid0,    
        pid1,    
        pid2,    
        pid3,    
        cid0,    
        cid1,    
        cid2,    
        cid3
    ])]
    sys_control_id_regs: SysControlIDReg,
    #[field(SysControlBlock[
        cpuid,    
        icsr,    
        vtor,    
        aircr,    
        scr,    
        ccr,    
        shpr1,    
        shpr2,    
        shpr3,    
        shcsr,    
        cfsr,    
        hfsr,    
        dfsr,    
        mmfar,    
        bfar,    
        afsr,    
        cpacr
    ])]
    sys_control_block: SysControlBlock,
}
