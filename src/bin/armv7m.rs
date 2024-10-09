use veriasm::{
    emulate_arm,
    armv7m::instrs::{Instr, InstrRegister, Value},
};

fn main() {
    emulate_arm(vec![Instr::Mov(InstrRegister::R0, Value::Value(5))]);
}
