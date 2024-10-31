use crate::armv7m::lang::IsbOpt;

use super::super::Armv7m;

impl Armv7m {
    #[flux_rs::sig(fn (self: &strg Armv7m[@cpu], _) ensures self: Armv7m[cpu])]
    pub fn isb(&mut self, _opt: Option<IsbOpt>) {
        // a no-op in our case
    }
}
