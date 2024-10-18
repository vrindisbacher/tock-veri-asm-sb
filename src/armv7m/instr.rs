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
}

pub enum SpecialRegister {
    Control,
    IPSR,
}

#[flux_rs::refined_by(is_reg: bool, val: int)]
pub enum Value {
    #[variant({GeneralPurposeRegister[@n]} -> Value[true, n])]
    Register(GeneralPurposeRegister),
    #[variant({u32[@n]} -> Value[false, n])]
    Value(u32),
}
