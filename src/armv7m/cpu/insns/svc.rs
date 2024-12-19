use crate::armv7m::cpu::Armv7m;

impl Armv7m {
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], u8[@svc_num]) 
            requires 
                // Stack Pointer is valid and can grow downwards 20 bytes
                sp_can_handle_exception_entry(cpu)
                &&
                // and Stack Pointer used on exit is valid and can grow upwards 20 bytes
                sp_can_handle_preempt_exception_exit(cpu, 11)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_preempt(cpu, 11) }
    )]
    pub fn svc(&mut self, svc_num: u8) {
        // TODO: Finish this instruction
        self.preempt(11);
    }
}
