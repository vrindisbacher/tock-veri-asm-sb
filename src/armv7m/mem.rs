use crate::internals::rvec::RVec;
//
// The following file implements memory layout for the ARMv7m architecture.
//
// The Armv7-M architecture uses a single, flat address space of 232 8-bit bytes. Byte addresses are treated as unsigned numbers, running from 0 to 2^32 - 1.
//
// Alignment and data access:
//
//  The following data accesses always generate an alignment fault:
//      • Non halfword-aligned LDREXH and STREXH.
//      • Non word-aligned LDREX and STREX.
//      • Non word-aligned LDRD, LDMIA, LDMDB, POP, LDC, VLDR, VLDM, and VPOP.
//      • Non word-aligned STRD, STMIA, STMDB, PUSH, STC, VSTR, VSTM, and VPUSH.
//  The following data accesses support unaligned addressing, and only generate alignment faults when the
//  CCR.UNALIGN_TRP bit is set to 1, see Configuration and Control Register, CCR on page B3-604:
//      • Non halfword-aligned LDR{S}H{T} and STRH{T}.
//      • Non halfword-aligned TBH.
//      • Non word-aligned LDR{T} and STR{T}.
//
// Endianness:
//
// Armv7-M supports a selectable endian model in which, on a reset, a control input determines whether the
// endianness is big endian (BE) or little endian (LE). This endian mapping has the following restrictions:
//      • The endianness setting only applies to data accesses. Instruction fetches are always little endian.
//      • All accesses to the SCS are little endian, see System Control Space (SCS) on page B3-595.
// The AIRCR.ENDIANNESS bit indicates the endianness, see Application Interrupt and Reset Control Register,
// AIRCR on page B3-601.
//
// If an implementation requires support for big endian instruction fetches, it can implement this in the bus fabric. See
// Endian support on page D5-799 for more information.
//
// see page B3-592 of the ARMv7m arch manual for details on the system address map
//
// Memory types:
//
// - Normal Memory: Can be read or write and is idempotent (see p. A3-80 in the manual)
// - Device Memory: Causes side effects
// - Strongly Ordered Memory: An access to memory marked as Strongly Ordered acts as a memory barrier to all other explicit accesses from that processor, until the point at which the access is complete (that is, has changed the state of the target location or data has been returned). In addition, an access to memory marked as Strongly Ordered must complete before the end of a memory barrier

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegion {
    // Normal
    Code,
    // Normal
    SRAM,
    // Device
    Peripheral,
    // NOTE: Blurring over WBWA vs. WT cache here
    // Normal
    RAM,
    // NOTE: Glossing over shareable vs. nonshareable as we only care about single processor
    // Device
    Device,
    // Strongly Ordered
    PPB,
    // Device
    VendorSys,
}

#[derive(Debug)]
#[flux_rs::refined_by(
    code: int,
    sram: int,
    periph: int,
    ram: int,
    device: int,
    ppb: int,
    vendor_sys: int
)]
pub struct Memory {
    #[field(RVec<u8>[code])]
    code: RVec<u8>,
    #[field(RVec<u8>[sram])]
    sram: RVec<u8>,
    #[field(RVec<u8>[periph])]
    peripherals: RVec<u8>,
    #[field(RVec<u8>[ram])]
    ram: RVec<u8>,
    #[field(RVec<u8>[device])]
    device: RVec<u8>,
    #[field(RVec<u8>[ppb])]
    ppb: RVec<u8>,
    #[field(RVec<u8>[vendor_sys])]
    vendor_sys: RVec<u8>,
}

const CODE_START: u32 = 0x0000_0000;
const CODE_END: u32 = 0x1FFF_FFFF;

const SRAM_START: u32 = 0x2000_0000;
const SRAM_END: u32 = 0x3FFF_FFFF;

const PERIPH_START: u32 = 0x4000_0000;
const PERIPH_END: u32 = 0x5FFF_FFFF;

const RAM_START: u32 = 0x6000_0000;
const RAM_END: u32 = 0x9FFF_FFFF;

const DEVICE_START: u32 = 0xA000_0000;
const DEVICE_END: u32 = 0xDFFF_FFFF;

const PPB_START: u32 = 0xE000_0000;
const PPB_END: u32 = 0xE00F_FFFF;

const VENDOR_SYSTEM_START: u32 = 0xE010_0000;
const VENDOR_SYSTEM_END: u32 = 0xFFFF_FFFF;

impl Memory {
    fn get_region(&self, address: u32) -> MemoryRegion {
        match address {
            CODE_START..=CODE_END => MemoryRegion::Code,
            SRAM_START..=SRAM_END => MemoryRegion::SRAM,
            PERIPH_START..=PERIPH_END => MemoryRegion::Peripheral,
            RAM_START..=RAM_END => MemoryRegion::RAM,
            DEVICE_START..=DEVICE_END => MemoryRegion::Device,
            PPB_START..=PPB_END => MemoryRegion::PPB,
            VENDOR_SYSTEM_START..=VENDOR_SYSTEM_END => MemoryRegion::VendorSys,
        }
    }

    fn read_code(&self, address: u32) -> u8 {
        *self.code.get(address as usize)
    }

    fn read_sram(&self, address: u32) -> u8 {
        let idx = (address - SRAM_START) as usize;
        *self.sram.get(idx)
    }

    fn read_peripherals(&self, address: u32) -> u8 {
        let idx = (address - PERIPH_START) as usize;
        *self.peripherals.get(idx)
    }

    fn read_ram(&self, address: u32) -> u8 {
        let idx = (address - RAM_START) as usize;
        *self.ram.get(idx)
    }

    fn read_device(&self, address: u32) -> u8 {
        let idx = (address - DEVICE_START) as usize;
        *self.ram.get(idx)
    }

    fn read_ppb(&self, address: u32) -> u8 {
        // Accesses to PPB are always little endian and word access only (see B3.1.1 in manual)
        // TODO: See section B3.2.2 for the specific registers here
        let idx = (address - PPB_START) as usize;
        *self.ppb.get(idx)
    }

    fn read_vendor_sys(&self, address: u32) -> u8 {
        let idx = (address - VENDOR_SYSTEM_START) as usize;
        *self.vendor_sys.get(idx)
    }

    pub fn read(&self, address: u32) -> u8 {
        assert!(address <= std::u32::MAX);
        match self.get_region(address) {
            MemoryRegion::Code => self.read_code(address),
            MemoryRegion::SRAM => self.read_sram(address),
            // TODO: This is device dependent. Should we allow reads?
            MemoryRegion::Peripheral => self.read_peripherals(address),
            MemoryRegion::RAM => self.read_ram(address),
            // TODO: This is vendor dependent. Should we allow reads?
            MemoryRegion::Device => self.read_device(address),
            MemoryRegion::PPB => self.read_ppb(address),
            MemoryRegion::VendorSys => self.read_vendor_sys(address),
        }
    }

    fn write_sram(&mut self, address: u32, value: u8) {
        let idx = (address - SRAM_START) as usize;
        self.sram[idx] = value;
    }

    fn write_peripherals(&mut self, address: u32, value: u8) {
        let idx = (address - PERIPH_START) as usize;
        self.sram[idx] = value;
    }

    fn write_ram(&mut self, address: u32, value: u8) {
        let idx = (address - RAM_START) as usize;
        self.sram[idx] = value;
    }

    fn write_device(&mut self, address: u32, value: u8) {
        let idx = (address - DEVICE_START) as usize;
        self.sram[idx] = value;
    }

    fn write_ppb(&mut self, address: u32, value: u8) {
        // Accesses to PPB are always little endian and word access only (see B3.1.1 in manual)
        let idx = (address - PPB_START) as usize;
        self.ppb[idx] = value;
    }

    fn write_vendor_sys(&mut self, address: u32, value: u8) {
        let idx = (address - PPB_START) as usize;
        self.vendor_sys[idx] = value;
    }

    pub fn write(&mut self, address: u32, value: u8) {
        assert!(address <= std::u32::MAX);
        let region = self.get_region(address);
        assert!(region != MemoryRegion::Code);
        match region {
            MemoryRegion::Code => panic!("Can't write to code"),
            MemoryRegion::SRAM => self.write_sram(address, value),
            // TODO: This is device dependent. Should we allow reads?
            MemoryRegion::Peripheral => self.write_peripherals(address, value),
            MemoryRegion::RAM => self.write_ram(address, value),
            MemoryRegion::PPB => self.write_ppb(address, value),
            MemoryRegion::VendorSys => self.write(address, value),
            // TODO: This is device dependent. Should we allow reads?
            MemoryRegion::Device => self.write_device(address, value),
        }
    }
}
