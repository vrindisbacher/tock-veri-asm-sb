pub mod armv7m;

use armv7m::{
    cpu::Armv7m,
    lang::{GeneralPurposeRegister, SpecialRegister, Value},
};

//
// Here is disassembly of the armv7m program. Note that the .w specifies "wide"
// for the 32 bit version of the instruction
//
//
//   0:   f04f 0000       mov.w   r0, #0
//   4:   f380 8814       msr     CONTROL, r0
//   8:   f3bf 8f6f       isb     sy
//   c:   f06f 0e06       mvn.w   lr, #6
//   10:   f3ef 8005       mrs     r0, IPSR
//   14:   f000 00ff       and.w   r0, r0, #255    @ 0xff
//   18:   f1a0 0010       sub.w   r0, r0, #16
//   1c:   0942            lsrs    r2, r0, #5
//   1e:   2301            movs    r3, #1
//   20:   f000 001f       and.w   r0, r0, #31
//   24:   fa03 f000       lsl.w   r0, r3, r0
//   28:   4b03            ldr     r3, [pc, #12]   @ (38 <generic_isr_arm_v7m+0x38>)
//   2a:   f843 0022       str.w   r0, [r3, r2, lsl #2]
//   2e:   4b03            ldr     r3, [pc, #12]   @ (3c <generic_isr_arm_v7m+0x3c>)
//   30:   f843 0022       str.w   r0, [r3, r2, lsl #2]
//   34:   4770            bx      lr
//   38:   e000e180        .word   0xe000e180
//   3c:   e000e200        .word   0xe000e200
//
pub fn generic_isr_armv7m(mut armv7m: Armv7m) {
    armv7m.movw_imm(GeneralPurposeRegister::R0, 0);
    armv7m.msr(SpecialRegister::Control, GeneralPurposeRegister::R0);
    armv7m.isb(Some(armv7m::lang::IsbOpt::Sys));

    armv7m.mvn_imm(GeneralPurposeRegister::Lr, 6);

    armv7m.mrs(GeneralPurposeRegister::R0, SpecialRegister::IPSR);
    armv7m.and_imm(GeneralPurposeRegister::R0, 0xff);
    armv7m.subw_imm(GeneralPurposeRegister::R0, GeneralPurposeRegister::R0, 16);

    armv7m.lsrs_imm(GeneralPurposeRegister::R2, GeneralPurposeRegister::R0, 5);
    armv7m.movs_imm(GeneralPurposeRegister::R3, 0);
    armv7m.and_imm(GeneralPurposeRegister::R0, 31);
    armv7m.lslw_reg(
        GeneralPurposeRegister::R0,
        GeneralPurposeRegister::R3,
        GeneralPurposeRegister::R0,
    );

    // Note: Ignoring the dissasembled version of this because dealing with program counter is
    // annoying
    //
    // Gonna encode this as a pseudo instruction for now
    armv7m.pseudo_ldr(GeneralPurposeRegister::R3, 0xe000e180);

    // VTOCK TODO - is it ok to just hard code 4 here - also I don't our encoding of this is right ????
    // str r0, [r3, r2, lsl #2]          // *(r3 + r2 * 4) = r0
    armv7m.str(
        GeneralPurposeRegister::R0,
        vec![
            Value::GeneralRegister(GeneralPurposeRegister::R3),
            Value::GeneralRegister(GeneralPurposeRegister::R2),
            Value::Value(4),
        ],
    );

    // Note: Ignoring the dissasembled version of this because dealing with program counter is
    // annoying
    //
    // Gonna encode this as a pseudo instruction for now
    armv7m.pseudo_ldr(GeneralPurposeRegister::R3, 0xe000e200);

    // VTOCK TODO - is it ok to just hard code 4 here - also I don't our encoding of this is right ????
    // str r0, [r3, r2, lsl #2]          // *(r3 + r2 * 4) = r0
    armv7m.str(
        GeneralPurposeRegister::R0,
        vec![
            Value::GeneralRegister(GeneralPurposeRegister::R3),
            Value::GeneralRegister(GeneralPurposeRegister::R2),
            Value::Value(4),
        ],
    );

    armv7m.bx(GeneralPurposeRegister::Lr);
}
