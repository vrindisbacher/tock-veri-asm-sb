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

//
// See here for a full set of special registers in the Thumb Instruction Set: https://developer.arm.com/documentation/ddi0403/d/System-Level-Architecture/System-Instruction-Details/About-the-ARMv7-M-system-instructions/Special-register-encodings-used-in-ARMv7-M-system-instructions?lang=en
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
pub enum Value {
    #[variant({SpecialRegister[@n]} -> Value[true, true, n])]
    SpecialRegister(SpecialRegister),
    #[variant({GeneralPurposeRegister[@n]} -> Value[true, false, n])]
    GeneralRegister(GeneralPurposeRegister),
    #[variant({u32[@n]} -> Value[false, false, n])]
    Value(u32),
}
