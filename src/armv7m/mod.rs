use instrs::Prgm;
use reg::Armv7m;

pub mod instrs;
mod reg;

// NOTE:
// This defines a tiny subset of the Thumb instruction set and the architecture of Armv7M
//
// Based on https://developer.arm.com/documentation/ddi0403/ee/?lang=en
//

pub fn emulate(prgm: Prgm) -> Armv7m {
    let armv7m = Armv7m::new();
    armv7m.execute(prgm)
}
