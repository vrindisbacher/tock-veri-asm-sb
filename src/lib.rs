pub mod armv7m;

use armv7m::{
    cpu::Armv7m,
    lang::{GeneralPurposeRegister, SpecialRegister},
};

// NOTE: This is not proveable
// #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: check_mem_value_write(0x2, new_cpu.mem, 0) } )]
// pub fn test_invalid_mem_access(armv7m: &mut Armv7m) {
//     armv7m.pseudo_ldr(GeneralPurposeRegister::R3, 0x0);
//     armv7m.movw_imm(GeneralPurposeRegister::R3, 0);
//     armv7m.movw_imm(GeneralPurposeRegister::R2, 1);
//     armv7m.strw_lsl_reg(
//         GeneralPurposeRegister::R0,
//         GeneralPurposeRegister::R3,
//         GeneralPurposeRegister::R2,
//         2,
//     );
// }

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
#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu:  
        // note basically checks that NVIC ISER & ICER bits are set
        new_cpu.r0 == get_ipsr(old_cpu) % 32  
        &&
        new_cpu.r2 == get_ipsr(old_cpu) / 32 * 4 
        &&
        // nvic iser bit for ipsr is correct
        nth_bit(
            get_mem_value(0xE000_E180 + new_cpu.r2, new_cpu.mem), and(get_ipsr(old_cpu), 31)
        ) == 0
        &&
        // nvic icer bit for ipsr is correct
        nth_bit(
            get_mem_value(0xE000_E200 + new_cpu.r2, new_cpu.mem), and(get_ipsr(old_cpu), 31)
        ) == 0
        && 
        false
    }    
)]
pub fn generic_isr_armv7m(armv7m: &mut Armv7m) {
    // r0 = 0
    armv7m.movw_imm(GeneralPurposeRegister::R0, 0);
    // control = r0 = 0
    armv7m.msr(SpecialRegister::Control, GeneralPurposeRegister::R0);
    // isb
    armv7m.isb(Some(armv7m::lang::IsbOpt::Sys));

    // NOTE: using pseudo instr here 
    // lr = 0xFFFFFFF9
    armv7m.pseudo_ldr(GeneralPurposeRegister::Lr, 0xFFFFFFF9);

    // r0 = ipsr
    armv7m.mrs(GeneralPurposeRegister::R0, SpecialRegister::IPSR);
    // Note: this seems to be a useless instruction?
    armv7m.and_imm(GeneralPurposeRegister::R0, 0xff);
    // r0 = ipsr - 16
    armv7m.subw_imm(GeneralPurposeRegister::R0, GeneralPurposeRegister::R0, 16);

    // r2 = r0 >> 5 ---> (ipsr / 32)
    armv7m.lsrs_imm(GeneralPurposeRegister::R2, GeneralPurposeRegister::R0, 5);
    // r3 = 1
    armv7m.movs_imm(GeneralPurposeRegister::R3, 1);
    // r0 = r0 & 31
    armv7m.and_imm(GeneralPurposeRegister::R0, 31);
    
    // r0 = r3 << r0
    //      -     -
    //      1     (ipsr & 31)
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

    // r0 = 1 << (ipsr & 31)
    // r3 = 0xe000_e180
    // r2 = (ipsr >> 5) << 2
    armv7m.strw_lsl_reg(
        GeneralPurposeRegister::R0,
        GeneralPurposeRegister::R3,
        GeneralPurposeRegister::R2,
        2,
    );

    // Note: Ignoring the dissasembled version of this because dealing with program counter is
    // annoying
    //
    // Gonna encode this as a pseudo instruction for now
    armv7m.pseudo_ldr(GeneralPurposeRegister::R3, 0xe000e200);

    // r0 = 1 << (ipsr & 31)
    // r3 = 0xe000_e200
    // r2 = (ipsr >> 5) << 2
    //
    // mem[0xe000_e200] + ((ipsr >> 5) << 2) = (1 << ipsr & 31) i.e. "bit for the ipsr # is set"
    armv7m.strw_lsl_reg(
        GeneralPurposeRegister::R0,
        GeneralPurposeRegister::R3,
        GeneralPurposeRegister::R2,
        2,
    );

    armv7m.bx(GeneralPurposeRegister::Lr);
}

#[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu:
    new_cpu.r0 == get_ipsr(old_cpu) % 32 && false
})]
fn obviously_wrong(armv7m: &mut Armv7m) {
    armv7m.lsrs_imm(GeneralPurposeRegister::R2, GeneralPurposeRegister::R0, 5);
}
