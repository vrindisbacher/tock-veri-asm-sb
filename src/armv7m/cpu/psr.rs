use super::flux_defs::*;
use super::Armv7m;

// Manages PSR state, specifically the conditional flags:
//
// N, bit [31]
// Negative condition code flag. Set to bit [31] of the result of the instruction. If the result is regarded as a two's complement signed integer, then N == 1 if the result is negative and N == 0 if it is positive or zero.
//
// Z, bit [30]
// Zero condition code flag. Set to 1 if the result of the instruction is zero, and to 0 otherwise. A result of zero often indicates an equal result from a comparison.
// C, bit [29]
// Carry condition code flag. Set to 1 if the instruction results in a carry condition, for example an unsigned overflow on an addition.
//
// V, bit [28]
// Overflow condition code flag. Set to 1 if the instruction results in an overflow condition, for example a signed overflow on an addition.
//
// Q, bit [27]
//
// Set to 1 if a SSAT or USAT instruction changes the input value for the signed or unsigned range of the result. In a processor that implements the DSP extension, the processor sets this bit to 1 to indicate an overflow on some multiplies. Setting this bit to 1 is called saturation.

impl Armv7m {
    #[flux_rs::trusted]
    #[flux_rs::sig(fn (&Armv7m[@cpu]) -> bool[nth_bit_is_set(cpu.psr, 31)])]
    pub fn n_flag_set(&self) -> bool {
        self.psr & (1 << 31) == 1
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_set(new_cpu.psr, 31) })]
    pub fn set_n_flag(&mut self) {
        self.psr = self.psr | (1 << 31);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_unset(new_cpu.psr, 31) })]
    pub fn unset_n_flag(&mut self) {
        self.psr = self.psr & (1 << 31);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (&Armv7m[@cpu]) -> bool[nth_bit_is_set(cpu.psr, 30)])]
    pub fn z_flag_set(&self) -> bool {
        self.psr & (1 << 30) == 1
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_set(new_cpu.psr, 30) })]
    pub fn set_z_flag(&mut self) {
        self.psr = self.psr | (1 << 30);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_unset(new_cpu.psr, 30) })]
    pub fn unset_z_flag(&mut self) {
        self.psr = self.psr & (1 << 30);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (&Armv7m[@cpu]) -> bool[nth_bit_is_set(cpu.psr, 29)])]
    pub fn c_flag_set(&self) -> bool {
        self.psr & (1 << 29) == 1
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_set(new_cpu.psr, 29) })]
    pub fn set_c_flag(&mut self) {
        self.psr = self.psr | (1 << 29);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_unset(new_cpu.psr, 29) })]
    pub fn unset_c_flag(&mut self) {
        self.psr = self.psr & (1 << 29);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (&Armv7m[@cpu]) -> bool[nth_bit_is_set(cpu.psr, 28)])]
    pub fn v_flag_set(&self) -> bool {
        self.psr & (1 << 28) == 1
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_set(new_cpu.psr, 28) })]
    pub fn set_v_flag(&mut self) {
        self.psr = self.psr | (1 << 28);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_unset(new_cpu.psr, 28) })]
    pub fn unset_v_flag(&mut self) {
        self.psr = self.psr & (1 << 28);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (&Armv7m[@cpu]) -> bool[nth_bit_is_set(cpu.psr, 27)])]
    pub fn q_flag_set(&self) -> bool {
        self.psr & (1 << 27) == 1
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_set(new_cpu.psr, 27) })]
    pub fn set_q_flag(&mut self) {
        self.psr = self.psr | (1 << 27);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: nth_bit_is_unset(new_cpu.psr, 27) })]
    pub fn unset_q_flag(&mut self) {
        self.psr = self.psr & (1 << 27);
    }
}
