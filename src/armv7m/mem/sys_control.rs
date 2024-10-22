use super::flux_defs::sys_control_block_defs::*;
use super::flux_defs::sys_control_id_reg_defs::*;

// Range
use super::SYSTEM_CONTROL_BLOCK_START;
use super::SYSTEM_CONTROL_BLOCK_END;
use super::SW_TRIGGER_INTERRUPT_REG_START;
use super::SW_TRIGGER_INTERRUPT_REG_END;
use super::INTERRUPT_AUXILIARY_CONTROL_REGISTER_START;
use super::INTERRUPT_AUXILIARY_CONTROL_REGISTER_END;



// System Control Block Addresses
pub const CPUID_ADDR: u32 =	 0xE000ED00;	
pub const ICSR_ADDR: u32 =	 0xE000ED04;	
pub const VTOR_ADDR: u32 =	 0xE000ED08;	
pub const AIRCR_ADDR: u32 =	 0xE000ED0C;	
pub const SCR_ADDR: u32 =	 0xE000ED10;	
pub const CCR_ADDR: u32 =	 0xE000ED14;	
pub const SHPR1_ADDR: u32 =	 0xE000ED18;	
pub const SHPR2_ADDR: u32 =	 0xE000ED1C;	
pub const SHPR3_ADDR: u32 =	 0xE000ED20;	
pub const SHCSR_ADDR: u32 =	 0xE000ED24;	
pub const CFSR_ADDR: u32 =	 0xE000ED28;	
pub const HFSR_ADDR: u32 =	 0xE000ED2C;	
pub const DFSR_ADDR: u32 =	 0xE000ED30;	
pub const MMFAR_ADDR: u32 =	 0xE000ED34;	
pub const BFAR_ADDR: u32 =	 0xE000ED38;	
pub const AFSR_ADDR: u32 =	 0xE000ED3C;	
pub const CPACR_ADDR: u32 =	 0xE000ED88;	

// ID Reg
pub const ICTR_ADDR: u32 =	 0xE000E004;	
pub const ACTLR_ADDR: u32 =	 0xE000E008;	
pub const STIR_ADDR: u32 =	 0xE000EF00;	
pub const PID4_ADDR: u32 =	 0xE000EFD0;	
pub const PID5_ADDR: u32 =	 0xE000EFD4;	
pub const PID6_ADDR: u32 =	 0xE000EFD8;	
pub const PID7_ADDR: u32 =	 0xE000EFDC;	
pub const PID0_ADDR: u32 =	 0xE000EFE0;	
pub const PID1_ADDR: u32 =	 0xE000EFE4;	
pub const PID2_ADDR: u32 =	 0xE000EFE8;	
pub const PID3_ADDR: u32 =	 0xE000EFEC;	
pub const CID0_ADDR: u32 =	 0xE000EFF0;	
pub const CID1_ADDR: u32 =	 0xE000EFF4;	
pub const CID2_ADDR: u32 =	 0xE000EFF8;	
pub const CID3_ADDR: u32 =	 0xE000EFFC;	

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

    #[flux_rs::sig(
        fn (&SysControlBlock[@sys_control_block], u32[@addr]) -> u32[sys_control_block_addr_into_reg(addr, sys_control_block)]
            requires is_valid_sys_control_block_read_addr(addr)
    )]
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
            CPUID_ADDR => self.cpuid,
            ICSR_ADDR => self.icsr,
            VTOR_ADDR => self.vtor,
            AIRCR_ADDR => self.aircr,
            SCR_ADDR => self.scr,
            CCR_ADDR => self.ccr,
            SHPR1_ADDR => self.shpr1,
            SHPR2_ADDR => self.shpr2,
            SHPR3_ADDR => self.shpr3,
            SHCSR_ADDR => self.shcsr,
            CFSR_ADDR => self.cfsr,
            HFSR_ADDR => self.hfsr,
            DFSR_ADDR => self.dfsr,
            MMFAR_ADDR => self.mmfar,
            BFAR_ADDR => self.bfar,
            AFSR_ADDR => self.afsr,
            CPACR_ADDR => self.cpacr,
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

    #[flux_rs::sig(
        fn (self: &strg SysControlBlock[@sys_control_block], u32[@addr], u32[@val]) 
                requires is_valid_sys_control_block_write_addr(addr)
                ensures self: SysControlBlock {
                    new_sys_control_block: sys_control_block_addr_into_reg(addr, new_sys_control_block) == val
                }
    )]
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
            CPUID_ADDR => panic!("Write to read only reg"),
            ICSR_ADDR => &mut self.icsr,
            VTOR_ADDR => &mut self.vtor,
            AIRCR_ADDR => &mut self.aircr,
            SCR_ADDR => &mut self.scr,
            CCR_ADDR => &mut self.ccr,
            SHPR1_ADDR => &mut self.shpr1,
            SHPR2_ADDR => &mut self.shpr2,
            SHPR3_ADDR => &mut self.shpr3,
            SHCSR_ADDR => &mut self.shcsr,
            CFSR_ADDR => &mut self.cfsr,
            HFSR_ADDR => &mut self.hfsr,
            DFSR_ADDR => &mut self.dfsr,
            MMFAR_ADDR => &mut self.mmfar,
            BFAR_ADDR => &mut self.bfar,
            AFSR_ADDR => &mut self.afsr,
            CPACR_ADDR => &mut self.cpacr,
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
    #[flux_rs::sig(
        fn (&SysControlIDReg[@sys_control_id], u32[@addr]) -> u32[sys_control_id_reg_addr_into_reg(addr, sys_control_id)]
            requires is_valid_sys_control_id_reg_read_addr(addr)
    )]
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
            ICTR_ADDR => self.ictr,
            ACTLR_ADDR => self.actlr,
            STIR_ADDR =>  panic!("Read of write only register"),
            PID4_ADDR => self.pid4,
            PID5_ADDR => self.pid5,
            PID6_ADDR => self.pid6,
            PID7_ADDR => self.pid7,
            PID0_ADDR => self.pid0,
            PID1_ADDR => self.pid1,
            PID2_ADDR => self.pid2,
            PID3_ADDR => self.pid3,
            CID0_ADDR => self.cid0,
            CID1_ADDR => self.cid1,
            CID2_ADDR => self.cid2,
            CID3_ADDR => self.cid3,
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

    #[flux_rs::sig(
        fn (self: &strg SysControlIDReg[@sys_control_id], u32[@addr], u32[@val])
            requires is_valid_sys_control_id_reg_write_addr(addr)
            ensures self: SysControlIDReg { new_sys_control_id: sys_control_id_reg_addr_into_reg(addr, new_sys_control_id) == val }
    )]
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
            ICTR_ADDR => panic!("Attempted write to read only reg"),
            ACTLR_ADDR => &mut self.actlr,
            STIR_ADDR => &mut self.stir,
            PID4_ADDR => panic!("Attempted write to read only reg"),
            PID5_ADDR => panic!("Attempted write to read only reg"),
            PID6_ADDR => panic!("Attempted write to read only reg"),
            PID7_ADDR => panic!("Attempted write to read only reg"),
            PID0_ADDR => panic!("Attempted write to read only reg"),
            PID1_ADDR => panic!("Attempted write to read only reg"),
            PID2_ADDR => panic!("Attempted write to read only reg"),
            PID3_ADDR => panic!("Attempted write to read only reg"),
            CID0_ADDR => panic!("Attempted write to read only reg"),
            CID1_ADDR => panic!("Attempted write to read only reg"),
            CID2_ADDR => panic!("Attempted write to read only reg"),
            CID3_ADDR => panic!("Attempted write to read only reg"),
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
            0xE000EDF0..=0xE000EEFC	 => panic!("Write to debug reg (not implemented)"),
            _ => panic!("Write to invalid addr")
        };
        *reg = value;
    }
}


// System Control Space: https://developer.arm.com/documentation/ddi0403/d/System-Level-Architecture/System-Address-Map/System-Control-Space--SCS-/System-control-and-ID-registers?lang=en
#[derive(Debug)]
#[flux_rs::refined_by(
    sys_control_id_reg: SysControlIDReg,
    sys_control_block: SysControlBlock
)]
pub struct SysControlSpace {
    #[field(SysControlIDReg[sys_control_id_reg])]
    sys_control_id_regs: SysControlIDReg,
    #[field(SysControlBlock[sys_control_block])]
    sys_control_block: SysControlBlock,
}

impl SysControlSpace {
    
    #[flux_rs::sig(
        fn (&SysControlSpace[@sys_control], u32[@addr]) -> u32{ v: check_sys_control_space_value_read(addr, sys_control, v) } 
            requires is_valid_sys_control_space_read_addr(addr)
    )]
    pub fn read(&self, address: u32) -> u32 {
        match address {
            INTERRUPT_AUXILIARY_CONTROL_REGISTER_START..=INTERRUPT_AUXILIARY_CONTROL_REGISTER_END 
            | SW_TRIGGER_INTERRUPT_REG_START..=SW_TRIGGER_INTERRUPT_REG_END => self.sys_control_id_regs.read(address),
             SYSTEM_CONTROL_BLOCK_START..=SYSTEM_CONTROL_BLOCK_END => self.sys_control_block.read(address),
            _ => panic!("Read of invalid addr")
        }
    }
    
    #[flux_rs::sig(
        fn (self: &strg SysControlSpace[@sys_control], u32[@addr], u32[@val]) 
            requires is_valid_sys_control_space_write_addr(addr)
            ensures self: SysControlSpace { new_sys_control: check_sys_control_space_value_write(addr, new_sys_control, val) }
    )]
    pub fn write(&mut self, address: u32, value: u32) {
        match address {
            INTERRUPT_AUXILIARY_CONTROL_REGISTER_START..=INTERRUPT_AUXILIARY_CONTROL_REGISTER_END 
            | SW_TRIGGER_INTERRUPT_REG_START..=SW_TRIGGER_INTERRUPT_REG_END => self.sys_control_id_regs.write(address, value),
             SYSTEM_CONTROL_BLOCK_START..=SYSTEM_CONTROL_BLOCK_END => self.sys_control_block.write(address, value),
            _ => panic!("Write to invalid addr")
        }
    }
}
