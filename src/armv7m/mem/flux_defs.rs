use super::{
    mpu::{
        MPU_CTRL_ADDR, MPU_RASR_A1_ADDR, MPU_RASR_A2_ADDR, MPU_RASR_ADDR, MPU_RBAR_A1_ADDR,
        MPU_RBAR_A2_ADDR, MPU_RBAR_A3_ADDR, MPU_RBAR_ADDR, MPU_RNR_ADDR, MPU_TYPE_ADDR,
    },
    nvic::{
        IABR0_ADDR, IABR10_ADDR, IABR11_ADDR, IABR12_ADDR, IABR13_ADDR, IABR14_ADDR, IABR15_ADDR,
        IABR1_ADDR, IABR2_ADDR, IABR3_ADDR, IABR4_ADDR, IABR5_ADDR, IABR6_ADDR, IABR7_ADDR,
        IABR8_ADDR, IABR9_ADDR, ICER0_ADDR, ICER10_ADDR, ICER11_ADDR, ICER12_ADDR, ICER13_ADDR,
        ICER14_ADDR, ICER15_ADDR, ICER1_ADDR, ICER2_ADDR, ICER3_ADDR, ICER4_ADDR, ICER5_ADDR,
        ICER6_ADDR, ICER7_ADDR, ICER8_ADDR, ICER9_ADDR, ICPR0_ADDR, ICPR10_ADDR, ICPR11_ADDR,
        ICPR12_ADDR, ICPR13_ADDR, ICPR14_ADDR, ICPR15_ADDR, ICPR1_ADDR, ICPR2_ADDR, ICPR3_ADDR,
        ICPR4_ADDR, ICPR5_ADDR, ICPR6_ADDR, ICPR7_ADDR, ICPR8_ADDR, ICPR9_ADDR, IPR0_ADDR,
        IPR100_ADDR, IPR101_ADDR, IPR102_ADDR, IPR103_ADDR, IPR104_ADDR, IPR105_ADDR, IPR106_ADDR,
        IPR107_ADDR, IPR108_ADDR, IPR109_ADDR, IPR10_ADDR, IPR110_ADDR, IPR111_ADDR, IPR112_ADDR,
        IPR113_ADDR, IPR114_ADDR, IPR115_ADDR, IPR116_ADDR, IPR117_ADDR, IPR118_ADDR, IPR119_ADDR,
        IPR11_ADDR, IPR120_ADDR, IPR121_ADDR, IPR122_ADDR, IPR123_ADDR, IPR12_ADDR, IPR13_ADDR,
        IPR14_ADDR, IPR15_ADDR, IPR16_ADDR, IPR17_ADDR, IPR18_ADDR, IPR19_ADDR, IPR1_ADDR,
        IPR20_ADDR, IPR21_ADDR, IPR22_ADDR, IPR23_ADDR, IPR24_ADDR, IPR25_ADDR, IPR26_ADDR,
        IPR27_ADDR, IPR28_ADDR, IPR29_ADDR, IPR2_ADDR, IPR30_ADDR, IPR31_ADDR, IPR32_ADDR,
        IPR33_ADDR, IPR34_ADDR, IPR35_ADDR, IPR36_ADDR, IPR37_ADDR, IPR38_ADDR, IPR39_ADDR,
        IPR3_ADDR, IPR40_ADDR, IPR41_ADDR, IPR42_ADDR, IPR43_ADDR, IPR44_ADDR, IPR45_ADDR,
        IPR46_ADDR, IPR47_ADDR, IPR48_ADDR, IPR49_ADDR, IPR4_ADDR, IPR50_ADDR, IPR51_ADDR,
        IPR52_ADDR, IPR53_ADDR, IPR54_ADDR, IPR55_ADDR, IPR56_ADDR, IPR57_ADDR, IPR58_ADDR,
        IPR59_ADDR, IPR5_ADDR, IPR60_ADDR, IPR61_ADDR, IPR62_ADDR, IPR63_ADDR, IPR64_ADDR,
        IPR65_ADDR, IPR66_ADDR, IPR67_ADDR, IPR68_ADDR, IPR69_ADDR, IPR6_ADDR, IPR70_ADDR,
        IPR71_ADDR, IPR72_ADDR, IPR73_ADDR, IPR74_ADDR, IPR75_ADDR, IPR76_ADDR, IPR77_ADDR,
        IPR78_ADDR, IPR79_ADDR, IPR7_ADDR, IPR80_ADDR, IPR81_ADDR, IPR82_ADDR, IPR83_ADDR,
        IPR84_ADDR, IPR85_ADDR, IPR86_ADDR, IPR87_ADDR, IPR88_ADDR, IPR89_ADDR, IPR8_ADDR,
        IPR90_ADDR, IPR91_ADDR, IPR92_ADDR, IPR93_ADDR, IPR94_ADDR, IPR95_ADDR, IPR96_ADDR,
        IPR97_ADDR, IPR98_ADDR, IPR99_ADDR, IPR9_ADDR, ISER0_ADDR, ISER10_ADDR, ISER11_ADDR,
        ISER12_ADDR, ISER13_ADDR, ISER14_ADDR, ISER15_ADDR, ISER1_ADDR, ISER2_ADDR, ISER3_ADDR,
        ISER4_ADDR, ISER5_ADDR, ISER6_ADDR, ISER7_ADDR, ISER8_ADDR, ISER9_ADDR, ISPR0_ADDR,
        ISPR10_ADDR, ISPR11_ADDR, ISPR12_ADDR, ISPR13_ADDR, ISPR14_ADDR, ISPR15_ADDR, ISPR1_ADDR,
        ISPR2_ADDR, ISPR3_ADDR, ISPR4_ADDR, ISPR5_ADDR, ISPR6_ADDR, ISPR7_ADDR, ISPR8_ADDR,
        ISPR9_ADDR,
    },
    sys_control::{
        ACTLR_ADDR, AFSR_ADDR, AIRCR_ADDR, BFAR_ADDR, CCR_ADDR, CFSR_ADDR, CID0_ADDR, CID1_ADDR,
        CID2_ADDR, CID3_ADDR, CPACR_ADDR, CPUID_ADDR, DFSR_ADDR, HFSR_ADDR, ICSR_ADDR, ICTR_ADDR,
        MMFAR_ADDR, PID0_ADDR, PID1_ADDR, PID2_ADDR, PID3_ADDR, PID4_ADDR, PID5_ADDR, PID6_ADDR,
        PID7_ADDR, SCR_ADDR, SHCSR_ADDR, SHPR1_ADDR, SHPR2_ADDR, SHPR3_ADDR, STIR_ADDR, VTOR_ADDR,
    },
    sys_tick::{SYST_CVR_ADDR, SYST_CALIB_ADDR, SYST_CSR_ADDR, SYST_RVR_ADDR},
    Memory, INTERRUPT_AUXILIARY_CONTROL_REGISTER_END, INTERRUPT_AUXILIARY_CONTROL_REGISTER_START,
    MPU_END, MPU_START, NVIC_END, NVIC_START, PPB_END, PPB_START, SW_TRIGGER_INTERRUPT_REG_END,
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
    fn addr_into_field(address: int, mem: Memory) -> int {
        if address == ICTR_ADDR {
            mem.ictr
        } else if address == ACTLR_ADDR {
            mem.actlr
        } else if address == STIR_ADDR {
            mem.stir
        } else if address == PID4_ADDR {
            mem.pid4
        } else if address == PID5_ADDR {
            mem.pid5
        } else if address == PID6_ADDR {
            mem.pid6
        } else if address == PID7_ADDR {
            mem.pid7
        } else if address == PID0_ADDR {
            mem.pid0
        } else if address == PID1_ADDR {
            mem.pid1
        } else if address == PID2_ADDR {
            mem.pid2
        } else if address == PID3_ADDR {
            mem.pid3
        } else if address == CID0_ADDR {
            mem.cid0
        } else if address == CID1_ADDR {
            mem.cid1
        } else if address == CID2_ADDR {
            mem.cid2
        } else if address == CID3_ADDR {
            mem.cid3
        } else if address == CPUID_ADDR {
            mem.cpuid
        } else if address == ICSR_ADDR {
            mem.icsr
        } else if address == VTOR_ADDR {
            mem.vtor
        } else if address == AIRCR_ADDR {
            mem.aircr
        } else if address == SCR_ADDR {
            mem.scr
        } else if address == CCR_ADDR {
            mem.ccr
        } else if address == SHPR1_ADDR {
            mem.shpr1
        } else if address == SHPR2_ADDR {
            mem.shpr2
        } else if address == SHPR3_ADDR {
            mem.shpr3
        } else if address == SHCSR_ADDR {
            mem.shcsr
        } else if address == CFSR_ADDR {
            mem.cfsr
        } else if address == HFSR_ADDR {
            mem.hfsr
        } else if address == DFSR_ADDR {
            mem.dfsr
        } else if address == MMFAR_ADDR {
            mem.mmfar
        } else if address == BFAR_ADDR {
            mem.bfar
        } else if address == AFSR_ADDR {
            mem.afsr
        } else if address == CPACR_ADDR {
            mem.cpacr
        } else if address == SYST_CSR_ADDR {
            mem.syst_csr
        } else if address == SYST_RVR_ADDR {
            mem.syst_rvr
        } else if address == SYST_CVR_ADDR {
            mem.syst_cvr
        } else if address == SYST_CALIB_ADDR {
            mem.syst_calib
        } else if address == ISER0_ADDR {
            mem.iser0
        } else if address == ISER1_ADDR {
            mem.iser1
        } else if address == ISER2_ADDR {
            mem.iser2
        } else if address == ISER3_ADDR {
            mem.iser3
        } else if address == ISER4_ADDR {
            mem.iser4
        } else if address == ISER5_ADDR {
            mem.iser5
        } else if address == ISER6_ADDR {
            mem.iser6
        } else if address == ISER7_ADDR {
            mem.iser7
        } else if address == ISER8_ADDR {
            mem.iser8
        } else if address == ISER9_ADDR {
            mem.iser9
        } else if address == ISER10_ADDR {
            mem.iser10
        } else if address == ISER11_ADDR {
            mem.iser11
        } else if address == ISER12_ADDR {
            mem.iser12
        } else if address == ISER13_ADDR {
            mem.iser13
        } else if address == ISER14_ADDR {
            mem.iser14
        } else if address == ISER15_ADDR {
            mem.iser15
        } else if address == ICER0_ADDR {
            mem.icer0
        } else if address == ICER1_ADDR {
            mem.icer1
        } else if address == ICER2_ADDR {
            mem.icer2
        } else if address == ICER3_ADDR {
            mem.icer3
        } else if address == ICER4_ADDR {
            mem.icer4
        } else if address == ICER5_ADDR {
            mem.icer5
        } else if address == ICER6_ADDR {
            mem.icer6
        } else if address == ICER7_ADDR {
            mem.icer7
        } else if address == ICER8_ADDR {
            mem.icer8
        } else if address == ICER9_ADDR {
            mem.icer9
        } else if address == ICER10_ADDR {
            mem.icer10
        } else if address == ICER11_ADDR {
            mem.icer11
        } else if address == ICER12_ADDR {
            mem.icer12
        } else if address == ICER13_ADDR {
            mem.icer13
        } else if address == ICER14_ADDR {
            mem.icer14
        } else if address == ICER15_ADDR {
            mem.icer15
        } else if address == ISPR0_ADDR {
            mem.ispr0
        } else if address == ISPR1_ADDR {
            mem.ispr1
        } else if address == ISPR2_ADDR {
            mem.ispr2
        } else if address == ISPR3_ADDR {
            mem.ispr3
        } else if address == ISPR4_ADDR {
            mem.ispr4
        } else if address == ISPR5_ADDR {
            mem.ispr5
        } else if address == ISPR6_ADDR {
            mem.ispr6
        } else if address == ISPR7_ADDR {
            mem.ispr7
        } else if address == ISPR8_ADDR {
            mem.ispr8
        } else if address == ISPR9_ADDR {
            mem.ispr9
        } else if address == ISPR10_ADDR {
            mem.ispr10
        } else if address == ISPR11_ADDR {
            mem.ispr11
        } else if address == ISPR12_ADDR {
            mem.ispr12
        } else if address == ISPR13_ADDR {
            mem.ispr13
        } else if address == ISPR14_ADDR {
            mem.ispr14
        } else if address == ISPR15_ADDR {
            mem.ispr15
        } else if address == ICPR0_ADDR {
            mem.icpr0
        } else if address == ICPR1_ADDR {
            mem.icpr1
        } else if address == ICPR2_ADDR {
            mem.icpr2
        } else if address == ICPR3_ADDR {
            mem.icpr3
        } else if address == ICPR4_ADDR {
            mem.icpr4
        } else if address == ICPR5_ADDR {
            mem.icpr5
        } else if address == ICPR6_ADDR {
            mem.icpr6
        } else if address == ICPR7_ADDR {
            mem.icpr7
        } else if address == ICPR8_ADDR {
            mem.icpr8
        } else if address == ICPR9_ADDR {
            mem.icpr9
        } else if address == ICPR10_ADDR {
            mem.icpr10
        } else if address == ICPR11_ADDR {
            mem.icpr11
        } else if address == ICPR12_ADDR {
            mem.icpr12
        } else if address == ICPR13_ADDR {
            mem.icpr13
        } else if address == ICPR14_ADDR {
            mem.icpr14
        } else if address == ICPR15_ADDR {
            mem.icpr15
        } else if address == IABR0_ADDR {
            mem.iabr0
        } else if address == IABR1_ADDR {
            mem.iabr1
        } else if address == IABR2_ADDR {
            mem.iabr2
        } else if address == IABR3_ADDR {
            mem.iabr3
        } else if address == IABR4_ADDR {
            mem.iabr4
        } else if address == IABR5_ADDR {
            mem.iabr5
        } else if address == IABR6_ADDR {
            mem.iabr6
        } else if address == IABR7_ADDR {
            mem.iabr7
        } else if address == IABR8_ADDR {
            mem.iabr8
        } else if address == IABR9_ADDR {
            mem.iabr9
        } else if address == IABR10_ADDR {
            mem.iabr10
        } else if address == IABR11_ADDR {
            mem.iabr11
        } else if address == IABR12_ADDR {
            mem.iabr12
        } else if address == IABR13_ADDR {
            mem.iabr13
        } else if address == IABR14_ADDR {
            mem.iabr14
        } else if address == IABR15_ADDR {
            mem.iabr15
        } else if address == IPR0_ADDR {
            mem.ipr0
        } else if address == IPR1_ADDR {
            mem.ipr1
        } else if address == IPR2_ADDR {
            mem.ipr2
        } else if address == IPR3_ADDR {
            mem.ipr3
        } else if address == IPR4_ADDR {
            mem.ipr4
        } else if address == IPR5_ADDR {
            mem.ipr5
        } else if address == IPR6_ADDR {
            mem.ipr6
        } else if address == IPR7_ADDR {
            mem.ipr7
        } else if address == IPR8_ADDR {
            mem.ipr8
        } else if address == IPR9_ADDR {
            mem.ipr9
        } else if address == IPR10_ADDR {
            mem.ipr10
        } else if address == IPR11_ADDR {
            mem.ipr11
        } else if address == IPR12_ADDR {
            mem.ipr12
        } else if address == IPR13_ADDR {
            mem.ipr13
        } else if address == IPR14_ADDR {
            mem.ipr14
        } else if address == IPR15_ADDR {
            mem.ipr15
        } else if address == IPR16_ADDR {
            mem.ipr16
        } else if address == IPR17_ADDR {
            mem.ipr17
        } else if address == IPR18_ADDR {
            mem.ipr18
        } else if address == IPR19_ADDR {
            mem.ipr19
        } else if address == IPR20_ADDR {
            mem.ipr20
        } else if address == IPR21_ADDR {
            mem.ipr21
        } else if address == IPR22_ADDR {
            mem.ipr22
        } else if address == IPR23_ADDR {
            mem.ipr23
        } else if address == IPR24_ADDR {
            mem.ipr24
        } else if address == IPR25_ADDR {
            mem.ipr25
        } else if address == IPR26_ADDR {
            mem.ipr26
        } else if address == IPR27_ADDR {
            mem.ipr27
        } else if address == IPR28_ADDR {
            mem.ipr28
        } else if address == IPR29_ADDR {
            mem.ipr29
        } else if address == IPR30_ADDR {
            mem.ipr30
        } else if address == IPR31_ADDR {
            mem.ipr31
        } else if address == IPR32_ADDR {
            mem.ipr32
        } else if address == IPR33_ADDR {
            mem.ipr33
        } else if address == IPR34_ADDR {
            mem.ipr34
        } else if address == IPR35_ADDR {
            mem.ipr35
        } else if address == IPR36_ADDR {
            mem.ipr36
        } else if address == IPR37_ADDR {
            mem.ipr37
        } else if address == IPR38_ADDR {
            mem.ipr38
        } else if address == IPR39_ADDR {
            mem.ipr39
        } else if address == IPR40_ADDR {
            mem.ipr40
        } else if address == IPR41_ADDR {
            mem.ipr41
        } else if address == IPR42_ADDR {
            mem.ipr42
        } else if address == IPR43_ADDR {
            mem.ipr43
        } else if address == IPR44_ADDR {
            mem.ipr44
        } else if address == IPR45_ADDR {
            mem.ipr45
        } else if address == IPR46_ADDR {
            mem.ipr46
        } else if address == IPR47_ADDR {
            mem.ipr47
        } else if address == IPR48_ADDR {
            mem.ipr48
        } else if address == IPR49_ADDR {
            mem.ipr49
        } else if address == IPR50_ADDR {
            mem.ipr50
        } else if address == IPR51_ADDR {
            mem.ipr51
        } else if address == IPR52_ADDR {
            mem.ipr52
        } else if address == IPR53_ADDR {
            mem.ipr53
        } else if address == IPR54_ADDR {
            mem.ipr54
        } else if address == IPR55_ADDR {
            mem.ipr55
        } else if address == IPR56_ADDR {
            mem.ipr56
        } else if address == IPR57_ADDR {
            mem.ipr57
        } else if address == IPR58_ADDR {
            mem.ipr58
        } else if address == IPR59_ADDR {
            mem.ipr59
        } else if address == IPR60_ADDR {
            mem.ipr60
        } else if address == IPR61_ADDR {
            mem.ipr61
        } else if address == IPR62_ADDR {
            mem.ipr62
        } else if address == IPR63_ADDR {
            mem.ipr63
        } else if address == IPR64_ADDR {
            mem.ipr64
        } else if address == IPR65_ADDR {
            mem.ipr65
        } else if address == IPR66_ADDR {
            mem.ipr66
        } else if address == IPR67_ADDR {
            mem.ipr67
        } else if address == IPR68_ADDR {
            mem.ipr68
        } else if address == IPR69_ADDR {
            mem.ipr69
        } else if address == IPR70_ADDR {
            mem.ipr70
        } else if address == IPR71_ADDR {
            mem.ipr71
        } else if address == IPR72_ADDR {
            mem.ipr72
        } else if address == IPR73_ADDR {
            mem.ipr73
        } else if address == IPR74_ADDR {
            mem.ipr74
        } else if address == IPR75_ADDR {
            mem.ipr75
        } else if address == IPR76_ADDR {
            mem.ipr76
        } else if address == IPR77_ADDR {
            mem.ipr77
        } else if address == IPR78_ADDR {
            mem.ipr78
        } else if address == IPR79_ADDR {
            mem.ipr79
        } else if address == IPR80_ADDR {
            mem.ipr80
        } else if address == IPR81_ADDR {
            mem.ipr81
        } else if address == IPR82_ADDR {
            mem.ipr82
        } else if address == IPR83_ADDR {
            mem.ipr83
        } else if address == IPR84_ADDR {
            mem.ipr84
        } else if address == IPR85_ADDR {
            mem.ipr85
        } else if address == IPR86_ADDR {
            mem.ipr86
        } else if address == IPR87_ADDR {
            mem.ipr87
        } else if address == IPR88_ADDR {
            mem.ipr88
        } else if address == IPR89_ADDR {
            mem.ipr89
        } else if address == IPR90_ADDR {
            mem.ipr90
        } else if address == IPR91_ADDR {
            mem.ipr91
        } else if address == IPR92_ADDR {
            mem.ipr92
        } else if address == IPR93_ADDR {
            mem.ipr93
        } else if address == IPR94_ADDR {
            mem.ipr94
        } else if address == IPR95_ADDR {
            mem.ipr95
        } else if address == IPR96_ADDR {
            mem.ipr96
        } else if address == IPR97_ADDR {
            mem.ipr97
        } else if address == IPR98_ADDR {
            mem.ipr98
        } else if address == IPR99_ADDR {
            mem.ipr99
        } else if address == IPR100_ADDR {
            mem.ipr100
        } else if address == IPR101_ADDR {
            mem.ipr101
        } else if address == IPR102_ADDR {
            mem.ipr102
        } else if address == IPR103_ADDR {
            mem.ipr103
        } else if address == IPR104_ADDR {
            mem.ipr104
        } else if address == IPR105_ADDR {
            mem.ipr105
        } else if address == IPR106_ADDR {
            mem.ipr106
        } else if address == IPR107_ADDR {
            mem.ipr107
        } else if address == IPR108_ADDR {
            mem.ipr108
        } else if address == IPR109_ADDR {
            mem.ipr109
        } else if address == IPR110_ADDR {
            mem.ipr110
        } else if address == IPR111_ADDR {
            mem.ipr111
        } else if address == IPR112_ADDR {
            mem.ipr112
        } else if address == IPR113_ADDR {
            mem.ipr113
        } else if address == IPR114_ADDR {
            mem.ipr114
        } else if address == IPR115_ADDR {
            mem.ipr115
        } else if address == IPR116_ADDR {
            mem.ipr116
        } else if address == IPR117_ADDR {
            mem.ipr117
        } else if address == IPR118_ADDR {
            mem.ipr118
        } else if address == IPR119_ADDR {
            mem.ipr119
        } else if address == IPR120_ADDR {
            mem.ipr120
        } else if address == IPR121_ADDR {
            mem.ipr121
        } else if address == IPR122_ADDR {
            mem.ipr122
        } else if address == IPR123_ADDR {
            mem.ipr123
        } else if address == MPU_TYPE_ADDR {
            mem.mpu_type
        } else if address == MPU_CTRL_ADDR {
            mem.mpu_ctrl
        } else if address == MPU_RNR_ADDR {
            mem.mpu_rnr
        } else if address == MPU_RBAR_ADDR {
            mem.mpu_rbar
        } else if address == MPU_RASR_ADDR {
            mem.mpu_rasr
        } else if address == MPU_RBAR_A1_ADDR {
            mem.mpu_rbar_a1
        } else if address == MPU_RASR_A1_ADDR {
            mem.mpu_rasr_a1
        } else if address == MPU_RBAR_A2_ADDR {
            mem.mpu_rbar_a2
        } else if address == MPU_RASR_A2_ADDR {
            mem.mpu_rasr_a2
        } else if address == MPU_RBAR_A3_ADDR {
            mem.mpu_rbar_a3
        } else {
            mem.mpu_rasr_a3
        }
    }
}
