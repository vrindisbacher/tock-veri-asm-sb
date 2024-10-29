#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[flux_rs::refined_by(n: int)]
pub enum GeneralPurposeRegister {
    #[variant(GeneralPurposeRegister[0])]
    R0,
    #[variant(GeneralPurposeRegister[1])]
    R1,
    #[variant(GeneralPurposeRegister[2])]
    R2,
    #[variant(GeneralPurposeRegister[3])]
    R3,
    #[variant(GeneralPurposeRegister[4])]
    R4,
    #[variant(GeneralPurposeRegister[5])]
    R5,
    #[variant(GeneralPurposeRegister[6])]
    R6,
    #[variant(GeneralPurposeRegister[7])]
    R7,
    #[variant(GeneralPurposeRegister[8])]
    R8,
    #[variant(GeneralPurposeRegister[9])]
    R9,
    #[variant(GeneralPurposeRegister[10])]
    R10,
    #[variant(GeneralPurposeRegister[11])]
    R11,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[flux_rs::refined_by(n : int)]
pub enum SpecialRegister {
    #[variant(SpecialRegister[12])]
    // R12 is used for IPC
    R12,
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

#[derive(Debug)]
pub enum IsbOpt {
    Sys,
}
