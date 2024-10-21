// NVIC: https://developer.arm.com/documentation/ddi0403/d/System-Level-Architecture/System-Address-Map/Nested-Vectored-Interrupt-Controller--NVIC/NVIC-register-support-in-the-SCS?lang=en
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
    iser0: int,
    iser1: int,
    iser2: int,
    iser3: int,
    iser4: int,
    iser5: int,
    iser6: int,
    iser7: int,
    iser8: int,
    iser9: int,
    iser10: int,
    iser11: int,
    iser12: int,
    iser13: int,
    iser14: int,
    iser15: int,
    icer0: int,
    icer1: int,
    icer2: int,
    icer3: int,
    icer4: int,
    icer5: int,
    icer6: int,
    icer7: int,
    icer8: int,
    icer9: int,
    icer10: int,
    icer11: int,
    icer12: int,
    icer13: int,
    icer14: int,
    icer15: int,
    ispr0: int,
    ispr1: int,
    ispr2: int,
    ispr3: int,
    ispr4: int,
    ispr5: int,
    ispr6: int,
    ispr7: int,
    ispr8: int,
    ispr9: int,
    ispr10: int,
    ispr11: int,
    ispr12: int,
    ispr13: int,
    ispr14: int,
    ispr15: int,
    icpr0: int,
    icpr1: int,
    icpr2: int,
    icpr3: int,
    icpr4: int,
    icpr5: int,
    icpr6: int,
    icpr7: int,
    icpr8: int,
    icpr9: int,
    icpr10: int,
    icpr11: int,
    icpr12: int,
    icpr13: int,
    icpr14: int,
    icpr15: int,
    iabr0: int,
    iabr1: int,
    iabr2: int,
    iabr3: int,
    iabr4: int,
    iabr5: int,
    iabr6: int,
    iabr7: int,
    iabr8: int,
    iabr9: int,
    iabr10: int,
    iabr11: int,
    iabr12: int,
    iabr13: int,
    iabr14: int,
    iabr15: int,
    ipr0: int,
    ipr1: int,
    ipr2: int,
    ipr3: int,
    ipr4: int,
    ipr5: int,
    ipr6: int,
    ipr7: int,
    ipr8: int,
    ipr9: int,
    ipr10: int,
    ipr11: int,
    ipr12: int,
    ipr13: int,
    ipr14: int,
    ipr15: int,
    ipr16: int,
    ipr17: int,
    ipr18: int,
    ipr19: int,
    ipr20: int,
    ipr21: int,
    ipr22: int,
    ipr23: int,
    ipr24: int,
    ipr25: int,
    ipr26: int,
    ipr27: int,
    ipr28: int,
    ipr29: int,
    ipr30: int,
    ipr31: int,
    ipr32: int,
    ipr33: int,
    ipr34: int,
    ipr35: int,
    ipr36: int,
    ipr37: int,
    ipr38: int,
    ipr39: int,
    ipr40: int,
    ipr41: int,
    ipr42: int,
    ipr43: int,
    ipr44: int,
    ipr45: int,
    ipr46: int,
    ipr47: int,
    ipr48: int,
    ipr49: int,
    ipr50: int,
    ipr51: int,
    ipr52: int,
    ipr53: int,
    ipr54: int,
    ipr55: int,
    ipr56: int,
    ipr57: int,
    ipr58: int,
    ipr59: int,
    ipr60: int,
    ipr61: int,
    ipr62: int,
    ipr63: int,
    ipr64: int,
    ipr65: int,
    ipr66: int,
    ipr67: int,
    ipr68: int,
    ipr69: int,
    ipr70: int,
    ipr71: int,
    ipr72: int,
    ipr73: int,
    ipr74: int,
    ipr75: int,
    ipr76: int,
    ipr77: int,
    ipr78: int,
    ipr79: int,
    ipr80: int,
    ipr81: int,
    ipr82: int,
    ipr83: int,
    ipr84: int,
    ipr85: int,
    ipr86: int,
    ipr87: int,
    ipr88: int,
    ipr89: int,
    ipr90: int,
    ipr91: int,
    ipr92: int,
    ipr93: int,
    ipr94: int,
    ipr95: int,
    ipr96: int,
    ipr97: int,
    ipr98: int,
    ipr99: int,
    ipr100: int,
    ipr101: int,
    ipr102: int,
    ipr103: int,
    ipr104: int,
    ipr105: int,
    ipr106: int,
    ipr107: int,
    ipr108: int,
    ipr109: int,
    ipr110: int,
    ipr111: int,
    ipr112: int,
    ipr113: int,
    ipr114: int,
    ipr115: int,
    ipr116: int,
    ipr117: int,
    ipr118: int,
    ipr119: int,
    ipr120: int,
    ipr121: int,
    ipr122: int,
    ipr123: int,
)]
pub struct Nvic {
    // 0xE000E100 -0xE000E13C	NVIC_ISER0 -NVIC_ISER15	RW	0x00000000
    // Interrupt Set-Enable Registers, NVIC_ISER0 - NVIC_ISER15
    #[field(u32[iser0])]
    iser0: u32,
    #[field(u32[iser1])]
    iser1: u32,
    #[field(u32[iser2])]
    iser2: u32,
    #[field(u32[iser3])]
    iser3: u32,
    #[field(u32[iser4])]
    iser4: u32,
    #[field(u32[iser5])]
    iser5: u32,
    #[field(u32[iser6])]
    iser6: u32,
    #[field(u32[iser7])]
    iser7: u32,
    #[field(u32[iser8])]
    iser8: u32,
    #[field(u32[iser9])]
    iser9: u32,
    #[field(u32[iser10])]
    iser10: u32,
    #[field(u32[iser11])]
    iser11: u32,
    #[field(u32[iser12])]
    iser12: u32,
    #[field(u32[iser13])]
    iser13: u32,
    #[field(u32[iser14])]
    iser14: u32,
    #[field(u32[iser15])]
    iser15: u32,
    // 0xE000E180 -0xE000E1BC	NVIC_ICER0 -NVIC_ICER15	RW	0x00000000
    // Interrupt Clear-Enable Registers, NVIC_ICER0 - NVIC_ICER15
    #[field(u32[icer0])]
    icer0: u32,
    #[field(u32[icer1])]
    icer1: u32,
    #[field(u32[icer2])]
    icer2: u32,
    #[field(u32[icer3])]
    icer3: u32,
    #[field(u32[icer4])]
    icer4: u32,
    #[field(u32[icer5])]
    icer5: u32,
    #[field(u32[icer6])]
    icer6: u32,
    #[field(u32[icer7])]
    icer7: u32,
    #[field(u32[icer8])]
    icer8: u32,
    #[field(u32[icer9])]
    icer9: u32,
    #[field(u32[icer10])]
    icer10: u32,
    #[field(u32[icer11])]
    icer11: u32,
    #[field(u32[icer12])]
    icer12: u32,
    #[field(u32[icer13])]
    icer13: u32,
    #[field(u32[icer14])]
    icer14: u32,
    #[field(u32[icer15])]
    icer15: u32,
    // 0xE000E200 -0xE000E23C	NVIC_ISPR0 -NVIC_ISPR15	RW	0x00000000
    // Interrupt Set-Pending Registers, NVIC_ISPR0 - NVIC_ISPR15
    #[field(u32[ispr0])]
    ispr0: u32,
    #[field(u32[ispr1])]
    ispr1: u32,
    #[field(u32[ispr2])]
    ispr2: u32,
    #[field(u32[ispr3])]
    ispr3: u32,
    #[field(u32[ispr4])]
    ispr4: u32,
    #[field(u32[ispr5])]
    ispr5: u32,
    #[field(u32[ispr6])]
    ispr6: u32,
    #[field(u32[ispr7])]
    ispr7: u32,
    #[field(u32[ispr8])]
    ispr8: u32,
    #[field(u32[ispr9])]
    ispr9: u32,
    #[field(u32[ispr10])]
    ispr10: u32,
    #[field(u32[ispr11])]
    ispr11: u32,
    #[field(u32[ispr12])]
    ispr12: u32,
    #[field(u32[ispr13])]
    ispr13: u32,
    #[field(u32[ispr14])]
    ispr14: u32,
    #[field(u32[ispr15])]
    ispr15: u32,
    // 0xE000E280 -0xE000E2BC	NVIC_ICPR0 -NVIC_ICPR15	RW	0x00000000
    // Interrupt Clear-Pending Registers, NVIC_ICPR0 - NVIC_ICPR15
    #[field(u32[icpr0])]
    icpr0: u32,
    #[field(u32[icpr1])]
    icpr1: u32,
    #[field(u32[icpr2])]
    icpr2: u32,
    #[field(u32[icpr3])]
    icpr3: u32,
    #[field(u32[icpr4])]
    icpr4: u32,
    #[field(u32[icpr5])]
    icpr5: u32,
    #[field(u32[icpr6])]
    icpr6: u32,
    #[field(u32[icpr7])]
    icpr7: u32,
    #[field(u32[icpr8])]
    icpr8: u32,
    #[field(u32[icpr9])]
    icpr9: u32,
    #[field(u32[icpr10])]
    icpr10: u32,
    #[field(u32[icpr11])]
    icpr11: u32,
    #[field(u32[icpr12])]
    icpr12: u32,
    #[field(u32[icpr13])]
    icpr13: u32,
    #[field(u32[icpr14])]
    icpr14: u32,
    #[field(u32[icpr15])]
    icpr15: u32,
    // 0xE000E300 -0xE000E37C	NVIC_IABR0 -NVIC_IABR15	RO	0x00000000
    // Interrupt Active Bit Registers, NVIC_IABR0 - NVIC_IABR15
    #[field(u32[iabr0])]
    iabr0: u32,
    #[field(u32[iabr1])]
    iabr1: u32,
    #[field(u32[iabr2])]
    iabr2: u32,
    #[field(u32[iabr3])]
    iabr3: u32,
    #[field(u32[iabr4])]
    iabr4: u32,
    #[field(u32[iabr5])]
    iabr5: u32,
    #[field(u32[iabr6])]
    iabr6: u32,
    #[field(u32[iabr7])]
    iabr7: u32,
    #[field(u32[iabr8])]
    iabr8: u32,
    #[field(u32[iabr9])]
    iabr9: u32,
    #[field(u32[iabr10])]
    iabr10: u32,
    #[field(u32[iabr11])]
    iabr11: u32,
    #[field(u32[iabr12])]
    iabr12: u32,
    #[field(u32[iabr13])]
    iabr13: u32,
    #[field(u32[iabr14])]
    iabr14: u32,
    #[field(u32[iabr15])]
    iabr15: u32,
    // 0xE000E400 -0xE000E7EC	NVIC_IPR0 -NVIC_IPR123	RW	0x00000000
    // Interrupt Priority Registers, NVIC_IPR0 - NVC_IPR123
    #[field(u32[ipr0])]
    ipr0: u32,
    #[field(u32[ipr1])]
    ipr1: u32,
    #[field(u32[ipr2])]
    ipr2: u32,
    #[field(u32[ipr3])]
    ipr3: u32,
    #[field(u32[ipr4])]
    ipr4: u32,
    #[field(u32[ipr5])]
    ipr5: u32,
    #[field(u32[ipr6])]
    ipr6: u32,
    #[field(u32[ipr7])]
    ipr7: u32,
    #[field(u32[ipr8])]
    ipr8: u32,
    #[field(u32[ipr9])]
    ipr9: u32,
    #[field(u32[ipr10])]
    ipr10: u32,
    #[field(u32[ipr11])]
    ipr11: u32,
    #[field(u32[ipr12])]
    ipr12: u32,
    #[field(u32[ipr13])]
    ipr13: u32,
    #[field(u32[ipr14])]
    ipr14: u32,
    #[field(u32[ipr15])]
    ipr15: u32,
    #[field(u32[ipr16])]
    ipr16: u32,
    #[field(u32[ipr17])]
    ipr17: u32,
    #[field(u32[ipr18])]
    ipr18: u32,
    #[field(u32[ipr19])]
    ipr19: u32,
    #[field(u32[ipr20])]
    ipr20: u32,
    #[field(u32[ipr21])]
    ipr21: u32,
    #[field(u32[ipr22])]
    ipr22: u32,
    #[field(u32[ipr23])]
    ipr23: u32,
    #[field(u32[ipr24])]
    ipr24: u32,
    #[field(u32[ipr25])]
    ipr25: u32,
    #[field(u32[ipr26])]
    ipr26: u32,
    #[field(u32[ipr27])]
    ipr27: u32,
    #[field(u32[ipr28])]
    ipr28: u32,
    #[field(u32[ipr29])]
    ipr29: u32,
    #[field(u32[ipr30])]
    ipr30: u32,
    #[field(u32[ipr31])]
    ipr31: u32,
    #[field(u32[ipr32])]
    ipr32: u32,
    #[field(u32[ipr33])]
    ipr33: u32,
    #[field(u32[ipr34])]
    ipr34: u32,
    #[field(u32[ipr35])]
    ipr35: u32,
    #[field(u32[ipr36])]
    ipr36: u32,
    #[field(u32[ipr37])]
    ipr37: u32,
    #[field(u32[ipr38])]
    ipr38: u32,
    #[field(u32[ipr39])]
    ipr39: u32,
    #[field(u32[ipr40])]
    ipr40: u32,
    #[field(u32[ipr41])]
    ipr41: u32,
    #[field(u32[ipr42])]
    ipr42: u32,
    #[field(u32[ipr43])]
    ipr43: u32,
    #[field(u32[ipr44])]
    ipr44: u32,
    #[field(u32[ipr45])]
    ipr45: u32,
    #[field(u32[ipr46])]
    ipr46: u32,
    #[field(u32[ipr47])]
    ipr47: u32,
    #[field(u32[ipr48])]
    ipr48: u32,
    #[field(u32[ipr49])]
    ipr49: u32,
    #[field(u32[ipr50])]
    ipr50: u32,
    #[field(u32[ipr51])]
    ipr51: u32,
    #[field(u32[ipr52])]
    ipr52: u32,
    #[field(u32[ipr53])]
    ipr53: u32,
    #[field(u32[ipr54])]
    ipr54: u32,
    #[field(u32[ipr55])]
    ipr55: u32,
    #[field(u32[ipr56])]
    ipr56: u32,
    #[field(u32[ipr57])]
    ipr57: u32,
    #[field(u32[ipr58])]
    ipr58: u32,
    #[field(u32[ipr59])]
    ipr59: u32,
    #[field(u32[ipr60])]
    ipr60: u32,
    #[field(u32[ipr61])]
    ipr61: u32,
    #[field(u32[ipr62])]
    ipr62: u32,
    #[field(u32[ipr63])]
    ipr63: u32,
    #[field(u32[ipr64])]
    ipr64: u32,
    #[field(u32[ipr65])]
    ipr65: u32,
    #[field(u32[ipr66])]
    ipr66: u32,
    #[field(u32[ipr67])]
    ipr67: u32,
    #[field(u32[ipr68])]
    ipr68: u32,
    #[field(u32[ipr69])]
    ipr69: u32,
    #[field(u32[ipr70])]
    ipr70: u32,
    #[field(u32[ipr71])]
    ipr71: u32,
    #[field(u32[ipr72])]
    ipr72: u32,
    #[field(u32[ipr73])]
    ipr73: u32,
    #[field(u32[ipr74])]
    ipr74: u32,
    #[field(u32[ipr75])]
    ipr75: u32,
    #[field(u32[ipr76])]
    ipr76: u32,
    #[field(u32[ipr77])]
    ipr77: u32,
    #[field(u32[ipr78])]
    ipr78: u32,
    #[field(u32[ipr79])]
    ipr79: u32,
    #[field(u32[ipr80])]
    ipr80: u32,
    #[field(u32[ipr81])]
    ipr81: u32,
    #[field(u32[ipr82])]
    ipr82: u32,
    #[field(u32[ipr83])]
    ipr83: u32,
    #[field(u32[ipr84])]
    ipr84: u32,
    #[field(u32[ipr85])]
    ipr85: u32,
    #[field(u32[ipr86])]
    ipr86: u32,
    #[field(u32[ipr87])]
    ipr87: u32,
    #[field(u32[ipr88])]
    ipr88: u32,
    #[field(u32[ipr89])]
    ipr89: u32,
    #[field(u32[ipr90])]
    ipr90: u32,
    #[field(u32[ipr91])]
    ipr91: u32,
    #[field(u32[ipr92])]
    ipr92: u32,
    #[field(u32[ipr93])]
    ipr93: u32,
    #[field(u32[ipr94])]
    ipr94: u32,
    #[field(u32[ipr95])]
    ipr95: u32,
    #[field(u32[ipr96])]
    ipr96: u32,
    #[field(u32[ipr97])]
    ipr97: u32,
    #[field(u32[ipr98])]
    ipr98: u32,
    #[field(u32[ipr99])]
    ipr99: u32,
    #[field(u32[ipr100])]
    ipr100: u32,
    #[field(u32[ipr101])]
    ipr101: u32,
    #[field(u32[ipr102])]
    ipr102: u32,
    #[field(u32[ipr103])]
    ipr103: u32,
    #[field(u32[ipr104])]
    ipr104: u32,
    #[field(u32[ipr105])]
    ipr105: u32,
    #[field(u32[ipr106])]
    ipr106: u32,
    #[field(u32[ipr107])]
    ipr107: u32,
    #[field(u32[ipr108])]
    ipr108: u32,
    #[field(u32[ipr109])]
    ipr109: u32,
    #[field(u32[ipr110])]
    ipr110: u32,
    #[field(u32[ipr111])]
    ipr111: u32,
    #[field(u32[ipr112])]
    ipr112: u32,
    #[field(u32[ipr113])]
    ipr113: u32,
    #[field(u32[ipr114])]
    ipr114: u32,
    #[field(u32[ipr115])]
    ipr115: u32,
    #[field(u32[ipr116])]
    ipr116: u32,
    #[field(u32[ipr117])]
    ipr117: u32,
    #[field(u32[ipr118])]
    ipr118: u32,
    #[field(u32[ipr119])]
    ipr119: u32,
    #[field(u32[ipr120])]
    ipr120: u32,
    #[field(u32[ipr121])]
    ipr121: u32,
    #[field(u32[ipr122])]
    ipr122: u32,
    #[field(u32[ipr123])]
    ipr123: u32,
}
