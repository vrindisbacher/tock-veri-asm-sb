use crate::armv7m::cpu::Armv7m;

impl Armv7m {
    pub fn svc(&mut self, svc_num: u8) {
        self.preempt(11);
    }
}
