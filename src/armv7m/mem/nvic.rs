use crate::flux_support::rmap::Regs;

use super::flux_defs::nvic_defs::*;

pub type NvicRegs = Regs<u32, u32>;

pub const ISER_START: u32 = 0xE000E100;
pub const ISER_END: u32 = 0xE000E13C;
//
pub const ICER_START: u32 = 0xE000E180;
pub const ICER_END: u32 = 0xE000E1BC;
//
pub const ISPR_START: u32 = 0xE000E200;
pub const ISPR_END: u32 = 0xE000E23C;
//
pub const ICPR_START: u32 = 0xE000E280;
pub const ICPR_END: u32 = 0xE000E2BC;
//
pub const IABR_START: u32 = 0xE000E300;
pub const IABR_END: u32 = 0xE000E37C;
//
pub const IPR_START: u32 = 0xE000E400;
pub const IPR_END: u32 = 0xE000E7EC;

#[flux_rs::sig(fn (u32[@addr]) -> bool[is_valid_nvic_addr(addr)])]
fn is_valid_nvic_addr(address: u32) -> bool {
    (address >= ISER_START && address <= ISER_END)
        || (address >= ICER_START && address <= ICER_END)
        || (address >= ISPR_START && address <= ISPR_END)
        || (address >= ICPR_START && address <= ICPR_END)
        || (address >= IABR_START && address <= IABR_END)
        || (address >= IPR_START && address <= IPR_END)
}

#[flux_rs::sig(fn (u32[@addr]) -> bool[is_valid_nvic_read_addr(addr)])]
pub fn is_valid_nvic_read_addr(address: u32) -> bool {
    // all read
    is_valid_nvic_addr(address)
}

#[flux_rs::sig(fn (u32[@addr]) -> bool[is_valid_nvic_write_addr(addr)])]
pub fn is_valid_nvic_write_addr(address: u32) -> bool {
    // all write
    is_valid_nvic_addr(address)
}

#[flux_rs::sig(fn (u32[@addr]) -> bool[is_four_byte_aligned(addr)])]
fn is_four_byte_aligned(address: u32) -> bool {
    if address >= ISER_START && address <= ISER_END {
        (address - ISER_START) % 4 == 0
    } else if address >= ICER_START && address <= ICER_END {
        (address - ICER_START) % 4 == 0
    } else if address >= ISPR_START && address <= ISPR_END {
        (address - ISPR_START) % 4 == 0
    } else if address >= ICPR_START && address <= ICPR_END {
        (address - ICPR_START) % 4 == 0
    } else if address >= IABR_START && address <= IABR_END {
        (address - IABR_START) % 4 == 0
    } else if address >= IPR_START && address <= IPR_END {
        (address - IPR_START) % 4 == 0
    } else {
        false
    }
}

// NVIC
//
// Some unimplemented blocks:
//
// 0xE000E380 -0xE000E3FC	-	-	-	Reserved
//
// 0xE000E7F0 -0xE000ECFC	-	-	-	Reserved
//
//
// Columns are:
// Address	Name	Type	Reset	Description
#[derive(Debug)]
#[flux_rs::refined_by(
    isers: Map<int, int>,
    icers: Map<int, int>,
    isprs: Map<int, int>,
    icprs: Map<int, int>,
    iabrs: Map<int, int>,
    iprs: Map<int, int>
)]
pub struct Nvic {
    // 0xE000E100 -0xE000E13C	NVIC_ISER0 -NVIC_ISER15	RW	0x00000000
    // Interrupt Set-Enable Registers, NVIC_ISER0 - NVIC_ISER15
    #[field(Regs<u32, u32>[isers])]
    isers: NvicRegs,
    // 0xE000E180 -0xE000E1BC	NVIC_ICER0 -NVIC_ICER15	RW	0x00000000
    // Interrupt Clear-Enable Registers, NVIC_ICER0 - NVIC_ICER15
    #[field(Regs<u32, u32>[icers])]
    icers: NvicRegs,
    // 0xE000E200 -0xE000E23C	NVIC_ISPR0 -NVIC_ISPR15	RW	0x00000000
    // Interrupt Set-Pending Registers, NVIC_ISPR0 - NVIC_ISPR15
    #[field(Regs<u32, u32>[isprs])]
    isprs: NvicRegs,
    // 0xE000E280 -0xE000E2BC	NVIC_ICPR0 -NVIC_ICPR15	RW	0x00000000
    // Interrupt Clear-Pending Registers, NVIC_ICPR0 - NVIC_ICPR15
    #[field(Regs<u32, u32>[icprs])]
    icprs: NvicRegs,
    // 0xE000E300 -0xE000E37C	NVIC_IABR0 -NVIC_IABR15	RO	0x00000000
    // Interrupt Active Bit Registers, NVIC_IABR0 - NVIC_IABR15
    #[field(Regs<u32, u32>[iabrs])]
    iabrs: NvicRegs,
    // 0xE000E400 -0xE000E7EC	NVIC_IPR0 -NVIC_IPR123	RW	0x00000000
    // Interrupt Priority Registers, NVIC_IPR0 - NVC_IPR123
    #[field(Regs<u32, u32>[iprs])]
    iprs: NvicRegs,
}

impl Nvic {
    #[flux_rs::sig(fn (&Nvic[@nvic], u32[@addr]) -> u32[map_get(nvic_addr_to_reg_map(addr, nvic), addr)] requires is_valid_nvic_read_addr(addr) && is_four_byte_aligned(addr))]
    pub fn read(&self, addr: u32) -> u32 {
        if !is_four_byte_aligned(addr) {
            panic!("Unaligned read of register")
        }
        match addr {
            ISER_START..=ISER_END => *self.isers.get(&addr).unwrap(),
            ICER_START..=ICER_END => *self.icers.get(&addr).unwrap(),
            ISPR_START..=ISPR_END => *self.isprs.get(&addr).unwrap(),
            ICPR_START..=ICPR_END => *self.icprs.get(&addr).unwrap(),
            IABR_START..=IABR_END => *self.iabrs.get(&addr).unwrap(),
            IPR_START..=IPR_END => *self.iprs.get(&addr).unwrap(),
            _ => panic!("Read of invalid addr"),
        }
    }

    #[flux_rs::sig(
        fn (self: &strg Nvic[@old_nvic], u32[@addr], u32[@value]) 
            requires is_valid_nvic_write_addr(addr) && is_four_byte_aligned(addr)
            ensures self: Nvic { new_nvic: map_set(nvic_addr_to_reg_map(addr, old_nvic), addr, value) == nvic_addr_to_reg_map(addr, new_nvic) }
    )]
    pub fn write(&mut self, addr: u32, value: u32) {
        if !is_four_byte_aligned(addr) {
            panic!("Unaligned write to register")
        }
        match addr {
            ISER_START..=ISER_END => {
                self.isers.set(addr, value);
            }
            ICER_START..=ICER_END => {
                self.icers.set(addr, value);
            }
            ISPR_START..=ISPR_END => {
                self.isprs.set(addr, value);
            }
            ICPR_START..=ICPR_END => {
                self.icprs.set(addr, value);
            }
            IABR_START..=IABR_END => {
                self.iabrs.set(addr, value);
            }
            IPR_START..=IPR_END => {
                self.iprs.set(addr, value);
            }
            _ => panic!("Write to invalid addr"),
        }
    }
}
