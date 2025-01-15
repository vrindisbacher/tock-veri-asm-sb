use crate::armv7m::{
    cpu::Armv7m,
    lang::{SpecialRegister, GPR},
};
use flux_rs::bitvec::BV32;

flux_rs::defs! {
    fn mem_post_push(
        cpu: Armv7m,
        r1: int,
        r2: int,
        r3: int,
        r4: int,
        r5: int
    ) -> Map<BV32, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        map_set(
                            cpu.mem,
                            bv_sub(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x14)),
                            get_gpr(r1, cpu)
                        ),
                        bv_sub(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x10)),
                        get_gpr(r2, cpu)
                    ),
                    bv_sub(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0xc)),
                    get_gpr(r3, cpu)
                ),
                bv_sub(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x8)),
                get_gpr(r4, cpu)
            ),
            bv_sub(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x4)),
            get_special_reg(r5, cpu)
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
                is_valid_ram_addr(bv_sub(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x14)))
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                    mem: mem_post_push(old_cpu, r1, r2, r3, r4, r5),
                    sp: set_sp(
                        old_cpu.sp,
                        old_cpu.mode,
                        old_cpu.control,
                        bv_sub(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x14))
                    ),
                    ..old_cpu
                }
            }
    )]
    pub fn push(&mut self, r1: GPR, r2: GPR, r3: GPR, r4: GPR, r5: SpecialRegister) {
        // NOTE: using set number of registers because of instrs we need to report
        let mut sp = self.get_value_from_special_reg(&SpecialRegister::sp()) - BV32::from(0x14);

        // NOTE: write the lowest first - in this case we can cheat a little bit
        // because we know that the args passed go from least to greatest

        let val1 = self.get_value_from_general_reg(&r1);
        self.mem.write(sp, val1);
        sp = sp + BV32::from(0x4);

        let val2 = self.get_value_from_general_reg(&r2);
        self.mem.write(sp, val2);
        sp = sp + BV32::from(0x4);

        let val3 = self.get_value_from_general_reg(&r3);
        self.mem.write(sp, val3);
        sp = sp + BV32::from(0x4);

        let val4 = self.get_value_from_general_reg(&r4);
        self.mem.write(sp, val4);
        sp = sp + BV32::from(0x4);

        let val5 = self.get_value_from_special_reg(&r5);
        self.mem.write(sp, val5);
        sp = sp + BV32::from(0x4);

        self.update_special_reg_with_b32(SpecialRegister::sp(), sp - BV32::from(0x14));
    }
}
