use crate::armv7m::lang::{SpecialRegister, GPR};

use super::{Armv7m, CPUMode, Control};
use flux_rs::bitvec::BV32;

impl Armv7m {
    #[flux_rs::sig(
        fn (&mut Armv7m[@cpu]) -> (
            BV32[get_gpr(r0(), cpu)],
            BV32[get_gpr(r1(), cpu)],
            BV32[get_gpr(r2(), cpu)],
            BV32[get_gpr(r3(), cpu)],
            BV32[get_gpr(r12(), cpu)],
            BV32[get_special_reg(lr(), cpu)],
            BV32[get_special_reg(psr(), cpu)]
        )
    )]
    fn push_stack_get_gpr_vals(&mut self) -> (BV32, BV32, BV32, BV32, BV32, BV32, BV32) {
        let r0 = self.get_value_from_general_reg(&GPR::r0());
        let r1 = self.get_value_from_general_reg(&GPR::r1());
        let r2 = self.get_value_from_general_reg(&GPR::r2());
        let r3 = self.get_value_from_general_reg(&GPR::r3());
        let r12 = self.get_value_from_general_reg(&GPR::r12());
        let lr = self.get_value_from_special_reg(&SpecialRegister::lr());
        let psr = self.get_value_from_special_reg(&SpecialRegister::psr());
        (r0, r1, r2, r3, r12, lr, psr)
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu])
            requires is_valid_ram_addr(bv_sub(get_sp(cpu.sp, cpu.mode, cpu.control), bv32(0x20)))
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m { sp: sp_post_exception_entry(cpu), ..cpu } }
    )]
    fn push_stack_update_sp(&mut self) {
        let frame_size = BV32::from(0x20);
        let frame_ptr = self.get_value_from_special_reg(&SpecialRegister::sp());
        let frame_ptr = (frame_ptr - frame_size); // & !BV32::from(3);
        self.update_special_reg_with_b32(SpecialRegister::sp(), frame_ptr);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], BV32[@r0], BV32[@r1], BV32[@r2], BV32[@r3], BV32[@r12], BV32[@lr], BV32[@psr])
            requires push_stack_sp_precondition(get_sp(cpu.sp, cpu.mode, cpu.control))
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                mem: mem_post_push_stack_write_gpr_vals(
                     get_sp(cpu.sp, cpu.mode, cpu.control),
                     cpu,
                     r0,
                     r1,
                     r2,
                     r3,
                     r12,
                     lr,
                     psr
                ),
                ..cpu
            }}
    )]
    fn push_stack_write_gpr_vals(
        &mut self,
        r0: BV32,
        r1: BV32,
        r2: BV32,
        r3: BV32,
        r12: BV32,
        lr: BV32,
        psr: BV32,
    ) {
        let frame_ptr = self.get_value_from_special_reg(&SpecialRegister::sp());
        self.mem.write(frame_ptr, r0);
        self.mem.write(frame_ptr + BV32::from(0x4), r1);
        self.mem.write(frame_ptr + BV32::from(0x8), r2);
        self.mem.write(frame_ptr + BV32::from(0xC), r3);
        self.mem.write(frame_ptr + BV32::from(0x10), r12);
        self.mem.write(frame_ptr + BV32::from(0x14), lr);
        // putting a dummy value for ret addr
        self.mem.write(frame_ptr + BV32::from(0x18), BV32::from(0));
        // TODO: Real implementation skips bit 9
        self.mem.write(frame_ptr + BV32::from(0x1C), psr);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu])
            requires sp_can_handle_exception_entry(cpu)
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                    sp: sp_post_exception_entry(cpu),
                    mem: mem_post_exception_entry(get_sp(sp_post_exception_entry(cpu), cpu.mode, cpu.control), cpu),
                    ..cpu
                }
            }
    )]
    fn push_stack(&mut self) {
        let (r0, r1, r2, r3, r12, lr, psr) = self.push_stack_get_gpr_vals();
        self.push_stack_update_sp();
        self.push_stack_write_gpr_vals(r0, r1, r2, r3, r12, lr, psr);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], u8[@exception_num])
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                    mode: handler_mode(),
                    control: control_post_exception_entry(old_cpu),
                    psr: psr_post_exception_entry(old_cpu, exception_num),
                    lr: lr_post_exception_entry(old_cpu, old_cpu.control),
                    ..old_cpu
                }
            }
    )]
    fn exception_taken(&mut self, exception_number: u8) {
        // TODO: need to forget r0 - r3, r12 somehow

        // set exception num in psr
        self.psr = (self.psr & !BV32::from(0xff)) | BV32::from(exception_number as u32);

        // set link register
        self.lr = if self.mode_is_handler() {
            // From another exception
            BV32::from(0xFFFF_FFF1)
        } else if self.control.spsel {
            // from process stack
            BV32::from(0xFFFF_FFFD)
        } else {
            // from main stack
            BV32::from(0xFFFF_FFF9)
        };

        // stack = main
        self.mode = CPUMode::Handler;
        self.control.spsel = false;

        // TODO: There are other ops here but I don't think they
        // matter
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], u8[@exception_num])
            requires sp_can_handle_exception_entry(cpu)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_exception_entry(cpu, exception_num) }
    )]
    fn exception_entry(&mut self, exception_number: u8) {
        self.push_stack();
        self.exception_taken(exception_number);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], BV32[@return_exec]) -> BV32[get_sp_from_isr_ret(cpu.sp, return_exec)]
            requires is_valid_ram_addr(bv_add(get_sp_from_isr_ret(cpu.sp, return_exec), bv32(0x20)))
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                    mode: thread_mode(),
                    control: Control {
                        spsel: return_exec != bv32(0xFFFF_FFF9),
                        ..cpu.control
                    },
                    sp: sp_post_exception_exit(cpu.sp, return_exec),
                    ..cpu
                }
            }
    )]
    fn exception_exit_get_fp_update_sp(&mut self, return_exec: BV32) -> BV32 {
        if return_exec == BV32::from(0xFFFF_FFF9) {
            self.control.spsel = false;
            self.mode = CPUMode::Thread;
            let fp = self.sp.sp_main;
            self.sp.sp_main = fp + BV32::from(0x20);
            fp
        } else {
            self.control.spsel = true;
            self.mode = CPUMode::Thread;
            let fp = self.sp.sp_process;
            self.sp.sp_process = fp + BV32::from(0x20);
            fp
        }
    }

    #[flux_rs::sig(
        fn (
            &Armv7m[@cpu],
            BV32[@fp]
        ) -> (
            BV32[get_mem_addr(fp, cpu.mem)],
            BV32[get_mem_addr(bv_add(fp, bv32(0x4)), cpu.mem)],
            BV32[get_mem_addr(bv_add(fp, bv32(0x8)), cpu.mem)],
            BV32[get_mem_addr(bv_add(fp, bv32(0xC)), cpu.mem)],
            BV32[get_mem_addr(bv_add(fp, bv32(0x10)), cpu.mem)],
            BV32[get_mem_addr(bv_add(fp, bv32(0x14)), cpu.mem)],
            BV32[get_mem_addr(bv_add(fp, bv32(0x1C)), cpu.mem)],
        )
        requires sp_can_handle_exception_exit(fp)
    )]
    fn exception_exit_read_regs(
        &self,
        frame_ptr: BV32,
    ) -> (BV32, BV32, BV32, BV32, BV32, BV32, BV32) {
        let r0 = self.mem.read(frame_ptr);
        let r1 = self.mem.read(frame_ptr + BV32::from(0x4));
        let r2 = self.mem.read(frame_ptr + BV32::from(0x8));
        let r3 = self.mem.read(frame_ptr + BV32::from(0xC));
        let r12 = self.mem.read(frame_ptr + BV32::from(0x10));
        let lr = self.mem.read(frame_ptr + BV32::from(0x14));
        let psr = self.mem.read(frame_ptr + BV32::from(0x1C));
        (r0, r1, r2, r3, r12, lr, psr)
    }

    #[flux_rs::sig(
        fn (
            self: &strg Armv7m[@cpu],
            BV32[@r0],
            BV32[@r1],
            BV32[@r2],
            BV32[@r3],
            BV32[@r12],
            BV32[@lr],
            BV32[@psr],
        )
        ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                general_regs: gprs_post_exception_exit_write_regs(cpu, r0, r1, r2, r3, r12),
                lr: lr,
                psr: psr,
                ..cpu
            }
        }
    )]
    fn exception_exit_write_regs(
        &mut self,
        r0: BV32,
        r1: BV32,
        r2: BV32,
        r3: BV32,
        r12: BV32,
        lr: BV32,
        psr: BV32,
    ) {
        self.update_general_reg_with_b32(GPR::r0(), r0);
        self.update_general_reg_with_b32(GPR::r1(), r1);
        self.update_general_reg_with_b32(GPR::r2(), r2);
        self.update_general_reg_with_b32(GPR::r3(), r3);
        self.update_general_reg_with_b32(GPR::r12(), r12);
        self.update_special_reg_with_b32(SpecialRegister::lr(), lr);
        self.update_special_reg_with_b32(SpecialRegister::psr(), psr);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], BV32[@return_exec])
            requires sp_can_handle_exception_exit(get_sp_from_isr_ret(cpu.sp, return_exec))
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_exception_exit(cpu, return_exec) }
    )]
    fn exception_exit(&mut self, return_exec: BV32) {
        let frame_ptr = self.exception_exit_get_fp_update_sp(return_exec);
        let (r0, r1, r2, r3, r12, lr, psr) = self.exception_exit_read_regs(frame_ptr);
        self.exception_exit_write_regs(r0, r1, r2, r3, r12, lr, psr);
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], u8[@exception_num]) -> BV32[get_bx_from_exception_num(exception_num, cpu.lr)]
            requires mode_is_handler(cpu.mode) && get_special_reg(ipsr(), cpu) == bv32(exception_num)
            // (exception_num == 11 || exception_num == 15 || (exception_num >= 16 => bv_uge(get_special_reg(ipsr(), cpu), bv32(16))))
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_run_isr(cpu, exception_num)  }
    )]
    fn run_isr(&mut self, exception_number: u8) -> BV32 {
        match exception_number {
            11 => self.svc_isr(),
            15 => self.sys_tick_isr(),
            16..=255 => self.generic_isr(),
            _ => panic!("Unhandled"),
        }
    }

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], u8[@exception_num])
            requires
                (exception_num == 11 || exception_num >= 15)
                &&
                // Stack Pointer is valid and can grow downwards 20 bytes
                sp_can_handle_exception_entry(cpu)
                &&
                // and Stack Pointer used on exit is valid and can grow upwards 20 bytes
                sp_can_handle_preempt_exception_exit(cpu, exception_num)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_preempt(cpu, exception_num) }
    )]
    pub fn preempt(&mut self, exception_number: u8) {
        // stack
        self.exception_entry(exception_number);
        // TODO: get rid of this assume - it should hold automagically
        assume(self.psr & BV32::from(0xff) == BV32::from(exception_number as u32));
        // call isr
        let ret_value = self.run_isr(exception_number);
        // unstack
        self.exception_exit(ret_value);
    }
}
#[flux_rs::sig(fn(b:bool) ensures b)]
pub const fn assume(b: bool) {
    if !b {
        panic!("assume fails")
    }
}
