use super::Memory;
use super::{Armv7m, CPUMode, Control, SP};
use crate::armv7m::lang::{SpecialRegister, GPR};
use crate::flux_support::rmap::*;
use flux_rs::bitvec::BV32;

const U32_MAX: u32 = std::u32::MAX;

flux_rs::defs! {

    fn cpu_post_exception_entry(cpu: Armv7m, exception_num: int) -> Armv7m {
        Armv7m {
            mode: handler_mode(),
            control: control_post_exception_entry(cpu),
            psr: psr_post_exception_entry(cpu, exception_num),
            lr: lr_post_exception_entry(cpu, cpu.control),
            sp: sp_post_exception_entry(cpu),
            mem: mem_post_exception_entry(get_sp(sp_post_exception_entry(cpu), cpu.mode, cpu.control), cpu),
            ..cpu
        }
    }

    fn get_lr_direct(cpu: Armv7m) -> BV32 {
        cpu.lr
    }

    fn get_sp_direct(cpu: Armv7m) -> SP {
        cpu.sp
    }

    fn get_mem_direct(cpu: Armv7m) -> Memory {
        cpu.mem
    }

    fn cpu_post_exception_exit(cpu: Armv7m, return_exec: BV32) -> Armv7m {
        Armv7m {
            mode: thread_mode(),
            control: Control { spsel: return_exec != bv32(0xFFFF_FFF9), ..cpu.control },
            general_regs: gprs_post_exception_exit(get_sp_from_isr_ret(cpu.sp, return_exec), cpu),
            lr: get_mem_addr(bv_add(get_sp_from_isr_ret(cpu.sp, return_exec), bv32( 0x14)), cpu.mem),
            psr: get_mem_addr(bv_add(get_sp_from_isr_ret(cpu.sp, return_exec), bv32(0x1C)), cpu.mem),
            sp: sp_post_exception_exit(cpu.sp, return_exec),
            ..cpu
        }
    }

    fn cpu_post_preempt(cpu: Armv7m, exception_num: int) -> Armv7m {
        cpu_post_exception_exit(
            cpu_post_run_isr(
                cpu_post_exception_entry(cpu, exception_num),
                exception_num
            ),
            get_bx_from_exception_num(exception_num, get_lr_direct(cpu_post_exception_entry(cpu, exception_num)))
        )
    }

    fn get_bx_from_exception_num(exception_num: int, lr: BV32) -> BV32 {
        if exception_num == 11 && lr == bv32(0xFFFF_FFF9) {
            bv32(0xFFFF_FFFD)
        } else {
            bv32(0xFFFF_FFF9)
        }
    }

    fn get_sp_from_exception_num(sp: SP, lr: BV32, exception_num: int) -> BV32 {
        if exception_num == 11 {
            // svc - depends on where we're coming from right now
            if lr == bv32(0xFFFF_FF1) || lr == bv32(0xFFFF_FFFD) {
                sp.sp_main
            } else {
                sp.sp_process
            }
        } else {
            sp.sp_main
        }
    }

    fn get_sp_from_isr_ret(sp: SP, return_exec: BV32) -> BV32 {
        if return_exec == bv32(0xFFFF_FFFF9) {
            sp.sp_main
        } else {
            sp.sp_process
        }
    }

    fn sp_post_exception_exit(sp: SP, return_exec: BV32) -> SP {
        if return_exec == bv32(0xFFFF_FFFF9) {
            SP { sp_main: bv_add(sp.sp_main, bv32(0x20)), ..sp }
        } else {
            SP { sp_process: bv_add(sp.sp_process, bv32(0x20)), ..sp }
        }

    }

    fn gprs_post_exception_exit(sp: BV32, cpu: Armv7m) -> Map<GPR, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        map_set(
                            cpu.general_regs,
                            r0(),
                            get_mem_addr(sp, cpu.mem)
                        ),
                        r1(),
                        get_mem_addr(bv_add(sp, bv32(0x4)), cpu.mem)
                    ),
                    r2(),
                    get_mem_addr(bv_add(sp, bv32(0x8)), cpu.mem)
                ),
                r3(),
                get_mem_addr(bv_add(sp, bv32(0xc)), cpu.mem)
            ),
            r12(),
            get_mem_addr(bv_add(sp, bv32(0x10)), cpu.mem)
        )
    }

    fn gprs_post_exception_exit_write_regs(
        cpu: Armv7m,
        val_r0: BV32,
        val_r1: BV32,
        val_r2: BV32,
        val_r3: BV32,
        val_r12: BV32,
    ) -> Map<GPR, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        map_set(
                            cpu.general_regs,
                            r0(),
                            val_r0
                        ),
                        r1(),
                        val_r1
                    ),
                    r2(),
                    val_r2
                ),
                r3(),
                val_r3
            ),
            r12(),
            val_r12
        )
    }

    fn register_frame_preserved(addr: BV32, old_cpu: Armv7m, new_cpu: Armv7m) -> bool {
        map_get(
            old_cpu.mem,
            addr
        ) == map_get(
            new_cpu.mem,
            addr
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x4))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x4))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x8))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x8))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0xc))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0xc))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x10))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x10))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x14))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x14))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x18))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x18))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x1c))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x1c))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x20))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x20))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x24))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x24))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x28))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x28))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x2c))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x2c))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x30))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x30))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x34))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x34))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x38))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x38))
        )
        &&
        map_get(
            old_cpu.mem,
            bv_add(addr, bv32(0x3c))
        ) == map_get(
            new_cpu.mem,
            bv_add(addr, bv32(0x3c))
        )
    }

    fn mem_post_exception_entry(sp: BV32, cpu: Armv7m) -> Map<BV32, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        map_set(
                            map_set(
                                map_set(
                                    map_set(
                                        cpu.mem,
                                        sp,
                                        get_gpr(r0(), cpu)
                                    ),
                                    bv_add(sp, bv32(0x4)),
                                    get_gpr(r1(), cpu)
                                ),
                                bv_add(sp, bv32(0x8)),
                                get_gpr(r2(), cpu)
                            ),
                            bv_add(sp, bv32(0xc)),
                            get_gpr(r3(), cpu)
                        ),
                        bv_add(sp, bv32(0x10)),
                        get_gpr(r12(), cpu)
                    ),
                    bv_add(sp, bv32(0x14)),
                    get_special_reg(lr(), cpu)
                ),
                bv_add(sp, bv32(0x18)),
                bv32(0) // nonsense value
            ),
            bv_add(sp, bv32(0x1c)),
            get_special_reg(psr(), cpu)
        )
    }

    fn mem_post_push_stack_write_gpr_vals(
        sp: BV32,
        cpu: Armv7m,
        r0: BV32,
        r1: BV32,
        r2: BV32,
        r3: BV32,
        r12: BV32,
        lr: BV32,
        psr: BV32
    ) -> Map<BV32, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        map_set(
                            map_set(
                                map_set(
                                    map_set(
                                        cpu.mem,
                                        sp,
                                        r0
                                    ),
                                    bv_add(sp, bv32(0x4)),
                                    r1
                                ),
                                bv_add(sp, bv32(0x8)),
                                r2
                            ),
                            bv_add(sp, bv32(0xc)),
                            r3
                        ),
                        bv_add(sp, bv32(0x10)),
                        r12
                    ),
                    bv_add(sp, bv32(0x14)),
                    lr
                ),
                bv_add(sp, bv32(0x18)),
                bv32(0) // nonsense value
            ),
            bv_add(sp, bv32(0x1c)),
            psr
        )
    }

    fn gprs_post_ldmia_w(
        cpu: Armv7m,
        rd: int,
        rm1: int,
        rm2: int,
        rm3: int,
        rm4: int,
        rm5: int,
        rm6: int,
        rm7: int,
        rm8: int
    ) -> Map<GPR, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        map_set(
                            map_set(
                                map_set(
                                    map_set(
                                        cpu.general_regs,
                                        rm1,
                                        get_mem_addr(get_gpr(rd, cpu), cpu.mem)
                                    ),
                                    rm2,
                                    get_mem_addr(bv_add(get_gpr(rd, cpu), bv32(0x4)), cpu.mem)
                                ),
                                rm3,
                                get_mem_addr(bv_add(get_gpr(rd, cpu), bv32(0x8)), cpu.mem)
                            ),
                            rm4,
                            get_mem_addr(bv_add(get_gpr(rd, cpu), bv32(0xc)), cpu.mem)
                        ),
                        rm5,
                        get_mem_addr(bv_add(get_gpr(rd, cpu), bv32(0x10)), cpu.mem)
                    ),
                    rm6,
                    get_mem_addr(bv_add(get_gpr(rd, cpu), bv32(0x14)), cpu.mem)
                ),
                rm7,
                get_mem_addr(bv_add(get_gpr(rd, cpu), bv32(0x18)), cpu.mem)
            ),
            rm8,
            get_mem_addr(bv_add(get_gpr(rd, cpu), bv32(0x1c)), cpu.mem)
        )
    }

    fn gprs_post_ldmia_w_special(
        cpu: Armv7m,
        rd: int,
        rm1: int,
        rm2: int,
        rm3: int,
    ) -> Map<GPR, BV32> {
        map_set(
            map_set(
                map_set(
                    cpu.general_regs,
                    rm1,
                    get_mem_addr(get_special_reg(rd, cpu), cpu.mem)
                ),
                rm2,
                get_mem_addr(bv_add(get_special_reg(rd, cpu), bv32(0x4)), cpu.mem)
            ),
            rm3,
            get_mem_addr(bv_add(get_special_reg(rd, cpu), bv32(0x8)), cpu.mem)
        )
    }

    fn mem_post_stmia_w(
        cpu: Armv7m,
        rd: int,
        rm1: int,
        rm2: int,
        rm3: int,
        rm4: int,
        rm5: int,
        rm6: int,
        rm7: int,
        rm8: int
    ) -> Map<BV32, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        map_set(
                            map_set(
                                map_set(
                                    map_set(
                                        cpu.mem,
                                        get_gpr(rd, cpu),
                                        get_gpr(rm1, cpu),
                                    ),
                                    bv_add(get_gpr(rd, cpu), bv32(0x4)),
                                    get_gpr(rm2, cpu)
                                ),
                                bv_add(get_gpr(rd, cpu), bv32(0x8)),
                                get_gpr(rm3, cpu)
                            ),
                            bv_add(get_gpr(rd, cpu), bv32(0xc)),
                            get_gpr(rm4, cpu)
                        ),
                        bv_add(get_gpr(rd, cpu), bv32(0x10)),
                        get_gpr(rm5, cpu)
                    ),
                    bv_add(get_gpr(rd, cpu), bv32(0x14)),
                    get_gpr(rm6, cpu)
                ),
                bv_add(get_gpr(rd, cpu), bv32(0x18)),
                get_gpr(rm7, cpu)
            ),
            bv_add(get_gpr(rd, cpu), bv32(0x1c)),
            get_gpr(rm8, cpu)
        )
    }

    fn switch_to_user_pt1_save_clobbers_precondition(cpu: Armv7m) -> bool {
        mode_is_thread_privileged(cpu.mode, cpu.control)
        &&
        is_valid_ram_addr(sp_main(cpu.sp))
        &&
        is_valid_ram_addr(bv_sub(sp_main(cpu.sp), bv32(0x3c)))
    }

    fn switch_to_user_pt1_reg_restores_precondition(cpu: Armv7m) -> bool {
        is_valid_ram_addr(get_gpr(r0(), cpu))
        &&
        is_valid_ram_addr(bv_add(get_gpr(r0(), cpu), bv32(0x20)))
        &&
        is_valid_ram_addr(get_gpr(r1(), cpu))
        &&
        is_valid_ram_addr(bv_add(get_gpr(r1(), cpu), bv32(0x1c)))
    }

    fn switch_to_user_pt1_precondition(cpu: Armv7m) -> bool {
        switch_to_user_pt1_save_clobbers_precondition(cpu)
        &&
        switch_to_user_pt1_reg_restores_precondition(cpu)
        &&
        sp_can_handle_exception_entry(
            cpu_post_switch_to_user_pt1_reg_restores(
                cpu_post_switch_to_user_pt1_save_clobbers(cpu)
            )
        )
        &&
        sp_can_handle_preempt_exception_exit(
            cpu_post_switch_to_user_pt1_reg_restores(
                cpu_post_switch_to_user_pt1_save_clobbers(cpu)
            ),
            11
        )
    }

    fn cpu_post_switch_to_user_pt1_reg_restores(cpu: Armv7m) -> Armv7m {
        Armv7m {
            general_regs: gprs_post_switch_to_user_pt1_reg_restores(cpu),
            sp: SP {
                sp_process: get_gpr(r0(), cpu),
                ..cpu.sp
            },
            ..cpu
        }
    }

    fn cpu_post_switch_to_user_pt1_save_clobbers(cpu: Armv7m) -> Armv7m {
        Armv7m {
            mem: mem_post_switch_to_user_pt1_save_clobbers(cpu),
            sp: SP {
                sp_main: bv_sub(sp_main(cpu.sp), bv32(0x20)),
                ..cpu.sp
            },
            ..cpu
        }
    }

    fn cpu_post_switch_to_user_pt1(cpu: Armv7m) -> Armv7m {
        cpu_post_preempt(
            cpu_post_switch_to_user_pt1_reg_restores(
                cpu_post_switch_to_user_pt1_save_clobbers(cpu)
            ),
            11
        )
    }

    fn gprs_post_switch_to_user_pt1_reg_restores(cpu: Armv7m) -> Map<GPR, BV32> {
        gprs_post_ldmia_w(
            Armv7m {
                general_regs: map_set(
                    map_set(
                        map_set(
                            cpu.general_regs,
                            r2(),
                            get_gpr(r6(), cpu)
                        ),
                        r3(),
                        get_gpr(r7(), cpu)
                    ),
                    r12(),
                    get_gpr(r9(), cpu)
                ),
                ..cpu
            },
            r1(), r4(), r5(), r6(), r7(), r8(), r9(), r10(), r11()
        )
    }

    fn mem_post_switch_to_user_pt1_save_clobbers(cpu: Armv7m) -> Map<BV32, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        map_set(
                            map_set(
                                map_set(
                                    map_set(
                                        cpu.mem,
                                        bv_sub(sp_main(cpu.sp), bv32(0x14)),
                                        get_gpr(r4(), cpu)
                                    ),
                                    bv_sub(sp_main(cpu.sp), bv32(0x10)),
                                    get_gpr(r5(), cpu)
                                ),
                                bv_sub(sp_main(cpu.sp), bv32(0xc)),
                                get_gpr(r6(), cpu)
                            ),
                            bv_sub(sp_main(cpu.sp), bv32(0x8)),
                            get_gpr(r7(), cpu)
                        ),
                        bv_sub(sp_main(cpu.sp), bv32(0x4)),
                        cpu.lr
                    ),
                    bv_sub(sp_main(cpu.sp), bv32(0x20)),
                    get_gpr(r8(), cpu),
                ),
                bv_sub(sp_main(cpu.sp), bv32(0x1c)),
                get_gpr(r10(), cpu),
            ),
            bv_sub(sp_main(cpu.sp), bv32(0x18)),
            get_gpr(r11(), cpu),
        )
    }

    fn switch_to_user_pt2_save_registers_precondition(cpu: Armv7m) -> bool {
        // need r1 to be valid store
        is_valid_ram_addr(get_gpr(r1(), cpu))
        &&
        is_valid_ram_addr(bv_add(get_gpr(r1(), cpu), bv32(0x1c)))
    }

    fn switch_to_user_pt2_restore_clobbers_precondition(cpu: Armv7m) -> bool {
        mode_is_thread_privileged(cpu.mode, cpu.control)
        &&
        is_valid_ram_addr(sp_main(cpu.sp))
        &&
        is_valid_ram_addr(bv_add(sp_main(cpu.sp), bv32(0x20)))
    }

    fn switch_to_user_pt2_precondition(cpu: Armv7m) -> bool {
        switch_to_user_pt2_save_registers_precondition(cpu)
        &&
        switch_to_user_pt2_restore_clobbers_precondition(cpu)
    }

    fn cpu_post_switch_to_user_pt2(cpu: Armv7m) -> Armv7m {
        cpu_post_switch_to_user_pt2_restore_clobbers(
            cpu_post_switch_to_user_pt2_save_registers(cpu)
        )
    }

    fn cpu_post_switch_to_user_pt2_save_registers(cpu: Armv7m) -> Armv7m {
        Armv7m {
            mem: mem_post_stmia_w(cpu, r1(), r4(), r5(), r6(), r7(), r8(), r9(), r10(), r11()),
            ..cpu
        }
    }

    fn cpu_post_switch_to_user_pt2_restore_clobbers(cpu: Armv7m) -> Armv7m {
        Armv7m {
            general_regs: gprs_post_switch_to_user_pt2_restore_clobbers(cpu),
            sp: SP {
                sp_main: bv_add(sp_main(cpu.sp), bv32(0x20)),
                ..cpu.sp
            },
            pc: get_mem_addr(bv_add(sp_main(cpu.sp), bv32(0x1c)), cpu.mem),
            ..cpu
        }
    }

    fn gprs_post_switch_to_user_pt2_restore_clobbers(cpu: Armv7m) -> Map<GPR, BV32> {
        map_set(
            map_set(
                map_set(
                    map_set(
                        map_set(
                            map_set(
                                map_set(
                                    map_set(
                                        map_set(
                                            cpu.general_regs,
                                            r0(),
                                            get_special_reg(psp(), cpu)
                                        ),
                                        r9(),
                                        get_gpr(r12(), cpu)
                                    ),
                                    r8(),
                                    get_mem_addr(sp_main(cpu.sp), cpu.mem)
                                ),
                                r10(),
                                get_mem_addr(bv_add(sp_main(cpu.sp), bv32(0x4)), cpu.mem)
                            ),
                            r11(),
                            get_mem_addr(bv_add(sp_main(cpu.sp), bv32(0x8)), cpu.mem)
                        ),
                        r4(),
                        get_mem_addr(bv_add(sp_main(cpu.sp), bv32(0xc)), cpu.mem)
                    ),
                    r5(),
                    get_mem_addr(bv_add(sp_main(cpu.sp), bv32(0x10)), cpu.mem)
                ),
                r6(),
                get_mem_addr(bv_add(sp_main(cpu.sp), bv32(0x14)), cpu.mem)
            ),
            r7(),
            get_mem_addr(bv_add(sp_main(cpu.sp), bv32(0x18)), cpu.mem)
        )
    }

    fn lr_post_exception_entry(cpu: Armv7m, control: Control) -> BV32 {
        if mode_is_handler(cpu.mode) {
            bv32(0xFFFF_FFF1)
        } else if control.spsel {
            bv32(0xFFFF_FFFD)
        } else {
            bv32(0xFFFF_FFF9)
        }
    }

    fn control_post_exception_entry(cpu: Armv7m) -> Control {
        Control { spsel: false, ..cpu.control }
    }

    fn psr_post_exception_entry(cpu: Armv7m, exception_num: int) -> BV32 {
        bv_or(bv_and(cpu.psr, bv_not(bv32(0xff))), bv32(exception_num))
    }

    fn sp_post_exception_entry(cpu: Armv7m) -> SP {
        set_sp(
            cpu.sp,
            cpu.mode,
            cpu.control,
            // bv_and(bv_sub(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x20)), bv_not(bv32(3)))
            bv_sub(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x20))
        )
    }

    fn push_stack_sp_precondition(sp: BV32) -> bool {
        is_valid_ram_addr(sp)
        &&
        is_valid_ram_addr(bv_add(sp, bv32(0x1C)))
    }

    fn sp_can_handle_exception_entry(cpu: Armv7m) -> bool {
        // requires we have enough space to push 8 x 4 byte values into mem
        is_valid_ram_addr(
            get_sp(cpu.sp, cpu.mode, cpu.control)
        )
        &&
        is_valid_ram_addr(
            bv_sub(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x20))
        )
    }

    fn sp_can_handle_exception_exit(sp: BV32) -> bool {
        is_valid_ram_addr(sp)
        &&
        is_valid_ram_addr(
            bv_add(
                sp,
                bv32(0x1c)
            )
        )
    }

    fn sp_can_handle_preempt_exception_exit(cpu: Armv7m, exception_num: int) -> bool {
        sp_can_handle_exception_exit(
            get_sp_from_isr_ret(
                sp_post_exception_entry(cpu),
                get_bx_from_exception_num(
                    exception_num,
                    lr_post_exception_entry(cpu, cpu.control)
                )
            )
        )
    }

    fn mode_is_thread_privileged(mode: int, control: Control) -> bool {
        mode == 1 && !control.spsel && !control.npriv
    }

    fn mode_is_thread_unprivileged(mode: int, control: Control) -> bool {
        mode == 1 && control.spsel && control.npriv
    }

    fn sp_main(sp: SP) -> BV32 {
        sp.sp_main
    }

    fn sp_process(sp: SP) -> BV32 {
        sp.sp_process
    }

    fn bv32(x: int) -> BV32 { bv_int_to_bv32(x) }

    fn get_gpr(reg: int, cpu: Armv7m) -> BV32 {
        map_get(cpu.general_regs, reg)
    }

    fn set_gpr(reg: int, old_cpu: Armv7m, val: BV32) -> Map<GPR, BV32> {
        map_set(old_cpu.general_regs, reg, val)
    }

    fn control_update(val: BV32, old_cpu: Armv7m) -> Control {
        if int(val) == 0 {
            Control { npriv: false, spsel: false }
        } else if int(val) == 1 {
            Control { npriv: true, spsel: false }
        } else if int(val) == 2 {
            Control { npriv: false, spsel: true }
        } else {
            Control { npriv: false, spsel: true }
        }
    }

    fn get_control(control: Control) -> BV32 {
        if control.npriv && control.spsel {
            bv32(3)
        } else if control.npriv {
            // first bit is 1 - i.e. 01
            bv32(1)
        } else if control.spsel {
            // second bit is 1 - i.e. 10
            bv32(2)
        } else {
            bv32(0)
        }
    }

    fn get_sp(sp: SP, mode: int, control: Control) -> BV32 {
        if mode_is_handler(mode) || !control.spsel {
            sp.sp_main
        } else {
            sp.sp_process
        }
    }

    fn get_psp(sp: SP) -> BV32 {
        sp.sp_process
    }

    fn get_special_reg(reg: int, cpu: Armv7m) -> BV32 {
        if is_psp(reg) {
            get_psp(cpu.sp)
        } else if is_sp(reg) {
            get_sp(cpu.sp, cpu.mode, cpu.control)
        } else if is_lr(reg) {
            cpu.lr
        } else if is_pc(reg) {
            cpu.pc
        } else if is_control(reg) {
            get_control(cpu.control)
        } else if is_psr(reg) {
            cpu.psr
        } else {
            // ipsr
            bv_and(cpu.psr, bv32(0xff))
        }
    }

    fn set_control(control: Control, mode: int, val: BV32) -> Control {
        Control {
            npriv: nth_bit_is_set(val, bv32(0)),
            spsel: if !mode_is_handler(mode) { nth_bit_is_set(val, bv32(1)) } else { control.spsel }
        }
    }

    fn set_sp(sp: SP, mode: int, control: Control, val: BV32) -> SP {
        if mode_is_handler(mode) || !control.spsel {
            SP { sp_main: val, ..sp }
        } else {
            SP { sp_process: val, ..sp }
        }
    }

    fn set_psp(sp: SP, val: BV32) -> SP {
        SP { sp_process: val, ..sp }
    }

    fn set_spr(reg: int, cpu: Armv7m, val: BV32) -> Armv7m {
        if is_psp(reg) {
            Armv7m { sp: set_psp(cpu.sp, val), ..cpu }
        } else if is_sp(reg) {
            Armv7m { sp: set_sp(cpu.sp, cpu.mode, cpu.control, val), ..cpu }
        } else if is_lr(reg) {
            Armv7m { lr: val, ..cpu }
        } else if is_pc(reg) {
            Armv7m { pc: val, ..cpu }
        } else if is_control(reg) {
            Armv7m { control: set_control(cpu.control, cpu.mode, val), ..cpu }
        } else if is_psr(reg) {
            Armv7m { psr: val, ..cpu }
        } else {
            cpu
        }
    }

    fn pop_spr_get_mem_addr_and_incr_precondition(cpu: Armv7m) -> bool {
        is_valid_ram_addr(get_sp(cpu.sp, cpu.mode, cpu.control))
        &&
        is_valid_ram_addr(bv_add(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x4)))
    }

    fn pop_spr_update_reg_precondition(cpu: Armv7m, reg: int, val: BV32) -> bool {
        (is_sp(reg) || is_psp(reg)) => is_valid_ram_addr(val)
    }

    fn pop_spr_get_mem_addr_and_incr_ret_val(cpu: Armv7m) -> BV32 {
        get_mem_addr(get_sp(cpu.sp, cpu.mode, cpu.control), cpu.mem)
    }

    fn cpu_post_pop_spr_get_mem_addr_and_incr(cpu: Armv7m) -> Armv7m {
        set_spr(sp(), cpu, bv_add(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x4)))
    }

    fn cpu_post_pop_spr_update_reg(cpu: Armv7m, reg: int, val: BV32) -> Armv7m {
        set_spr(reg, cpu, val)
    }

    fn pop_spr_precondition(cpu: Armv7m, reg: int) -> bool {
        pop_spr_get_mem_addr_and_incr_precondition(cpu)
        &&
        pop_spr_update_reg_precondition(cpu, reg, pop_spr_get_mem_addr_and_incr_ret_val(cpu))
    }

    fn cpu_post_pop_spr(cpu: Armv7m, reg: int) -> Armv7m {
        cpu_post_pop_spr_update_reg(
            cpu_post_pop_spr_get_mem_addr_and_incr(cpu),
            reg,
            pop_spr_get_mem_addr_and_incr_ret_val(cpu)
        )
    }

    fn mem_post_stmdb_wback(
        cpu: Armv7m,
        rd: int,
        r1: int,
        r2: int,
        r3: int,
    ) -> Map<BV32, BV32> {
        map_set(
            map_set(
                map_set(
                    cpu.mem,
                    bv_sub(get_special_reg(rd, cpu), bv32(0xc)),
                    get_gpr(r1, cpu)
                ),
                bv_sub(get_special_reg(rd, cpu), bv32(0x8)),
                get_gpr(r2, cpu)
            ),
            bv_sub(get_special_reg(rd, cpu), bv32(0x4)),
            get_gpr(r3, cpu)
        )
    }

    fn cpu_post_pop(cpu: Armv7m, r1: int, r2: int, r3: int, r4: int, r5: int) -> Armv7m {
        set_spr(
            sp(),
            set_spr(
                r5,
                Armv7m {
                    general_regs: gprs_post_pop(
                        cpu,
                        get_sp(cpu.sp, cpu.mode, cpu.control),
                        r1,
                        r2,
                        r3,
                        r4
                    ),
                    ..cpu
                },
                get_mem_addr(
                    bv_add(
                        get_sp(cpu.sp, cpu.mode, cpu.control),
                        bv32(0x10)
                    ),
                    cpu.mem
                )
            ),
            bv_add(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x14))
        )
    }

    fn cpu_post_stmdb_wback(cpu: Armv7m, rd: int, r1: int, r2: int, r3: int) -> Armv7m {
            Armv7m {
                mem: mem_post_stmdb_wback(cpu, rd, r1, r2, r3),
                ..set_spr(
                    rd,
                    cpu,
                    bv_sub(get_special_reg(rd, cpu), bv32(0xc))
                )
            }
    }

    fn generic_isr_bit_loc(old_cpu: Armv7m) -> BV32 {
        bv_and(bv_sub(get_special_reg(ipsr(), old_cpu), bv32(16)), bv32(31))
    }

    fn generic_isr_r0(old_cpu: Armv7m) -> BV32 {
        left_shift(
            bv32(1),
            generic_isr_bit_loc(old_cpu)
        )
    }

    fn generic_isr_r2(old_cpu: Armv7m) -> BV32 {
        right_shift(bv_sub(get_special_reg(ipsr(), old_cpu), bv32(16)), bv32(5))
    }

    fn generic_isr_offset(old_cpu: Armv7m) -> BV32 {
        left_shift(generic_isr_r2(old_cpu), bv32(2))
    }

    fn cpu_post_generic_isr(old_cpu: Armv7m) -> Armv7m {
        Armv7m {
            mem: update_mem(
                 bv_add(bv32(0xe000_e200), generic_isr_offset(old_cpu)),
                 update_mem(
                     bv_add(bv32(0xe000_e180), generic_isr_offset(old_cpu)),
                     old_cpu.mem,
                     generic_isr_r0(old_cpu)
                ),
                generic_isr_r0(old_cpu)
            ),
            general_regs: map_set(
                map_set(
                    map_set(
                        old_cpu.general_regs,
                        r0(),
                        generic_isr_r0(old_cpu)
                    ),
                    r2(),
                    generic_isr_r2(old_cpu)
                ),
                r3(),
                bv32(0xe000_e200)
            ),
            control: Control { npriv: false, ..old_cpu.control },
            lr: bv32(0xFFFF_FFF9),
            ..old_cpu
        }
    }

    fn cpu_post_svc_to_kernel_isr(old_cpu: Armv7m) -> Armv7m {
        Armv7m {
            mem: map_set(old_cpu.mem, bv32(0x8000_0000), bv32(1)),
            general_regs: map_set(map_set(old_cpu.general_regs, r0(), bv32(0)), r1(), bv32(1)),
            control: Control { npriv: false, ..old_cpu.control },
            lr: bv32(0xFFFF_FFF9),
            ..old_cpu
        }
    }

    fn cpu_post_svc_to_app_isr(old_cpu: Armv7m) -> Armv7m {
        Armv7m {
            general_regs: map_set(old_cpu.general_regs, r0(), bv32(1)),
            control: Control { npriv: true, ..old_cpu.control },
            lr: bv32(0xFFFF_FFFD),
            ..old_cpu
        }
    }

    fn svc_isr_ret_val(old_cpu: Armv7m) -> BV32 {
        if get_special_reg(lr(), old_cpu) == bv32(0xFFFF_FFF9) {
            bv32(0xFFFF_FFFD)
        } else {
           bv32(0xFFFF_FFF9)
        }
    }

    fn cpu_post_svc_isr(old_cpu: Armv7m) -> Armv7m {
        if get_special_reg(lr(), old_cpu) == bv32(0xFFFF_FFF9) {
            cpu_post_svc_to_app_isr(old_cpu)
        } else {
            cpu_post_svc_to_kernel_isr(old_cpu)
        }
    }

    fn cpu_post_sys_tick_isr(old_cpu: Armv7m) -> Armv7m {
        Armv7m {
            general_regs: map_set(old_cpu.general_regs, r0(), bv32(0)),
            control: Control { npriv: false, ..old_cpu.control },
            lr: bv32(0xFFFF_FFF9),
            ..old_cpu
        }
    }

    fn cpu_post_run_isr(cpu: Armv7m, exception_num: int) -> Armv7m {
        if exception_num == 11 {
            cpu_post_svc_isr(cpu)
        } else if exception_num == 15 {
            cpu_post_sys_tick_isr(cpu)
        } else if exception_num >= 16 {
            cpu_post_generic_isr(cpu)
        } else {
            // should not reach this
            cpu
        }
    }

    fn get_psr(cpu: Armv7m) ->  BV32 { get_special_reg(psr(), cpu) }

    fn mode_is_handler(mode: int) -> bool {
        mode == 0
    }

    fn handler_mode() -> int {
        0
    }

    fn thread_mode() -> int {
        1
    }

    fn is_ipsr(reg: int) -> bool {
        reg == 18
    }

    fn is_psr(reg: int) -> bool {
        reg == 17
    }

    fn is_control(reg: int) -> bool {
        reg == 16
    }

    fn is_pc(reg: int) -> bool {
        reg == 15
    }

    fn is_lr(reg: int) -> bool {
        reg == 14
    }

    fn is_sp(reg: int) -> bool {
        reg == 13
    }

    fn is_psp(reg: int) -> bool {
        reg == 19
    }

    fn r0() -> int {
        0
    }

    fn r1() -> int {
        1
    }

    fn r2() -> int {
        2
    }

    fn r3() -> int {
        3
    }

    fn r4() -> int {
        4
    }

    fn r5() -> int {
        5
    }

    fn r6() -> int {
        6
    }

    fn r7() -> int {
        7
    }

    fn r8() -> int {
        8
    }

    fn r9() -> int {
        9
    }

    fn r10() -> int {
        10
    }

    fn r11() -> int {
        11
    }

    fn r12() -> int {
        12
    }

    fn sp() -> int {
        13
    }

    fn lr() -> int {
        14
    }

    fn pc() -> int {
        15
    }

    fn control() -> int {
        16
    }

    fn psr() -> int {
        17
    }

    fn ipsr() -> int {
        18
    }

    fn psp() -> int {
        19
    }

    fn nth_bit_is_set(val: BV32, n: BV32) -> bool {
        // val & (1 << n)
        bv_and(val, left_shift(bv32(1), n)) != bv32(0)
    }

    fn right_shift(val: BV32, n: BV32) -> BV32 {
        // right shift
        bv_lshr(val, n)
    }

    fn left_shift(val: BV32, n: BV32) -> BV32 {
        // shift left
        bv_shl(val, n)
    }

    fn wrapping_add_u32(val1: int, val2: int) -> int {
        if val1 + val2 > U32_MAX {
            val1 + val2 % U32_MAX
        } else {
            val1 + val2
        }
    }

    // fn itstate_0_4_not_all_zero(cpu: Armv7m) -> bool {
    //     !(
    //         nth_bit(cpu.psr, 25) == 0
    //         &&
    //         nth_bit(cpu.psr, 26) == 0
    //         &&
    //         nth_bit(cpu.psr, 10) == 0
    //         &&
    //         nth_bit(cpu.psr, 11) == 0
    //     )
    // }

    // fn movs_flag_updates(cpu: Armv7m) -> bool {
    //     if !itstate_0_4_not_all_zero(cpu) {
    //         // flag updates
    //         // n flag and z flag are unset and set
    //         nth_bit_is_unset(cpu.psr, 31) && nth_bit_is_set(cpu.psr, 30)
    //     } else {
    //             // no flag updates
    //             true
    //     }
    // }

    // fn right_shift_immediate_computation(reg: GPR, old_cpu: Armv7m, shift: BV32) -> BV32 {
    //     if (
    //         get_gpr(reg, old_cpu) > 0
    //         &&
    //         lshr(get_gpr(reg, old_cpu), shift) == get_gpr(reg, old_cpu)
    //     ) {
    //         0
    //     } else {
    //         lshr(get_gpr(reg, old_cpu), shift)
    //     }
    // }

    // fn right_shift_immediate_carry_flag(reg: GPR, old_cpu: Armv7m, shift: BV32) -> BV32 {
    //     if (
    //         get_gpr(reg, old_cpu) > 0
    //         &&
    //         lshr(get_gpr(reg, old_cpu), shift) == get_gpr(reg, old_cpu)
    //     ) {
    //         nth_bit(get_gpr(reg, old_cpu), 31)
    //     } else {
    //         if shift >= 1 && shift <= 31 {
    //             nth_bit(get_gpr(reg, old_cpu), shift - 1)
    //         } else {
    //             0
    //         }
    //     }
    // }

    // fn lsrs_imm_flag_updates(reg: GPR, old_cpu: Armv7m, new_cpu: Armv7m, shift: BV32) -> bool {
    //     if !itstate_0_4_not_all_zero(old_cpu) {
    //         // flag updates
    //         // n flag and z flag are unset and set and carry is computed
    //         nth_bit_is_unset(new_cpu.psr, 31)
    //         &&
    //         nth_bit_is_set(new_cpu.psr, 30)
    //         &&
    //         if right_shift_immediate_carry_flag(reg, old_cpu, shift) == 1 {
    //             nth_bit_is_set(new_cpu.psr, 29)
    //         } else if right_shift_immediate_carry_flag(reg, old_cpu, shift) == 0 {
    //             nth_bit_is_unset(new_cpu.psr, 29)
    //         } else {
    //             // that's wrong :)
    //             false
    //         }
    //     } else {
    //             // no flag updates
    //             true
    //     }
    // }

    // fn left_shift_reg_computation(reg: GPR, old_cpu: Armv7m, shift: BV32) -> BV32 {
    //     if (
    //         get_gpr(reg, old_cpu) > 0
    //         &&
    //         lshl(get_gpr(reg, old_cpu), shift) == get_gpr(reg, old_cpu)
    //     ) {
    //         0
    //     } else {
    //         lshl(get_gpr(reg, old_cpu), shift)
    //     }
    // }

    // fn left_shift_reg_carry_flag(reg: GPR, old_cpu: Armv7m, shift: BV32) -> BV32 {
    //     if (
    //         get_gpr(reg, old_cpu) > 0
    //         &&
    //         lshl(get_gpr(reg, old_cpu), shift) == get_gpr(reg, old_cpu)
    //     ) {
    //         nth_bit(get_gpr(reg, old_cpu), 31)
    //     } else {
    //         if shift >= 1 && shift <= 31 {
    //             nth_bit(get_gpr(reg, old_cpu), shift - 1)
    //         } else {
    //             0
    //         }
    //     }
    // }

    // fn lslw_reg_flag_updates(reg: GPR, old_cpu: Armv7m, new_cpu: Armv7m, shift: BV32) -> bool {
    //     if !itstate_0_4_not_all_zero(old_cpu) {
    //         // flag updates
    //         // n flag and z flag are unset and set and carry is computed
    //         nth_bit_is_unset(new_cpu.psr, 31)
    //         &&
    //         nth_bit_is_set(new_cpu.psr, 30)
    //         &&
    //         if left_shift_reg_carry_flag(reg, old_cpu, shift) == 1 {
    //             nth_bit_is_set(new_cpu.psr, 29)
    //         } else if left_shift_reg_carry_flag(reg, old_cpu, shift) == 0 {
    //             nth_bit_is_unset(new_cpu.psr, 29)
    //         } else {
    //             // that's wrong :)
    //             false
    //         }
    //     } else {
    //             // no flag updates
    //             true
    //     }
    // }
}

#[flux_rs::extern_spec(std::u32)]
impl u32 {
    #[flux_rs::sig(fn (u32[@val1], u32[@val2]) -> u32[wrapping_add_u32(val1, val2)])]
    fn wrapping_add(self, other: u32) -> u32;
}
