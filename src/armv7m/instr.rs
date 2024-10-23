#[derive(Debug, PartialEq, Eq)]
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
    #[variant(GeneralPurposeRegister[12])]
    R12,
    #[variant(GeneralPurposeRegister[13])]
    Sp,
    #[variant(GeneralPurposeRegister[14])]
    Lr,
    #[variant(GeneralPurposeRegister[15])]
    Pc,
}

#[flux_rs::refined_by(n : int)]
pub enum SpecialRegister {
    #[variant(SpecialRegister[16])]
    Control,
    // PSR and one of the sub register (IPSR)
    #[variant(SpecialRegister[17])]
    PSR,
    #[variant(SpecialRegister[18])]
    IPSR,
}

#[flux_rs::refined_by(is_reg: bool, is_special: bool, val: int)]
// NOTE: Glossing over negative values...
pub enum Value {
    #[variant({SpecialRegister[@n]} -> Value[true, true, n])]
    SpecialRegister(SpecialRegister),
    #[variant({GeneralPurposeRegister[@n]} -> Value[true, false, n])]
    GeneralRegister(GeneralPurposeRegister),
    #[variant({u32[@n]} -> Value[false, false, n])]
    Value(u32),
}

#[derive(Debug)]
pub enum IsbOpt {
    Sys,
}
