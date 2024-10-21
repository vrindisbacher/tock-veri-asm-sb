use super::{
    INTERRUPT_AUXILIARY_CONTROL_REGISTER_END, INTERRUPT_AUXILIARY_CONTROL_REGISTER_START, MPU_END,
    MPU_START, NVIC_END, NVIC_START, PPB_END, PPB_START, SW_TRIGGER_INTERRUPT_REG_END,
    SW_TRIGGER_INTERRUPT_REG_START, SYSTEM_CONTROL_BLOCK_END, SYSTEM_CONTROL_BLOCK_START,
    SYS_TICK_END, SYS_TICK_START,
};

flux_rs::defs! {
    fn in_ppb(address: int) -> bool {
        address >= PPB_START && address <= PPB_END
    }

    fn in_ppb(address: int) -> bool {
        in_system_control(address) || in_systick(address) || in_nvic(address) || in_mpu(address)
    }

    fn in_system_control(address: int) -> bool {
        (address >= INTERRUPT_AUXILIARY_CONTROL_REGISTER_START && address <= INTERRUPT_AUXILIARY_CONTROL_REGISTER_END)
        ||
        (address >= SYSTEM_CONTROL_BLOCK_START && address <= SYSTEM_CONTROL_BLOCK_END)
        ||
        (address >= SW_TRIGGER_INTERRUPT_REG_START && address <= SW_TRIGGER_INTERRUPT_REG_END)
    }

    fn in_systick(address: int) -> bool {
        (address >= SYS_TICK_START && address <= SYS_TICK_END)
    }

    fn in_mpu(address: int) -> bool {
        (address >= MPU_START && address <= MPU_END)
    }

    fn in_nvic(address: int) -> bool {
        (address >= NVIC_START && address <= NVIC_END)
    }

    // want two functions here - read right bits + write right bits
    // fn addr_into_field(address: int, mem: Memory) -> int {
    // }
}
