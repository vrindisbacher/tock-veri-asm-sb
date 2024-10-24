use crate::armv7m::lang::IsbOpt;

use super::super::Armv7m;

impl Armv7m {
    pub fn isb(&mut self, opt: Option<IsbOpt>) {
        // a no-op in our case I think
    }
}
