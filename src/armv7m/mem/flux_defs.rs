use super::mpu;
use super::nvic;
use super::sys_control;
use super::sys_tick;
use super::{
    Memory, Ppb, INTERRUPT_AUXILIARY_CONTROL_REGISTER_END,
    INTERRUPT_AUXILIARY_CONTROL_REGISTER_START, MPU_END, MPU_START, NVIC_END, NVIC_START, PPB_END,
    PPB_START, SW_TRIGGER_INTERRUPT_REG_END, SW_TRIGGER_INTERRUPT_REG_START,
    SYSTEM_CONTROL_BLOCK_END, SYSTEM_CONTROL_BLOCK_START, SYS_TICK_END, SYS_TICK_START,
};

pub mod mpu_defs {
    use super::mpu::{
        Mpu, MPU_CTRL_ADDR, MPU_RASR_A1_ADDR, MPU_RASR_A2_ADDR, MPU_RASR_A3_ADDR, MPU_RASR_ADDR,
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

        fn mpu_addr_into_reg(address: int, mpu: Mpu) -> int {
            if address == MPU_TYPE_ADDR {
                mpu.mpu_type
            } else if address == MPU_CTRL_ADDR {
                mpu.mpu_ctrl
            } else if address == MPU_RNR_ADDR {
                mpu.mpu_rnr
            } else if address == MPU_RBAR_ADDR {
                mpu.mpu_rbar
            } else if address == MPU_RASR_ADDR {
                mpu.mpu_rasr
            } else if address == MPU_RBAR_A1_ADDR {
                mpu.mpu_rbar_a1
            } else if address == MPU_RASR_A1_ADDR {
                mpu.mpu_rasr_a1
            } else if address == MPU_RBAR_A2_ADDR {
                mpu.mpu_rbar_a2
            } else if address == MPU_RASR_A2_ADDR {
                mpu.mpu_rasr_a2
            } else if address == MPU_RBAR_A3_ADDR {
                mpu.mpu_rbar_a3
            } else if address == MPU_RASR_A3_ADDR {
                mpu.mpu_rasr_a3
            } else {
                -1
            }
        }
    }
}

pub mod sys_tick_defs {

    use super::sys_tick::{SysTick, SYST_CALIB_ADDR, SYST_CSR_ADDR, SYST_CVR_ADDR, SYST_RVR_ADDR};

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

        fn sys_tick_addr_into_reg(address: int, sys_tick: SysTick) -> int {
            if address == SYST_CSR_ADDR {
                sys_tick.syst_csr
            } else if address == SYST_RVR_ADDR  {
                sys_tick.syst_rvr
            } else if address == SYST_CVR_ADDR {
                sys_tick.syst_cvr
            } else if address == SYST_CALIB_ADDR {
                sys_tick.syst_calib
            } else {
                -1
            }
        }
    }
}

pub mod sys_control_block_defs {
    use super::sys_control::{
        SysControlBlock, AFSR_ADDR, AIRCR_ADDR, BFAR_ADDR, CCR_ADDR, CFSR_ADDR, CPACR_ADDR,
        CPUID_ADDR, DFSR_ADDR, HFSR_ADDR, ICSR_ADDR, MMFAR_ADDR, SCR_ADDR, SHCSR_ADDR, SHPR1_ADDR,
        SHPR2_ADDR, SHPR3_ADDR, VTOR_ADDR,
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

        fn sys_control_block_addr_into_reg(address: int, sys_control: SysControlBlock) -> int {
            if address == CPUID_ADDR {
                sys_control.cpuid
            } else if address == ICSR_ADDR {
                sys_control.icsr
            } else if address == VTOR_ADDR {
                sys_control.vtor
            } else if address == AIRCR_ADDR {
                sys_control.aircr
            } else if address == SCR_ADDR {
                sys_control.scr
            } else if address == CCR_ADDR {
                sys_control.ccr
            } else if address == SHPR1_ADDR {
                sys_control.shpr1
            } else if address == SHPR2_ADDR {
                sys_control.shpr2
            } else if address == SHPR3_ADDR {
                sys_control.shpr3
            } else if address == SHCSR_ADDR {
                sys_control.shcsr
            } else if address == CFSR_ADDR {
                sys_control.cfsr
            } else if address == HFSR_ADDR {
                sys_control.hfsr
            } else if address == DFSR_ADDR {
                sys_control.dfsr
            } else if address == MMFAR_ADDR {
                sys_control.mmfar
            } else if address == BFAR_ADDR {
                sys_control.bfar
            } else if address == AFSR_ADDR {
                sys_control.afsr
            } else if address == CPACR_ADDR {
                sys_control.cpacr
            }  else {
                -1
            }
        }
    }
}

pub mod sys_control_id_reg_defs {
    use super::sys_control::{
        SysControlIDReg, ACTLR_ADDR, CID0_ADDR, CID1_ADDR, CID2_ADDR, CID3_ADDR, ICTR_ADDR,
        PID0_ADDR, PID1_ADDR, PID2_ADDR, PID3_ADDR, PID4_ADDR, PID5_ADDR, PID6_ADDR, PID7_ADDR,
        STIR_ADDR,
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

            fn sys_control_id_reg_addr_into_reg(address: int, sys_control_id: SysControlIDReg) -> int {
                if address == ICTR_ADDR {
                    sys_control_id.ictr
                } else if address == ACTLR_ADDR {
                    sys_control_id.actlr
                } else if address == STIR_ADDR {
                    sys_control_id.stir
                } else if address == PID4_ADDR {
                    sys_control_id.pid4
                } else if address == PID5_ADDR {
                    sys_control_id.pid5
                } else if address == PID6_ADDR {
                    sys_control_id.pid6
                } else if address == PID7_ADDR {
                    sys_control_id.pid7
                } else if address == PID0_ADDR {
                    sys_control_id.pid0
                } else if address == PID1_ADDR {
                    sys_control_id.pid1
                } else if address == PID2_ADDR {
                    sys_control_id.pid2
                } else if address == PID3_ADDR {
                    sys_control_id.pid3
                } else if address == CID0_ADDR {
                    sys_control_id.cid0
                } else if address == CID1_ADDR {
                    sys_control_id.cid1
                } else if address == CID2_ADDR {
                    sys_control_id.cid2
                } else {
                    // if address == CID3_ADDR {
                    sys_control_id.cid3
                }
            }
    }
}

pub mod sys_control_space_defs {
    use super::sys_control::{
        SysControlBlock, SysControlIDReg, SysControlSpace, ACTLR_ADDR, AFSR_ADDR, AIRCR_ADDR,
        BFAR_ADDR, CCR_ADDR, CFSR_ADDR, CID0_ADDR, CID1_ADDR, CID2_ADDR, CID3_ADDR, CPACR_ADDR,
        CPUID_ADDR, DFSR_ADDR, HFSR_ADDR, ICSR_ADDR, ICTR_ADDR, MMFAR_ADDR, PID0_ADDR, PID1_ADDR,
        PID2_ADDR, PID3_ADDR, PID4_ADDR, PID5_ADDR, PID6_ADDR, PID7_ADDR, SCR_ADDR, SHCSR_ADDR,
        SHPR1_ADDR, SHPR2_ADDR, SHPR3_ADDR, STIR_ADDR, VTOR_ADDR,
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

            fn check_sys_control_space_value_read(address: int, sys_control: SysControlSpace, value: int) -> bool {
                if is_valid_sys_control_id_reg_read_addr(address) {
                    sys_control_id_reg_addr_into_reg(address, sys_control.sys_control_id_reg) == value
                } else if is_valid_sys_control_block_read_addr(address) {
                    sys_control_block_addr_into_reg(address, sys_control.sys_control_block) == value
                } else {
                    false
                }
            }

            fn check_sys_control_space_value_write(address: int, sys_control: SysControlSpace, value: int) -> bool {
                if is_valid_sys_control_id_reg_write_addr(address) {
                    sys_control_id_reg_addr_into_reg(address, sys_control.sys_control_id_reg) == value
                } else if is_valid_sys_control_block_write_addr(address) {
                    sys_control_block_addr_into_reg(address, sys_control.sys_control_block) == value
                } else {
                    false
                }
            }
    }
}

pub mod nvic_defs {
    use super::nvic::{
        Nvic, IABR_END, IABR_START, ICER_END, ICER_START, ICPR_END, ICPR_START, IPR_END, IPR_START,
        ISER_END, ISER_START, ISPR_END, ISPR_START,
    };

    flux_rs::defs! {
        fn map_set<K, V>(m:Map<K, V>, k: K, v: V) -> Map<K, V> { map_store(m, k, v) }
        fn map_get<K, V>(m: Map<K, V>, k:K) -> V { map_select(m, k) }
    }

    #[derive(Debug)]
    #[flux_rs::opaque]
    #[flux_rs::refined_by(vals: Map<int, int>)]
    pub struct RegMap {
        inner: std::collections::HashMap<u32, u32>,
    }

    impl RegMap {
        #[flux_rs::trusted]
        #[flux_rs::sig(fn(self: &strg RegMap[@m], k: u32, v: u32) ensures self: RegMap[map_set(m.vals, k, v)])]
        pub fn set(&mut self, k: u32, v: u32) {
            self.inner.insert(k, v);
        }

        #[flux_rs::trusted]
        #[flux_rs::sig(fn(&RegMap[@m], &u32[@k]) -> Option<&u32[map_get(m.vals, k)]>)]
        pub fn get(&self, k: &u32) -> Option<&u32> {
            self.inner.get(k)
        }
    }

    flux_rs::defs! {
            // all addresses are read / write
            fn is_valid_nvic_addr(address: int) -> bool {
                (address >= ISER_START && address <= ISER_END)
                ||
                (address >= ICER_START && address <= ICER_END)
                ||
                (address >= ISPR_START && address <= ISPR_END)
                ||
                (address >= ICPR_START && address <= ICPR_END)
                ||
                (address >= IABR_START && address <= IABR_END)
                ||
                (address >= IPR_START && address <= IPR_END)
            }

            fn is_valid_nvic_read_addr(address: int) -> bool {
                // all read
                is_valid_nvic_addr(address)
            }

            fn is_valid_nvic_write_addr(address: int) -> bool {
                // all write
                is_valid_nvic_addr(address)
            }

            fn nvic_addr_to_reg_map(address: int, nvic: Nvic) -> Map<int, int> {
                if (address >= ISER_START && address <= ISER_END) {
                    nvic.isers
                } else if (address >= ICER_START && address <= ICER_END) {
                    nvic.icers
                } else if (address >= ISPR_START && address <= ISPR_END) {
                    nvic.isprs
                } else if (address >= ICPR_START && address <= ICPR_END) {
                    nvic.icprs
                } else if (address >= IABR_START && address <= IABR_END) {
                    nvic.iabrs
                } else {
                    // (address >= IPR_START && address <= IPR_END)
                    nvic.iprs
                }
            }

            fn is_four_byte_aligned(address: int) -> bool {
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
                } else {
                    // (address >= IPR_START && address <= IPR_END)
                    (address - IPR_START) % 4 == 0
                }
            }
    }
}

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

    fn check_ppb_value_read(address: int, ppb: Ppb, value: int) -> bool {
        if is_valid_sys_control_space_read_addr(address) {
            check_sys_control_space_value_read(address, ppb.sys_control, value)
        } else if is_valid_nvic_read_addr(address) {
            map_get(nvic_addr_to_reg_map(address, ppb.nvic), address) == value
        } else if is_valid_mpu_read_addr(address) {
            mpu_addr_into_reg(address, ppb.mpu) == value
        } else if is_valid_sys_tick_read_addr(address) {
            sys_tick_addr_into_reg(address, ppb.sys_tick) == value
        } else {
            false
        }
    }

    fn check_ppb_value_write(address: int, ppb: Ppb, value: int) -> bool {
        if is_valid_sys_control_space_write_addr(address) {
            check_sys_control_space_value_write(address, ppb.sys_control, value)
        } else if is_valid_nvic_write_addr(address) {
            map_get(nvic_addr_to_reg_map(address, ppb.nvic), address) == value
        } else if is_valid_mpu_write_addr(address) {
            mpu_addr_into_reg(address, ppb.mpu) == value
        } else if is_valid_sys_tick_write_addr(address) {
            sys_tick_addr_into_reg(address, ppb.sys_tick) == value
        } else {
            false
        }

    }

    fn check_mem_value_read(address: int, mem: Memory, value: int) -> bool {
        check_ppb_value_read(address, mem.ppb, value)
    }

    fn check_mem_value_write(address: int, mem: Memory, value: int) -> bool {
        check_ppb_value_write(address, mem.ppb, value)
    }
}
