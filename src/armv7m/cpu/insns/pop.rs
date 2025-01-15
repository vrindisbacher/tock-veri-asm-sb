use crate::armv7m::{
    cpu::Armv7m,
    lang::{SpecialRegister, GPR},
};

use flux_rs::bitvec::BV32;

flux_rs::defs! {
    fn gprs_post_pop(cpu: Armv7m, sp: BV32, r1: int, r2: int, r3: int, r4: int) -> Map<GPR, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        cpu.general_regs,
                        r1,
                        get_mem_addr(sp, cpu.mem)
                    ),
                    r2,
                    get_mem_addr(bv_add(sp, bv32(0x4)), cpu.mem)
                ),
                r3,
                get_mem_addr(bv_add(sp, bv32(0x8)), cpu.mem)
            ),
            r4,
            get_mem_addr(bv_add(sp, bv32(0xc)), cpu.mem)
        )
    }
}

impl Armv7m {
    #[flux_rs::sig(
        fn (
            self: &strg Armv7m[@old_cpu],
            GPR[@r1],
            GPR[@r2],
            GPR[@r3],
            GPR[@r4],
            SpecialRegister[@r5]
        )
            requires
                is_valid_ram_addr(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control))
                &&
                is_valid_ram_addr(bv_add(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x14)))
                &&
                !is_sp(r5)
                &&
                !is_psp(r5)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_pop(old_cpu, r1, r2, r3, r4, r5) }
    )]
    pub fn pop(&mut self, r1: GPR, r2: GPR, r3: GPR, r4: GPR, r5: SpecialRegister) {
        let mut sp = self.get_value_from_special_reg(&SpecialRegister::sp());

        let val1 = self.mem.read(sp);
        self.update_general_reg_with_b32(r1, val1);
        sp = sp + BV32::from(0x4);

        let val2 = self.mem.read(sp);
        self.update_general_reg_with_b32(r2, val2);
        sp = sp + BV32::from(0x4);

        let val3 = self.mem.read(sp);
        self.update_general_reg_with_b32(r3, val3);
        sp = sp + BV32::from(0x4);

        let val4 = self.mem.read(sp);
        self.update_general_reg_with_b32(r4, val4);
        sp = sp + BV32::from(0x4);

        let val5 = self.mem.read(sp);
        self.update_special_reg_with_b32(r5, val5);
        sp = sp + BV32::from(0x4);

        self.update_special_reg_with_b32(SpecialRegister::sp(), sp);
    }
}
