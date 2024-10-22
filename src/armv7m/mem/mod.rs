// The following file implements memory layout for the ARMv7m architecture.
//
// NOTE: For now, just going to map the PPB - a 1 mb region reserved by the architecture. In the
// future, maybe we can expand this.
//
// Memory types:
//
// - Normal Memory: Can be read or write and is idempotent (see p. A3-80 in the manual)
// - Device Memory: Causes side effects
// - Strongly Ordered Memory: An access to memory marked as Strongly Ordered acts as a memory barrier to all other explicit accesses from that processor, until the point at which the access is complete (that is, has changed the state of the target location or data has been returned). In addition, an access to memory marked as Strongly Ordered must complete before the end of a memory barrier
//
// See here for PPB docs: https://developer.arm.com/documentation/ddi0403/d/System-Level-Architecture/System-Address-Map/System-Control-Space--SCS-?lang=en
//
// System control and ID registers	
// 0xE000E000-0xE000E00F	Includes the Interrupt Controller Type and Auxiliary Control registers
// 0xE000ED00-0xE000ED8F	System control block
// 0xE000EDF0-0xE000EEFF	Debug registers in the SCS
// 0xE000EF00-0xE000EF8F	Includes the SW Trigger Interrupt Register
// 0xE000EF90-0xE000EFCF	implementation defined
// 0xE000EFD0-0xE000EFFF	Microcontroller-specific ID space
//
//
// SysTick	0xE000E010-0xE000E0FF	System Timer, see The system timer, SysTick
// NVIC	0xE000E100-0xE000ECFF	External interrupt controller, see Nested Vectored Interrupt Controller, NVIC
// MPU	0xE000ED90-0xE000EDEF	Memory Protection Unit, see Protected Memory System Architecture, PMSAv7

const PPB_START: u32 = 0xE000_0000;
const PPB_END: u32 = 0xE00F_FFFF;

const INTERRUPT_AUXILIARY_CONTROL_REGISTER_START: u32 = 0xE000_E000;
const  INTERRUPT_AUXILIARY_CONTROL_REGISTER_END: u32 = 0xE000_E00F;

const SYSTEM_CONTROL_BLOCK_START: u32 = 0xE000_ED00;
const SYSTEM_CONTROL_BLOCK_END: u32 = 0xE000_ED8F;

const SW_TRIGGER_INTERRUPT_REG_START: u32 = 0xE000EF00;
const SW_TRIGGER_INTERRUPT_REG_END: u32 = 0xE000EF8F;

const SYS_TICK_START: u32 = 0xE000E010;
const SYS_TICK_END: u32 = 0xE000E0FF;

const NVIC_START: u32 =	0xE000E100;
const NVIC_END: u32 = 0xE000ECFF;

const MPU_START: u32 = 0xE000ED90;
const MPU_END: u32 = 0xE000EDEF;

mod flux_defs;
mod nvic;
mod sys_control;
mod sys_tick;
mod mpu;

use flux_defs::*;
use mpu::Mpu;
use sys_control::SysControlSpace;
use sys_tick::SysTick;
use nvic::Nvic;

#[derive(Debug)]
#[flux_rs::refined_by(
    sys_control: SysControlSpace,
    sys_tick: SysTick,
    nvic: Nvic,
    mpu: Mpu
)]
pub struct Ppb {
    #[field(SysControlSpace[sys_control])]
    system_control_space: SysControlSpace,

    #[field(SysTick[sys_tick])]
    sys_tick: SysTick,

    #[field(Nvic[nvic])]
    nvic: Nvic,

    #[field(Mpu[mpu])]
    mpu: Mpu,
}

impl Ppb {
    #[flux_rs::sig(fn (&Ppb[@ppb], u32[@addr]) -> u32{v: check_ppb_value_read(addr, ppb, v) } requires is_valid_read_addr(addr))]
    pub fn read(&self, address: u32) -> u32 {
        match address {
            INTERRUPT_AUXILIARY_CONTROL_REGISTER_START..=INTERRUPT_AUXILIARY_CONTROL_REGISTER_END
            | SYSTEM_CONTROL_BLOCK_START..=SYSTEM_CONTROL_BLOCK_END
            | SW_TRIGGER_INTERRUPT_REG_START..=SW_TRIGGER_INTERRUPT_REG_END
            => self.system_control_space.read(address),
            SYS_TICK_START..=SYS_TICK_END => self.sys_tick.read(address),
            NVIC_START..=NVIC_END => self.nvic.read(address),
            MPU_START..=MPU_END => self.mpu.read(address),
            // NOTE: Not supporting some of these for now
            0xE000EDF0..=0xE000EEFF => panic!("Read of debug reg (not implemented)"),
            0xE000EF90..=0xE000EFCF => panic!("Read of Implementation defined regs"),
            0xE000EFD0..=0xE000EFFF => panic!("Read of mc specific space"),
            _ => panic!("Read of invalid addr (only system control, sys tick, nvic, and mpun are defined)")
        }
    }

    #[flux_rs::sig(
        fn (self: &strg Ppb[@ppb], u32[@addr], u32[@val]) 
            requires is_valid_write_addr(addr)
            ensures self: Ppb { new_ppb: check_ppb_value_write(addr, new_ppb, val) }
    )]
    pub fn write(&mut self, address: u32, value: u32) {
        match address {
            INTERRUPT_AUXILIARY_CONTROL_REGISTER_START..=INTERRUPT_AUXILIARY_CONTROL_REGISTER_END
            | SYSTEM_CONTROL_BLOCK_START..=SYSTEM_CONTROL_BLOCK_END
            | SW_TRIGGER_INTERRUPT_REG_START..=SW_TRIGGER_INTERRUPT_REG_END => self.system_control_space.write(address, value),
            SYS_TICK_START..=SYS_TICK_END =>  self.sys_tick.write(address, value),
            NVIC_START..=NVIC_END => self.nvic.write(address, value),
            MPU_START..=MPU_END => self.mpu.write(address, value),
            // NOTE: Not supporting some of these for now
            0xE000EF90..=0xE000EFCF => panic!("Write to Implementation defined regs"),
            0xE000EFD0..=0xE000EFFF => panic!("Write to mc specific space"),
            0xE000EDF0..=0xE000EEFF =>  panic!("Write to debug regs (not implemented)"),
            _ => panic!("Write to invalid addr (only system control, sys tick, nvic, and mpun are defined)")
        }
    }
}

#[derive(Debug)]
#[flux_rs::refined_by(ppb: Ppb)]
pub struct Memory {
    #[field(Ppb[ppb])]
    ppb: Ppb,
}

impl Memory {

    #[flux_rs::sig(fn (&Memory[@mem], u32[@addr]) -> u32{ v: check_mem_value_read(addr, mem, v) } requires is_valid_read_addr(addr))]
    pub fn read(&self, address: u32) -> u32 {
        match address {
            PPB_START..=PPB_END => self.ppb.read(address),
            _ => panic!("Read of unknown memory address (only ppb is defined)")
        }
    }

    #[flux_rs::sig(
        fn (self: &strg Memory[@mem], u32[@addr], u32[@val]) 
            requires is_valid_write_addr(addr)
            ensures self: Memory { new_mem: check_mem_value_write(addr, new_mem, val) }
    )]
    pub fn write(&mut self, address: u32, value: u32) {
        match address {
            PPB_START..=PPB_END => self.ppb.write(address, value),
            _ => panic!("Write to unknown memory address (only ppb is defined)")
        }
    }
}
