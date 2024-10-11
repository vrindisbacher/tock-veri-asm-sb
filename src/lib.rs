pub mod armv7m;

use armv7m::cpu::{Armv7m, GeneralPurposeRegister, SpecialRegister, Value};

pub fn generic_isr_armv7m(mut armv7m: Armv7m) {
    armv7m.mov(GeneralPurposeRegister::R0, Value::Value(0));
    armv7m.msr(
        SpecialRegister::Control,
        Value::Register(GeneralPurposeRegister::R0),
    );
    armv7m.isb();
    
    // R14 is the link register
    armv7m.ldr(GeneralPurposeRegister::R14, Value::Value(0xFFFFFFF9));

    armv7m.mrs(GeneralPurposeRegister::R0, SpecialRegister::ISPR);
    armv7m.and(GeneralPurposeRegister::R0, Value::Value(0xFF), None);
    armv7m.sub(GeneralPurposeRegister::R0, Value::Value(16));

    armv7m.lsrs(
        GeneralPurposeRegister::R2,
        Value::Register(GeneralPurposeRegister::R0),
        Value::Value(5),
    );
    armv7m.movs(GeneralPurposeRegister::R3, Value::Value(1));
    armv7m.and(
        GeneralPurposeRegister::R0,
        Value::Register(GeneralPurposeRegister::R0),
        Some(Value::Value(31)),
    );
    armv7m.lsl(
        GeneralPurposeRegister::R0,
        Value::Register(GeneralPurposeRegister::R3),
        Value::Register(GeneralPurposeRegister::R0),
    );

    armv7m.ldr(GeneralPurposeRegister::R3, Value::Value(0xe000e180));

    // VTOCK TODO - is it ok to just hard code 4 here - also I don't our encoding of this is right ????
    // str r0, [r3, r2, lsl #2]          // *(r3 + r2 * 4) = r0
    armv7m.str(
        GeneralPurposeRegister::R0,
        vec![
            Value::Register(GeneralPurposeRegister::R3),
            Value::Register(GeneralPurposeRegister::R2),
            Value::Value(4),
        ],
    );

    armv7m.ldr(GeneralPurposeRegister::R3, Value::Value(0xe000e200));

    // VTOCK TODO - is it ok to just hard code 4 here - also I don't our encoding of this is right ????
    // str r0, [r3, r2, lsl #2]          // *(r3 + r2 * 4) = r0
    armv7m.str(
        GeneralPurposeRegister::R0,
        vec![
            Value::Register(GeneralPurposeRegister::R3),
            Value::Register(GeneralPurposeRegister::R2),
            Value::Value(4),
        ],
    );

    armv7m.bx(GeneralPurposeRegister::R14);
}
