use crate::armv7m::lang::IsbOpt;

use super::super::Armv7m;

impl Armv7m {
    #[flux_rs::sig(fn (self: &strg Armv7m, _) ensures self: Armv7m)]
    pub fn isb(&mut self, _opt: Option<IsbOpt>) {
        // a no-op in our case
    }
}
