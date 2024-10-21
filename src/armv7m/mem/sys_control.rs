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

impl SysControlBlock {

    pub fn read(&self, address: u32) -> u32 {
        // 0xE000ED00	CPUID	RO	implementation defined	CPUID Base Register.
        // 0xE000ED04	ICSR	RW	0x00000000	Interrupt Control and State Register, ICSR.
        // 0xE000ED08	VTOR	RW	0x00000000[a]	Vector Table Offset Register, VTOR.
        // 0xE000ED0C	AIRCR	RW	[b]	Application Interrupt and Reset Control Register, AIRCR.
        // 0xE000ED10	SCR	RW	0x00000000	System Control Register, SCR.
        // 0xE000ED14	CCR	RW	0x00000000	Configuration and Control Register, CCR.
        // 0xE000ED18	SHPR1	RW	0x00000000	System Handler Priority Register 1, SHPR1.
        // 0xE000ED1C	SHPR2	RW	0x00000000	System Handler Priority Register 2, SHPR2.
        // 0xE000ED20	SHPR3	RW	0x00000000	System Handler Priority Register 3, SHPR3.
        // 0xE000ED24	SHCSR	RW	0x00000000	System Handler Control and State Register, SHCSR.
        // 0xE000ED28	CFSR	RW	0x00000000
        // 0xE000ED2C	HFSR	RW	0x00000000	HardFault Status Register, HFSR.
        // 0xE000ED30	DFSR	RW	0x00000000[c]	Debug Fault Status Register, DFSR.
        // 0xE000ED34	MMFAR	RW	unknown	MemManage Fault Address Register, MMFAR.
        // 0xE000ED38	BFAR	RW	unknown	BusFault Address Register, BFAR.
        // 0xE000ED3C	AFSR	RW	unknown	Auxiliary Fault Status Register, AFSR, implementation defined.
        // 0xE000ED88	CPACR	RW	unknown	Coprocessor Access Control Register, CPACR.
        match address {
            0xE000ED00 => self.cpuid,
            0xE000ED04	=> self.icsr,
            0xE000ED08 => self.vtor,
            0xE000ED0C => self.aircr,
            0xE000ED10 => self.scr,
            0xE000ED14 => self.ccr,
            0xE000ED18 => self.shpr1,
            0xE000ED1C => self.shpr2,
            0xE000ED20 => self.shpr3,
            0xE000ED24 => self.shcsr,
            0xE000ED28 => self.cfsr,
            0xE000ED2C => self.hfsr,
            0xE000ED30 => self.dfsr,
            0xE000ED34 => self.mmfar,
            0xE000ED38 => self.bfar,
            0xE000ED3C => self.afsr,
            0xE000ED88 => self.cpacr,
            // Reserved fields etc.
            // 0xE000ED40 - 0xE000ED7C	-	-	-	Reserved for CPUID registers, see The CPUID Scheme.
            // 0xE000ED80 - 0xE000ED84	-	-	-	Reserved.
            // 0xE000ED8C	-	-	-	Reserved.
            0xE000ED40..=0xE000ED7C => panic!("Read of CPUID scheme - these are implementation defined"),
            0xE000ED80..=0xE000ED84 => panic!("Read of reserved register"),
            0xE000ED8C => panic!("Read of reserved register"),
            _ => panic!("Read of invalid addr")
        }
    }

    pub fn write(&mut self, address: u32, value: u32) {
        // 0xE000ED00	CPUID	RO	implementation defined	CPUID Base Register.
        // 0xE000ED04	ICSR	RW	0x00000000	Interrupt Control and State Register, ICSR.
        // 0xE000ED08	VTOR	RW	0x00000000[a]	Vector Table Offset Register, VTOR.
        // 0xE000ED0C	AIRCR	RW	[b]	Application Interrupt and Reset Control Register, AIRCR.
        // 0xE000ED10	SCR	RW	0x00000000	System Control Register, SCR.
        // 0xE000ED14	CCR	RW	0x00000000	Configuration and Control Register, CCR.
        // 0xE000ED18	SHPR1	RW	0x00000000	System Handler Priority Register 1, SHPR1.
        // 0xE000ED1C	SHPR2	RW	0x00000000	System Handler Priority Register 2, SHPR2.
        // 0xE000ED20	SHPR3	RW	0x00000000	System Handler Priority Register 3, SHPR3.
        // 0xE000ED24	SHCSR	RW	0x00000000	System Handler Control and State Register, SHCSR.
        // 0xE000ED28	CFSR	RW	0x00000000
        // 0xE000ED2C	HFSR	RW	0x00000000	HardFault Status Register, HFSR.
        // 0xE000ED30	DFSR	RW	0x00000000[c]	Debug Fault Status Register, DFSR.
        // 0xE000ED34	MMFAR	RW	unknown	MemManage Fault Address Register, MMFAR.
        // 0xE000ED38	BFAR	RW	unknown	BusFault Address Register, BFAR.
        // 0xE000ED3C	AFSR	RW	unknown	Auxiliary Fault Status Register, AFSR, implementation defined.
        // 0xE000ED88	CPACR	RW	unknown	Coprocessor Access Control Register, CPACR.
        let reg = match address {
            0xE000ED00 => panic!("Write to read only reg"),
            0xE000ED04	=> &mut self.icsr,
            0xE000ED08 => &mut self.vtor,
            0xE000ED0C => &mut self.aircr,
            0xE000ED10 => &mut self.scr,
            0xE000ED14 => &mut self.ccr,
            0xE000ED18 => &mut self.shpr1,
            0xE000ED1C => &mut self.shpr2,
            0xE000ED20 => &mut self.shpr3,
            0xE000ED24 => &mut self.shcsr,
            0xE000ED28 => &mut self.cfsr,
            0xE000ED2C => &mut self.hfsr,
            0xE000ED30 => &mut self.dfsr,
            0xE000ED34 => &mut self.mmfar,
            0xE000ED38 => &mut self.bfar,
            0xE000ED3C => &mut self.afsr,
            0xE000ED88 => &mut self.cpacr,
            // Reserved fields etc.
            // 0xE000ED40 - 0xE000ED7C	-	-	-	Reserved for CPUID registers, see The CPUID Scheme.
            // 0xE000ED80 - 0xE000ED84	-	-	-	Reserved.
            // 0xE000ED8C	-	-	-	Reserved.
            0xE000ED40..=0xE000ED7C => panic!("Write to CPUID scheme - these are implementation defined"),
            0xE000ED80..=0xE000ED84 =>  panic!("Write to Reserved Reg"),
            0xE000ED8C => panic!("Write to Reserved Reg"),
            _ => panic!("Write to invalid addr")
        };
        *reg = value;
    }
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

impl SysControlIDReg {
    pub fn read(&self, address: u32) -> u32 {
        // Address	Name	Type	Reset	Description
        // 0xE000E004	ICTR	RO	implementation defined	Interrupt Controller Type Register, ICTR
        // 0xE000E008	ACTLR	RW	implementation defined	Auxiliary Control Register, ACTLR
        // 0xE000EF00	STIR	WO	-	Software Triggered Interrupt Register, STIR
        // 0xE000EFD0	PID4	RO	[a]	Peripheral Identification Registers, see The CPUID Scheme
        // 0xE000EFD4	PID5	RO
        // 0xE000EFD8	PID6	RO
        // 0xE000EFDC	PID7	RO
        // 0xE000EFE0	PID0	RO
        // 0xE000EFE4	PID1	RO
        // 0xE000EFE8	PID2	RO
        // 0xE000EFEC	PID3	RO
        // 0xE000EFF0	CID0	RO	[a]	Component Identification Registers, see The CPUID Scheme
        // 0xE000EFF4	CID1	RO
        // 0xE000EFF8	CID2	RO
        // 0xE000EFFC	CID3	RO
        match address {
            0xE000E004	=> self.ictr,
            0xE000E008	=> self.actlr,
            0xE000EF00	=> panic!("Read of write only register"),
            0xE000EFD0	=> self.pid4,
            0xE000EFD4	=> self.pid5,
            0xE000EFD8	=> self.pid6,
            0xE000EFDC	=> self.pid7,
            0xE000EFE0	=> self.pid0,
            0xE000EFE4	=> self.pid1,
            0xE000EFE8	=> self.pid2,
            0xE000EFEC	=> self.pid3,
            0xE000EFF0	=> self.cid0,
            0xE000EFF4	=> self.cid1,
            0xE000EFF8	=> self.cid2,
            0xE000EFFC	=> self.cid3,
            // Reserved regs etc.
            // 0xE000E000	-	RW	0x00000000	Master Control register, Reserved
            // 0xE000E00C	-	-	-	Reserved
            // 0xE000EF04 - 0xE000EF8C	-	-	-	Reserved
            // 0xE000EF90 - 0xE000EFCC	...	...	...	implementation defined
            // 0xE000EDF0 - 0xE000EEFC	-	-	-	See Debug register support in the SCS
            0xE000E000 => panic!("READ of RESERVED REGISTER Master Control"),
            0xE000E00C	=> panic!("READ of RESERVED REGISTER"),
            0xE000EF04  => panic!("READ of RESERVED REGISTER"),
            0xE000EF90..=0xE000EFCC	 => panic!("READ of IMPLEMENTATION DEFINED REG"),
            0xE000EDF0..=0xE000EEFC	 => panic!("READ OF DEBUG REGISTER"),
            _ => panic!("Read of invalid addr")
        }
    }

    pub fn write(&mut self, address: u32, value: u32) {
        // Address	Name	Type	Reset	Description
        // 0xE000E004	ICTR	RO	implementation defined	Interrupt Controller Type Register, ICTR
        // 0xE000E008	ACTLR	RW	implementation defined	Auxiliary Control Register, ACTLR
        // 0xE000EF00	STIR	WO	-	Software Triggered Interrupt Register, STIR
        // 0xE000EFD0	PID4	RO	[a]	Peripheral Identification Registers, see The CPUID Scheme
        // 0xE000EFD4	PID5	RO
        // 0xE000EFD8	PID6	RO
        // 0xE000EFDC	PID7	RO
        // 0xE000EFE0	PID0	RO
        // 0xE000EFE4	PID1	RO
        // 0xE000EFE8	PID2	RO
        // 0xE000EFEC	PID3	RO
        // 0xE000EFF0	CID0	RO	[a]	Component Identification Registers, see The CPUID Scheme
        // 0xE000EFF4	CID1	RO
        // 0xE000EFF8	CID2	RO
        // 0xE000EFFC	CID3	RO
        let reg = match address {
            0xE000E004	=> panic!("Attempted write to read only reg"),
            0xE000E008	=> &mut self.actlr,
            0xE000EF00	=> &mut self.stir,
            0xE000EFD0	=> panic!("Attempted write to read only reg"),
            0xE000EFD4	=> panic!("Attempted write to read only reg"),
            0xE000EFD8	=> panic!("Attempted write to read only reg"),
            0xE000EFDC	=> panic!("Attempted write to read only reg"),
            0xE000EFE0	=> panic!("Attempted write to read only reg"),
            0xE000EFE4	=> panic!("Attempted write to read only reg"),
            0xE000EFE8	=> panic!("Attempted write to read only reg"),
            0xE000EFEC	=> panic!("Attempted write to read only reg"),
            0xE000EFF0	=> panic!("Attempted write to read only reg"),
            0xE000EFF4	=> panic!("Attempted write to read only reg"),
            0xE000EFF8	=> panic!("Attempted write to read only reg"),
            0xE000EFFC	=> panic!("Attempted write to read only reg"),
            // Reserved regs etc.
            // 0xE000E000	-	RW	0x00000000	Master Control register, Reserved
            // 0xE000E00C	-	-	-	Reserved
            // 0xE000EF04 - 0xE000EF8C	-	-	-	Reserved
            // 0xE000EF90 - 0xE000EFCC	...	...	...	implementation defined
            // 0xE000EDF0 - 0xE000EEFC	-	-	-	See Debug register support in the SCS
            0xE000E000 => panic!("Write to reserved reg"),
            0xE000E00C	=> panic!("Write to reserved reg"),
            0xE000EF04  => panic!("Write to reserved reg"),
            0xE000EF90..=0xE000EFCC	 => panic!("Write to implementation defined reg"),
            0xE000EDF0..=0xE000EEFC	 => panic!("Write to debug reg"),
            _ => panic!("Write to invalid addr")
        };
        *reg = value;
    }
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

impl SysControlSpace {
    
    pub fn read(&self, address: u32) -> u32 {
        match address {
            0xE000E000..=0xE000E00F => self.sys_control_id_regs.read(address),
             0xE000ED00..=0xE000ED8F => self.sys_control_block.read(address),
            _ => panic!("Read of invalid addr")
        }
    }

    pub fn write(&mut self, address: u32, value: u32) {
        match address {
            0xE000E000..=0xE000E00F => self.sys_control_id_regs.write(address, value),
             0xE000ED00..=0xE000ED8F => self.sys_control_block.write(address, value),
            _ => panic!("Write to invalid addr")
        }
    }
}
