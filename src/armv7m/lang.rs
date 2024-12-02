#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[flux_rs::refined_by(n: int)]
pub enum GPR {
    #[variant(GPR[0])]
    R0,
    #[variant(GPR[1])]
    R1,
    #[variant(GPR[2])]
    R2,
    #[variant(GPR[3])]
    R3,
    #[variant(GPR[4])]
    R4,
    #[variant(GPR[5])]
    R5,
    #[variant(GPR[6])]
    R6,
    #[variant(GPR[7])]
    R7,
    #[variant(GPR[8])]
    R8,
    #[variant(GPR[9])]
    R9,
    #[variant(GPR[10])]
    R10,
    #[variant(GPR[11])]
    R11,
    #[variant(GPR[12])]
    R12,
}

impl GPR {
    #[flux_rs::sig(fn () -> GPR[r0()])]
    pub fn r0() -> Self {
        Self::R0
    }

    #[flux_rs::sig(fn () -> GPR[r1()])]
    pub fn r1() -> Self {
        Self::R1
    }

    #[flux_rs::sig(fn () -> GPR[r2()])]
    pub fn r2() -> Self {
        Self::R2
    }

    #[flux_rs::sig(fn () -> GPR[r3()])]
    pub fn r3() -> Self {
        Self::R3
    }

    #[flux_rs::sig(fn () -> GPR[r12()])]
    pub fn r12() -> Self {
        Self::R12
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[flux_rs::refined_by(n : int)]
pub enum SpecialRegister {
    #[variant(SpecialRegister[13])]
    Sp,
    #[variant(SpecialRegister[14])]
    Lr,
    #[variant(SpecialRegister[15])]
    Pc,
    #[variant(SpecialRegister[16])]
    Control,
    // PSR and one of the sub register (IPSR)
    #[variant(SpecialRegister[17])]
    PSR,
    #[variant(SpecialRegister[18])]
    IPSR,
}

impl SpecialRegister {
    #[flux_rs::sig(fn () -> SpecialRegister[sp()])]
    pub fn sp() -> Self {
        Self::Sp
    }

    #[flux_rs::sig(fn () -> SpecialRegister[lr()])]
    pub fn lr() -> Self {
        Self::Lr
    }

    #[flux_rs::sig(fn () -> SpecialRegister[psr()])]
    pub fn psr() -> Self {
        Self::PSR
    }
}

#[derive(Debug)]
pub enum IsbOpt {
    Sys,
}
