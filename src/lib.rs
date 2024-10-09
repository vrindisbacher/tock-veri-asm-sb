use armv7m::{emulate, instrs::Prgm};

pub mod armv7m;

pub fn emulate_arm(prgm: Prgm) {
    let res = emulate(prgm);
    println!("{res:?}");
}
