// The following file implements memory layout for the ARMv7m architecture.
//
// NOTE: For now, just going to map the PPB - a 1 mb region reserved by the architecture. In the future, maybe we can expand this.
//
// Memory types:
//
// - Normal Memory: Can be read or write and is idempotent (see p. A3-80 in the manual)
// - Device Memory: Causes side effects
// - Strongly Ordered Memory: An access to memory marked as Strongly Ordered acts as a memory barrier to all other explicit accesses from that processor, until the point at which the access is complete (that is, has changed the state of the target location or data has been returned). In addition, an access to memory marked as Strongly Ordered must complete before the end of a memory barrier
//
// See here for PPB docs
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

pub type Mem = Regs<BV32, BV32>;

#[flux_rs::constant(bv_int_to_bv32(0xE000_0000))]
const PPB_START: BV32 = BV32::new(0xE000_0000);

#[flux_rs::constant(bv_int_to_bv32(0xE00F_FFFF))]
const PPB_END: BV32 = BV32::new(0xE00F_FFFF);

#[flux_rs::constant(bv_int_to_bv32(0xE000_E000))]
const INTERRUPT_AUXILIARY_CONTROL_REGISTER_START: BV32 = BV32::new(0xE000_E000);
#[flux_rs::constant(bv_int_to_bv32(0xE000_E00F))]
const INTERRUPT_AUXILIARY_CONTROL_REGISTER_END: BV32 = BV32::new(0xE000_E00F);

#[flux_rs::constant(bv_int_to_bv32(0xE000_ED00))]
const SYSTEM_CONTROL_BLOCK_START: BV32 = BV32::new(0xE000_ED00);
#[flux_rs::constant(bv_int_to_bv32(0xE000_ED8F))]
const SYSTEM_CONTROL_BLOCK_END: BV32 = BV32::new(0xE000_ED8F);

#[flux_rs::constant(bv_int_to_bv32(0xE000EF00))]
const SW_TRIGGER_INTERRUPT_REG_START: BV32 = BV32::new(0xE000EF00);
#[flux_rs::constant(bv_int_to_bv32(0xE000EF8F))]
const SW_TRIGGER_INTERRUPT_REG_END: BV32 = BV32::new(0xE000EF8F);

#[flux_rs::constant(bv_int_to_bv32(0xE000E010))]
const SYS_TICK_START: BV32 = BV32::new(0xE000E010);
#[flux_rs::constant(bv_int_to_bv32(0xE000E0FF))]
const SYS_TICK_END: BV32 = BV32::new(0xE000E0FF);

#[flux_rs::constant(bv_int_to_bv32(0xE000E100))]
const NVIC_START: BV32 = BV32::new(0xE000E100);
#[flux_rs::constant(bv_int_to_bv32(0xE000ECFF))]
const NVIC_END: BV32 = BV32::new(0xE000ECFF);

#[flux_rs::constant(bv_int_to_bv32(0xE000ED90))]
const MPU_START: BV32 = BV32::new(0xE000ED90);
#[flux_rs::constant(bv_int_to_bv32(0xE000EDEF))]
const MPU_END: BV32 = BV32::new(0xE000EDEF);

#[flux_rs::constant(bv_int_to_bv32(0x6000_0000))]
const RAM_START: BV32 = BV32::new(0x6000_0000);
#[flux_rs::constant(bv_int_to_bv32(0x9FFF_FFFF))]
const RAM_END: BV32 = BV32::new(0x9FFF_FFFF);

pub mod flux_defs;
mod mpu;
mod nvic;
mod sys_control;
mod sys_tick;

use flux_defs::*;
use mpu::{is_valid_mpu_read_addr, is_valid_mpu_write_addr};
use nvic::{is_valid_nvic_read_addr, is_valid_nvic_write_addr};
use sys_control::{is_valid_sys_control_space_read_addr, is_valid_sys_control_space_write_addr};
use sys_tick::{is_valid_sys_tick_read_addr, is_valid_sys_tick_write_addr};

use crate::flux_support::rmap::Regs;
use flux_rs::bitvec::BV32;

#[derive(Debug)]
#[flux_rs::refined_by(
    mem: Map<BV32, BV32>
)]
pub struct Memory {
    #[field(Regs<BV32, BV32>[mem])]
    mem: Mem,
}

impl Memory {
    #[flux_rs::sig(
        fn (&Memory[@mem], BV32[@addr]) -> BV32[get_mem_addr(addr, mem)]
            requires is_valid_read_addr(addr)
    )]
    pub fn read(&self, address: BV32) -> BV32 {
        let ppb_start = PPB_START;
        let ppb_end = PPB_END;
        let ram_start = RAM_START;
        let ram_end = RAM_END;
        if address >= ppb_start && address <= ppb_end {
            if !(is_valid_mpu_read_addr(address)
                || is_valid_sys_tick_read_addr(address)
                || is_valid_sys_control_space_read_addr(address)
                || is_valid_nvic_read_addr(address))
            {
                panic!("Read of Invalid PPB address")
            }
            *self.mem.get(&address).unwrap()
        } else if address >= ram_start && address <= ram_end {
            *self.mem.get(&address).unwrap()
        } else {
            panic!("Read of unknown memory address (only ppb is defined)")
        }
    }

    #[flux_rs::sig(
        fn (self: &strg Memory[@old_mem], BV32[@addr], BV32[@val])
            requires is_valid_write_addr(addr)
            ensures self: Memory { new_mem: new_mem == update_mem(addr, old_mem, val) }
    )]
    pub fn write(&mut self, address: BV32, value: BV32) {
        let ppb_start = PPB_START;
        let ppb_end = PPB_END;
        let ram_start = RAM_START;
        let ram_end = RAM_END;
        if address >= ppb_start && address <= ppb_end {
            if !(is_valid_mpu_write_addr(address)
                || is_valid_sys_tick_write_addr(address)
                || is_valid_sys_control_space_write_addr(address)
                || is_valid_nvic_write_addr(address))
            {
                panic!("Write to Invalid PPB address")
            }
            self.mem.set(address, value)
        } else if address >= ram_start && address <= ram_end {
            self.mem.set(address, value)
        } else {
            panic!("Write to unknown memory address (only ppb & RAM are defined)")
        }
    }
}
