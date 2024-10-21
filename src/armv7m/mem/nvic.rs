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

impl Nvic {
    fn addr_into_reg_value(&self, address: u32) -> u32 {
        match address {
            // NVIC_ISER0 - NVIC_ISER15
            0xE000E100 => self.iser0,
            0xE000E104 => self.iser1,
            0xE000E108 => self.iser2,
            0xE000E10C => self.iser3,
            0xE000E110 => self.iser4,
            0xE000E114 => self.iser5,
            0xE000E118 => self.iser6,
            0xE000E11C => self.iser7,
            0xE000E120 => self.iser8,
            0xE000E124 => self.iser9,
            0xE000E128 => self.iser10,
            0xE000E12C => self.iser11,
            0xE000E130 => self.iser12,
            0xE000E134 => self.iser13,
            0xE000E138 => self.iser14,
            0xE000E13C => self.iser15,

            // NVIC_ICER0 - NVIC_ICER15
            0xE000E180 => self.icer0,
            0xE000E184 => self.icer1,
            0xE000E188 => self.icer2,
            0xE000E18C => self.icer3,
            0xE000E190 => self.icer4,
            0xE000E194 => self.icer5,
            0xE000E198 => self.icer6,
            0xE000E19C => self.icer7,
            0xE000E1A0 => self.icer8,
            0xE000E1A4 => self.icer9,
            0xE000E1A8 => self.icer10,
            0xE000E1AC => self.icer11,
            0xE000E1B0 => self.icer12,
            0xE000E1B4 => self.icer13,
            0xE000E1B8 => self.icer14,
            0xE000E1BC => self.icer15,

            // NVIC_ISPR0 - NVIC_ISPR15
            0xE000E200 => self.ispr0,
            0xE000E204 => self.ispr1,
            0xE000E208 => self.ispr2,
            0xE000E20C => self.ispr3,
            0xE000E210 => self.ispr4,
            0xE000E214 => self.ispr5,
            0xE000E218 => self.ispr6,
            0xE000E21C => self.ispr7,
            0xE000E220 => self.ispr8,
            0xE000E224 => self.ispr9,
            0xE000E228 => self.ispr10,
            0xE000E22C => self.ispr11,
            0xE000E230 => self.ispr12,
            0xE000E234 => self.ispr13,
            0xE000E238 => self.ispr14,
            0xE000E23C => self.ispr15,

            // NVIC_ICPR0 - NVIC_ICPR15
            0xE000E280 => self.icpr0,
            0xE000E284 => self.icpr1,
            0xE000E288 => self.icpr2,
            0xE000E28C => self.icpr3,
            0xE000E290 => self.icpr4,
            0xE000E294 => self.icpr5,
            0xE000E298 => self.icpr6,
            0xE000E29C => self.icpr7,
            0xE000E2A0 => self.icpr8,
            0xE000E2A4 => self.icpr9,
            0xE000E2A8 => self.icpr10,
            0xE000E2AC => self.icpr11,
            0xE000E2B0 => self.icpr12,
            0xE000E2B4 => self.icpr13,
            0xE000E2B8 => self.icpr14,
            0xE000E2BC => self.icpr15,

            // NVIC_IABR0 - NVIC_IABR15
            0xE000E300 => self.iabr0,
            0xE000E304 => self.iabr1,
            0xE000E308 => self.iabr2,
            0xE000E30C => self.iabr3,
            0xE000E310 => self.iabr4,
            0xE000E314 => self.iabr5,
            0xE000E318 => self.iabr6,
            0xE000E31C => self.iabr7,
            0xE000E320 => self.iabr8,
            0xE000E324 => self.iabr9,
            0xE000E328 => self.iabr10,
            0xE000E32C => self.iabr11,
            0xE000E330 => self.iabr12,
            0xE000E334 => self.iabr13,
            0xE000E338 => self.iabr14,
            0xE000E33C => self.iabr15,

            // NVIC IPR0 - NVIC_IPR123
            0xE000E400 => self.ipr0,
            0xE000E404 => self.ipr1,
            0xE000E408 => self.ipr2,
            0xE000E40C => self.ipr3,
            0xE000E410 => self.ipr4,
            0xE000E414 => self.ipr5,
            0xE000E418 => self.ipr6,
            0xE000E41C => self.ipr7,
            0xE000E420 => self.ipr8,
            0xE000E424 => self.ipr9,
            0xE000E428 => self.ipr10,
            0xE000E42C => self.ipr11,
            0xE000E430 => self.ipr12,
            0xE000E434 => self.ipr13,
            0xE000E438 => self.ipr14,
            0xE000E43C => self.ipr15,
            0xE000E440 => self.ipr16,
            0xE000E444 => self.ipr17,
            0xE000E448 => self.ipr18,
            0xE000E44C => self.ipr19,
            0xE000E450 => self.ipr20,
            0xE000E454 => self.ipr21,
            0xE000E458 => self.ipr22,
            0xE000E45C => self.ipr23,
            0xE000E460 => self.ipr24,
            0xE000E464 => self.ipr25,
            0xE000E468 => self.ipr26,
            0xE000E46C => self.ipr27,
            0xE000E470 => self.ipr28,
            0xE000E474 => self.ipr29,
            0xE000E478 => self.ipr30,
            0xE000E47C => self.ipr31,
            0xE000E480 => self.ipr32,
            0xE000E484 => self.ipr33,
            0xE000E488 => self.ipr34,
            0xE000E48C => self.ipr35,
            0xE000E490 => self.ipr36,
            0xE000E494 => self.ipr37,
            0xE000E498 => self.ipr38,
            0xE000E49C => self.ipr39,
            0xE000E4A0 => self.ipr40,
            0xE000E4A4 => self.ipr41,
            0xE000E4A8 => self.ipr42,
            0xE000E4AC => self.ipr43,
            0xE000E4B0 => self.ipr44,
            0xE000E4B4 => self.ipr45,
            0xE000E4B8 => self.ipr46,
            0xE000E4BC => self.ipr47,
            0xE000E4C0 => self.ipr48,
            0xE000E4C4 => self.ipr49,
            0xE000E4C8 => self.ipr50,
            0xE000E4CC => self.ipr51,
            0xE000E4D0 => self.ipr52,
            0xE000E4D4 => self.ipr53,
            0xE000E4D8 => self.ipr54,
            0xE000E4DC => self.ipr55,
            0xE000E4E0 => self.ipr56,
            0xE000E4E4 => self.ipr57,
            0xE000E4E8 => self.ipr58,
            0xE000E4EC => self.ipr59,
            0xE000E4F0 => self.ipr60,
            0xE000E4F4 => self.ipr61,
            0xE000E4F8 => self.ipr62,
            0xE000E4FC => self.ipr63,
            0xE000E500 => self.ipr64,
            0xE000E504 => self.ipr65,
            0xE000E508 => self.ipr66,
            0xE000E50C => self.ipr67,
            0xE000E510 => self.ipr68,
            0xE000E514 => self.ipr69,
            0xE000E518 => self.ipr70,
            0xE000E51C => self.ipr71,
            0xE000E520 => self.ipr72,
            0xE000E524 => self.ipr73,
            0xE000E528 => self.ipr74,
            0xE000E52C => self.ipr75,
            0xE000E530 => self.ipr76,
            0xE000E534 => self.ipr77,
            0xE000E538 => self.ipr78,
            0xE000E53C => self.ipr79,
            0xE000E540 => self.ipr80,
            0xE000E544 => self.ipr81,
            0xE000E548 => self.ipr82,
            0xE000E54C => self.ipr83,
            0xE000E550 => self.ipr84,
            0xE000E554 => self.ipr85,
            0xE000E558 => self.ipr86,
            0xE000E55C => self.ipr87,
            0xE000E560 => self.ipr88,
            0xE000E564 => self.ipr89,
            0xE000E568 => self.ipr90,
            0xE000E56C => self.ipr91,
            0xE000E570 => self.ipr92,
            0xE000E574 => self.ipr93,
            0xE000E578 => self.ipr94,
            0xE000E57C => self.ipr95,
            0xE000E580 => self.ipr96,
            0xE000E584 => self.ipr97,
            0xE000E588 => self.ipr98,
            0xE000E58C => self.ipr99,
            0xE000E590 => self.ipr100,
            0xE000E594 => self.ipr101,
            0xE000E598 => self.ipr102,
            0xE000E59C => self.ipr103,
            0xE000E5A0 => self.ipr104,
            0xE000E5A4 => self.ipr105,
            0xE000E5A8 => self.ipr106,
            0xE000E5AC => self.ipr107,
            0xE000E5B0 => self.ipr108,
            0xE000E5B4 => self.ipr109,
            0xE000E5B8 => self.ipr110,
            0xE000E5BC => self.ipr111,
            0xE000E5C0 => self.ipr112,
            0xE000E5C4 => self.ipr113,
            0xE000E5C8 => self.ipr114,
            0xE000E5CC => self.ipr115,
            0xE000E5D0 => self.ipr116,
            0xE000E5D4 => self.ipr117,
            0xE000E5D8 => self.ipr118,
            0xE000E5DC => self.ipr119,
            0xE000E5E0 => self.ipr120,
            0xE000E5E4 => self.ipr121,
            0xE000E5E8 => self.ipr122,
            0xE000E5EC => self.ipr123,

            0xE000E380..=0xE000E3FC => panic!("Read of reserved addr"),
            0xE000E7F0..=0xE000ECFC => panic!("Read of reserved addr"),
            _ => panic!("Read of invalid addr"),
        }
    }

    fn addr_into_reg_mut(&mut self, address: u32) -> &mut u32 {
        match address {
            // NVIC_ISER0 - NVIC_ISER15
            0xE000E100 => &mut self.iser0,
            0xE000E104 => &mut self.iser1,
            0xE000E108 => &mut self.iser2,
            0xE000E10C => &mut self.iser3,
            0xE000E110 => &mut self.iser4,
            0xE000E114 => &mut self.iser5,
            0xE000E118 => &mut self.iser6,
            0xE000E11C => &mut self.iser7,
            0xE000E120 => &mut self.iser8,
            0xE000E124 => &mut self.iser9,
            0xE000E128 => &mut self.iser10,
            0xE000E12C => &mut self.iser11,
            0xE000E130 => &mut self.iser12,
            0xE000E134 => &mut self.iser13,
            0xE000E138 => &mut self.iser14,
            0xE000E13C => &mut self.iser15,

            // NVIC_ICER0 - NVIC_ICER15
            0xE000E180 => &mut self.icer0,
            0xE000E184 => &mut self.icer1,
            0xE000E188 => &mut self.icer2,
            0xE000E18C => &mut self.icer3,
            0xE000E190 => &mut self.icer4,
            0xE000E194 => &mut self.icer5,
            0xE000E198 => &mut self.icer6,
            0xE000E19C => &mut self.icer7,
            0xE000E1A0 => &mut self.icer8,
            0xE000E1A4 => &mut self.icer9,
            0xE000E1A8 => &mut self.icer10,
            0xE000E1AC => &mut self.icer11,
            0xE000E1B0 => &mut self.icer12,
            0xE000E1B4 => &mut self.icer13,
            0xE000E1B8 => &mut self.icer14,
            0xE000E1BC => &mut self.icer15,

            // NVIC_ISPR0 - NVIC_ISPR15
            0xE000E200 => &mut self.ispr0,
            0xE000E204 => &mut self.ispr1,
            0xE000E208 => &mut self.ispr2,
            0xE000E20C => &mut self.ispr3,
            0xE000E210 => &mut self.ispr4,
            0xE000E214 => &mut self.ispr5,
            0xE000E218 => &mut self.ispr6,
            0xE000E21C => &mut self.ispr7,
            0xE000E220 => &mut self.ispr8,
            0xE000E224 => &mut self.ispr9,
            0xE000E228 => &mut self.ispr10,
            0xE000E22C => &mut self.ispr11,
            0xE000E230 => &mut self.ispr12,
            0xE000E234 => &mut self.ispr13,
            0xE000E238 => &mut self.ispr14,
            0xE000E23C => &mut self.ispr15,

            // NVIC_ICPR0 - NVIC_ICPR15
            0xE000E280 => &mut self.icpr0,
            0xE000E284 => &mut self.icpr1,
            0xE000E288 => &mut self.icpr2,
            0xE000E28C => &mut self.icpr3,
            0xE000E290 => &mut self.icpr4,
            0xE000E294 => &mut self.icpr5,
            0xE000E298 => &mut self.icpr6,
            0xE000E29C => &mut self.icpr7,
            0xE000E2A0 => &mut self.icpr8,
            0xE000E2A4 => &mut self.icpr9,
            0xE000E2A8 => &mut self.icpr10,
            0xE000E2AC => &mut self.icpr11,
            0xE000E2B0 => &mut self.icpr12,
            0xE000E2B4 => &mut self.icpr13,
            0xE000E2B8 => &mut self.icpr14,
            0xE000E2BC => &mut self.icpr15,

            // NVIC_IABR0 - NVIC_IABR15
            0xE000E300 => &mut self.iabr0,
            0xE000E304 => &mut self.iabr1,
            0xE000E308 => &mut self.iabr2,
            0xE000E30C => &mut self.iabr3,
            0xE000E310 => &mut self.iabr4,
            0xE000E314 => &mut self.iabr5,
            0xE000E318 => &mut self.iabr6,
            0xE000E31C => &mut self.iabr7,
            0xE000E320 => &mut self.iabr8,
            0xE000E324 => &mut self.iabr9,
            0xE000E328 => &mut self.iabr10,
            0xE000E32C => &mut self.iabr11,
            0xE000E330 => &mut self.iabr12,
            0xE000E334 => &mut self.iabr13,
            0xE000E338 => &mut self.iabr14,
            0xE000E33C => &mut self.iabr15,

            // NVIC IPR0 - NVIC_IPR123
            0xE000E400 => &mut self.ipr0,
            0xE000E404 => &mut self.ipr1,
            0xE000E408 => &mut self.ipr2,
            0xE000E40C => &mut self.ipr3,
            0xE000E410 => &mut self.ipr4,
            0xE000E414 => &mut self.ipr5,
            0xE000E418 => &mut self.ipr6,
            0xE000E41C => &mut self.ipr7,
            0xE000E420 => &mut self.ipr8,
            0xE000E424 => &mut self.ipr9,
            0xE000E428 => &mut self.ipr10,
            0xE000E42C => &mut self.ipr11,
            0xE000E430 => &mut self.ipr12,
            0xE000E434 => &mut self.ipr13,
            0xE000E438 => &mut self.ipr14,
            0xE000E43C => &mut self.ipr15,
            0xE000E440 => &mut self.ipr16,
            0xE000E444 => &mut self.ipr17,
            0xE000E448 => &mut self.ipr18,
            0xE000E44C => &mut self.ipr19,
            0xE000E450 => &mut self.ipr20,
            0xE000E454 => &mut self.ipr21,
            0xE000E458 => &mut self.ipr22,
            0xE000E45C => &mut self.ipr23,
            0xE000E460 => &mut self.ipr24,
            0xE000E464 => &mut self.ipr25,
            0xE000E468 => &mut self.ipr26,
            0xE000E46C => &mut self.ipr27,
            0xE000E470 => &mut self.ipr28,
            0xE000E474 => &mut self.ipr29,
            0xE000E478 => &mut self.ipr30,
            0xE000E47C => &mut self.ipr31,
            0xE000E480 => &mut self.ipr32,
            0xE000E484 => &mut self.ipr33,
            0xE000E488 => &mut self.ipr34,
            0xE000E48C => &mut self.ipr35,
            0xE000E490 => &mut self.ipr36,
            0xE000E494 => &mut self.ipr37,
            0xE000E498 => &mut self.ipr38,
            0xE000E49C => &mut self.ipr39,
            0xE000E4A0 => &mut self.ipr40,
            0xE000E4A4 => &mut self.ipr41,
            0xE000E4A8 => &mut self.ipr42,
            0xE000E4AC => &mut self.ipr43,
            0xE000E4B0 => &mut self.ipr44,
            0xE000E4B4 => &mut self.ipr45,
            0xE000E4B8 => &mut self.ipr46,
            0xE000E4BC => &mut self.ipr47,
            0xE000E4C0 => &mut self.ipr48,
            0xE000E4C4 => &mut self.ipr49,
            0xE000E4C8 => &mut self.ipr50,
            0xE000E4CC => &mut self.ipr51,
            0xE000E4D0 => &mut self.ipr52,
            0xE000E4D4 => &mut self.ipr53,
            0xE000E4D8 => &mut self.ipr54,
            0xE000E4DC => &mut self.ipr55,
            0xE000E4E0 => &mut self.ipr56,
            0xE000E4E4 => &mut self.ipr57,
            0xE000E4E8 => &mut self.ipr58,
            0xE000E4EC => &mut self.ipr59,
            0xE000E4F0 => &mut self.ipr60,
            0xE000E4F4 => &mut self.ipr61,
            0xE000E4F8 => &mut self.ipr62,
            0xE000E4FC => &mut self.ipr63,
            0xE000E500 => &mut self.ipr64,
            0xE000E504 => &mut self.ipr65,
            0xE000E508 => &mut self.ipr66,
            0xE000E50C => &mut self.ipr67,
            0xE000E510 => &mut self.ipr68,
            0xE000E514 => &mut self.ipr69,
            0xE000E518 => &mut self.ipr70,
            0xE000E51C => &mut self.ipr71,
            0xE000E520 => &mut self.ipr72,
            0xE000E524 => &mut self.ipr73,
            0xE000E528 => &mut self.ipr74,
            0xE000E52C => &mut self.ipr75,
            0xE000E530 => &mut self.ipr76,
            0xE000E534 => &mut self.ipr77,
            0xE000E538 => &mut self.ipr78,
            0xE000E53C => &mut self.ipr79,
            0xE000E540 => &mut self.ipr80,
            0xE000E544 => &mut self.ipr81,
            0xE000E548 => &mut self.ipr82,
            0xE000E54C => &mut self.ipr83,
            0xE000E550 => &mut self.ipr84,
            0xE000E554 => &mut self.ipr85,
            0xE000E558 => &mut self.ipr86,
            0xE000E55C => &mut self.ipr87,
            0xE000E560 => &mut self.ipr88,
            0xE000E564 => &mut self.ipr89,
            0xE000E568 => &mut self.ipr90,
            0xE000E56C => &mut self.ipr91,
            0xE000E570 => &mut self.ipr92,
            0xE000E574 => &mut self.ipr93,
            0xE000E578 => &mut self.ipr94,
            0xE000E57C => &mut self.ipr95,
            0xE000E580 => &mut self.ipr96,
            0xE000E584 => &mut self.ipr97,
            0xE000E588 => &mut self.ipr98,
            0xE000E58C => &mut self.ipr99,
            0xE000E590 => &mut self.ipr100,
            0xE000E594 => &mut self.ipr101,
            0xE000E598 => &mut self.ipr102,
            0xE000E59C => &mut self.ipr103,
            0xE000E5A0 => &mut self.ipr104,
            0xE000E5A4 => &mut self.ipr105,
            0xE000E5A8 => &mut self.ipr106,
            0xE000E5AC => &mut self.ipr107,
            0xE000E5B0 => &mut self.ipr108,
            0xE000E5B4 => &mut self.ipr109,
            0xE000E5B8 => &mut self.ipr110,
            0xE000E5BC => &mut self.ipr111,
            0xE000E5C0 => &mut self.ipr112,
            0xE000E5C4 => &mut self.ipr113,
            0xE000E5C8 => &mut self.ipr114,
            0xE000E5CC => &mut self.ipr115,
            0xE000E5D0 => &mut self.ipr116,
            0xE000E5D4 => &mut self.ipr117,
            0xE000E5D8 => &mut self.ipr118,
            0xE000E5DC => &mut self.ipr119,
            0xE000E5E0 => &mut self.ipr120,
            0xE000E5E4 => &mut self.ipr121,
            0xE000E5E8 => &mut self.ipr122,
            0xE000E5EC => &mut self.ipr123,

            // RESERVED
            0xE000E380..=0xE000E3FC => panic!("Write to reserved addr"),
            0xE000E7F0..=0xE000ECFC => panic!("Write to reserved addr"),
            _ => panic!("Write to invalid addr"),
        }
    }

    pub fn read(&self, address: u32) -> u32 {
        // everything in NVIC is read / write
        self.addr_into_reg_value(address)
    }

    pub fn write(&mut self, address: u32, value: u32) {
        // everything in NVIC is read / write
        let reg = self.addr_into_reg_mut(address);
        *reg = value;
    }
}
