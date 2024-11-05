use super::mpu;
use super::nvic;
use super::sys_control;
use super::sys_tick;
use super::{
    Memory, INTERRUPT_AUXILIARY_CONTROL_REGISTER_END, INTERRUPT_AUXILIARY_CONTROL_REGISTER_START,
    MPU_END, MPU_START, NVIC_END, NVIC_START, PPB_END, PPB_START, SW_TRIGGER_INTERRUPT_REG_END,
    SW_TRIGGER_INTERRUPT_REG_START, SYSTEM_CONTROL_BLOCK_END, SYSTEM_CONTROL_BLOCK_START,
    SYS_TICK_END, SYS_TICK_START,
};

pub mod mpu_defs {
    use super::mpu::{
        MPU_CTRL_ADDR, MPU_RASR_A1_ADDR, MPU_RASR_A2_ADDR, MPU_RASR_A3_ADDR, MPU_RASR_ADDR,
        MPU_RBAR_A1_ADDR, MPU_RBAR_A2_ADDR, MPU_RBAR_A3_ADDR, MPU_RBAR_ADDR, MPU_RNR_ADDR,
        MPU_TYPE_ADDR,
    };

    flux_rs::defs! {
        fn is_valid_mpu_read_addr(address: int) -> bool {
            // all address are read
            address == MPU_TYPE_ADDR
                || address == MPU_CTRL_ADDR
                || address == MPU_RNR_ADDR
                || address == MPU_RBAR_ADDR
                || address == MPU_RASR_ADDR
                || address == MPU_RBAR_A1_ADDR
                || address == MPU_RASR_A1_ADDR
                || address == MPU_RBAR_A2_ADDR
                || address == MPU_RASR_A2_ADDR
                || address == MPU_RBAR_A3_ADDR
                || address == MPU_RASR_A3_ADDR
        }

        fn is_valid_mpu_write_addr(address: int) -> bool {
            // all address except MPU_TYPE are write
            address == MPU_CTRL_ADDR
                || address == MPU_RNR_ADDR
                || address == MPU_RBAR_ADDR
                || address == MPU_RASR_ADDR
                || address == MPU_RBAR_A1_ADDR
                || address == MPU_RASR_A1_ADDR
                || address == MPU_RBAR_A2_ADDR
                || address == MPU_RASR_A2_ADDR
                || address == MPU_RBAR_A3_ADDR
                || address == MPU_RASR_A3_ADDR
        }
    }
}

pub mod sys_tick_defs {

    use super::sys_tick::{SYST_CALIB_ADDR, SYST_CSR_ADDR, SYST_CVR_ADDR, SYST_RVR_ADDR};

    flux_rs::defs! {

        fn is_valid_sys_tick_read_addr(address: int) -> bool {
            // all addresses are read
            address == SYST_CSR_ADDR ||
            address == SYST_RVR_ADDR ||
            address == SYST_CVR_ADDR ||
            address == SYST_CALIB_ADDR
        }

        fn is_valid_sys_tick_write_addr(address: int) -> bool {
            // all addresses but SYS_CALIB are write
            address == SYST_CSR_ADDR ||
            address == SYST_RVR_ADDR ||
            address == SYST_CVR_ADDR
        }
    }
}

pub mod sys_control_block_defs {
    use super::sys_control::{
        AFSR_ADDR, AIRCR_ADDR, BFAR_ADDR, CCR_ADDR, CFSR_ADDR, CPACR_ADDR, CPUID_ADDR, DFSR_ADDR,
        HFSR_ADDR, ICSR_ADDR, MMFAR_ADDR, SCR_ADDR, SHCSR_ADDR, SHPR1_ADDR, SHPR2_ADDR, SHPR3_ADDR,
        VTOR_ADDR,
    };

    flux_rs::defs! {
        fn is_valid_sys_control_block_read_addr(address: int) -> bool {
            // all addresses are read
            address == CPUID_ADDR ||
                address == ICSR_ADDR ||
                address == VTOR_ADDR ||
                address == AIRCR_ADDR ||
                address == SCR_ADDR ||
                address == CCR_ADDR ||
                address == SHPR1_ADDR ||
                address == SHPR2_ADDR ||
                address == SHPR3_ADDR ||
                address == SHCSR_ADDR ||
                address == CFSR_ADDR ||
                address == HFSR_ADDR ||
                address == DFSR_ADDR ||
                address == MMFAR_ADDR ||
                address == BFAR_ADDR ||
                address == AFSR_ADDR ||
                address == CPACR_ADDR
        }

        fn is_valid_sys_control_block_write_addr(address: int) -> bool {
            // all addresses but CPUID are write
            address == ICSR_ADDR ||
                address == VTOR_ADDR ||
                address == AIRCR_ADDR ||
                address == SCR_ADDR ||
                address == CCR_ADDR ||
                address == SHPR1_ADDR ||
                address == SHPR2_ADDR ||
                address == SHPR3_ADDR ||
                address == SHCSR_ADDR ||
                address == CFSR_ADDR ||
                address == HFSR_ADDR ||
                address == DFSR_ADDR ||
                address == MMFAR_ADDR ||
                address == BFAR_ADDR ||
                address == AFSR_ADDR ||
                address == CPACR_ADDR
        }
    }
}

pub mod sys_control_id_reg_defs {
    use super::sys_control::{
        ACTLR_ADDR, CID0_ADDR, CID1_ADDR, CID2_ADDR, CID3_ADDR, ICTR_ADDR, PID0_ADDR, PID1_ADDR,
        PID2_ADDR, PID3_ADDR, PID4_ADDR, PID5_ADDR, PID6_ADDR, PID7_ADDR, STIR_ADDR,
    };

    flux_rs::defs! {
            fn is_valid_sys_control_id_reg_read_addr(address: int) -> bool {
                // all but STIR are read
                address == ICTR_ADDR ||
                    address == ACTLR_ADDR ||
                    address == PID4_ADDR ||
                    address == PID5_ADDR ||
                    address == PID6_ADDR ||
                    address == PID7_ADDR ||
                    address == PID0_ADDR ||
                    address == PID1_ADDR ||
                    address == PID2_ADDR ||
                    address == PID3_ADDR ||
                    address == CID0_ADDR ||
                    address == CID1_ADDR ||
                    address == CID2_ADDR ||
                    address == CID3_ADDR
            }

            fn is_valid_sys_control_id_reg_write_addr(address: int) -> bool {
                // only actlr && stir are write
                address == ACTLR_ADDR || address == STIR_ADDR
            }
    }
}

pub mod sys_control_space_defs {
    use super::sys_control::{
        ACTLR_ADDR, AFSR_ADDR, AIRCR_ADDR, BFAR_ADDR, CCR_ADDR, CFSR_ADDR, CID0_ADDR, CID1_ADDR,
        CID2_ADDR, CID3_ADDR, CPACR_ADDR, CPUID_ADDR, DFSR_ADDR, HFSR_ADDR, ICSR_ADDR, ICTR_ADDR,
        MMFAR_ADDR, PID0_ADDR, PID1_ADDR, PID2_ADDR, PID3_ADDR, PID4_ADDR, PID5_ADDR, PID6_ADDR,
        PID7_ADDR, SCR_ADDR, SHCSR_ADDR, SHPR1_ADDR, SHPR2_ADDR, SHPR3_ADDR, STIR_ADDR, VTOR_ADDR,
    };
    use super::sys_control_block_defs::*;
    use super::sys_control_block_defs::*;

    flux_rs::defs! {
        fn is_valid_sys_control_space_read_addr(address: int) -> bool {
            is_valid_sys_control_block_read_addr(address) || is_valid_sys_control_id_reg_read_addr(address)
        }

        fn is_valid_sys_control_space_write_addr(address: int) -> bool {
            is_valid_sys_control_block_write_addr(address) || is_valid_sys_control_id_reg_write_addr(address)
        }
    }
}

pub mod nvic_defs {
    use super::nvic::{
        IABR_END, IABR_START, ICER_END, ICER_START, ICPR_END, ICPR_START, IPR_END, IPR_START,
        ISER_END, ISER_START, ISPR_END, ISPR_START,
    };
    use crate::flux_support::*;

    flux_rs::defs! {
            // all addresses are read / write as long as they are 4 byte aligned
            fn is_valid_nvic_addr(address: int) -> bool {
                if (address >= ISER_START && address <= ISER_END) {
                   (address - ISER_START) % 4 == 0
                } else if (address >= ICER_START && address <= ICER_END) {
                    (address - ICER_START) % 4 == 0
                } else if (address >= ISPR_START && address <= ISPR_END) {
                    (address - ISPR_START) % 4 == 0
                } else if (address >= ICPR_START && address <= ICPR_END) {
                    (address - ICPR_START) % 4 == 0
                } else if (address >= IABR_START && address <= IABR_END) {
                    (address - IABR_START) % 4 == 0
                } else if (address >= IPR_START && address <= IPR_END) {
                    (address - IPR_START) % 4 == 0
                } else {
                    false
                }
            }

            fn is_valid_nvic_read_addr(address: int) -> bool {
                // all read
                is_valid_nvic_addr(address)
            }

            fn is_valid_nvic_write_addr(address: int) -> bool {
                // all write
                is_valid_nvic_addr(address)
            }
    }
}

use crate::flux_support::bv32::BV32;
use mpu_defs::*;
use nvic_defs::*;
use sys_control_space_defs::*;
use sys_tick_defs::*;

flux_rs::defs! {
    fn is_valid_read_addr(address: int) -> bool {
        is_valid_sys_control_space_read_addr(address)
        ||
        is_valid_nvic_read_addr(address)
        ||
        is_valid_mpu_read_addr(address)
        ||
        is_valid_sys_tick_read_addr(address)
    }

    fn is_valid_write_addr(address: int) -> bool {
        is_valid_sys_control_space_write_addr(address)
        ||
        is_valid_nvic_write_addr(address)
        ||
        is_valid_mpu_write_addr(address)
        ||
        is_valid_sys_tick_write_addr(address)
    }

    fn get_mem_addr(address: int, mem: Memory) -> BV32 {
        map_get(mem, address)
    }

    fn mem_value_updated(address: int, old_mem: Memory, new_mem: Memory, value: BV32) -> bool {
        map_set(old_mem, address, value) == new_mem
    }
}
