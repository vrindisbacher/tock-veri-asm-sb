use crate::{armv7m::lang::{SpecialRegister, GPR}, flux_support::bv32::BV32};

use super::{Armv7m, CPUMode, Control};

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
    fn push_stack_write_gpr_vals(&mut self, r0: BV32, r1: BV32, r2: BV32, r3: BV32, r12: BV32, lr: BV32, psr: BV32) {
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
        // self.exception_taken(exception_number);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], BV32[@return_exec])
            requires 
                is_valid_ram_addr(get_sp_from_isr_ret(cpu.sp, return_exec))
                &&
                is_valid_ram_addr(bv_add(get_sp_from_isr_ret(cpu.sp, return_exec), bv32(0x20)))
            ensures self: Armv7m { new_cpu: new_cpu == Armv7m {
                    mode: thread_mode(),
                    control: Control { spsel: return_exec != bv32(0xFFFF_FFF9), ..cpu.control },
                    general_regs: gprs_post_exception_exit(get_sp_from_isr_ret(cpu.sp, return_exec), cpu),
                    lr: get_mem_addr(bv_add(get_sp_from_isr_ret(cpu.sp, return_exec), bv32( 0x14)), cpu.mem),
                    psr: get_mem_addr(bv_add(get_sp_from_isr_ret(cpu.sp, return_exec), bv32(0x1C)), cpu.mem),
                    sp: sp_post_exception_exit(cpu.sp, return_exec),
                    ..cpu
                }
                && is_valid_ram_addr(get_sp(new_cpu.sp, new_cpu.mode, new_cpu.control))
            }
    )]
    fn exception_exit(&mut self, return_exec: BV32) {
        let frame_ptr = if return_exec == BV32::from(0xFFFF_FFF9) {
            self.control.spsel = false;
            self.mode = CPUMode::Thread;
            let fp = self.sp.sp_main;
            self.sp.sp_main = self.sp.sp_main + BV32::from(0x20);
            fp
        } else {
            self.control.spsel = true;
            self.mode = CPUMode::Thread;
            let fp = self.sp.sp_process;
            self.sp.sp_process = self.sp.sp_process + BV32::from(0x20);
            fp
        };
        // R[0] = MemA[frameptr,4];
        // R[1] = MemA[frameptr+0x4,4];
        // R[2] = MemA[frameptr+0x8,4];
        // R[3] = MemA[frameptr+0xC,4];
        // R[12] = MemA[frameptr+0x10,4];
        // LR = MemA[frameptr+0x14,4];
        // BranchTo(MemA[frameptr+0x18,4]); // UNPREDICTABLE if the new PC not halfword aligned
        // psr = MemA[frameptr+0x1C,4];
        let r0 = self.mem.read(frame_ptr);
        self.update_general_reg_with_b32(GPR::r0(), r0);
        let r1 = self.mem.read(frame_ptr + BV32::from(0x4));
        self.update_general_reg_with_b32(GPR::r1(), r1);
        let r2 =  self.mem.read(frame_ptr + BV32::from(0x8));
        self.update_general_reg_with_b32(GPR::r2(), r2);
        let r3 = self.mem.read(frame_ptr + BV32::from(0xC));
        self.update_general_reg_with_b32(GPR::r3(), r3);
        let r12 = self.mem.read(frame_ptr + BV32::from(0x10));
        self.update_general_reg_with_b32(GPR::r12(), r12);
        let lr = self.mem.read(frame_ptr + BV32::from(0x14));
        self.update_special_reg_with_b32(SpecialRegister::lr(), lr);
        let psr = self.mem.read(frame_ptr + BV32::from(0x1C));
        self.update_special_reg_with_b32(SpecialRegister::psr(), psr);
    }

    #[flux_rs::sig(
        fn (&Armv7m[@cpu], u8[@exception_num]) -> BV32[get_bx_from_exception_num(exception_num, cpu.lr)]
    )]
    fn run_isr(&self, exception_number: u8) -> BV32 {
        if exception_number == 11 && self.lr == BV32::from(0xFFFF_FFF9) {
            BV32::from(0xFFFF_FFFD)
        } else {
            BV32::from(0xFFFF_FFF9)
        }
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@cpu], u8[@exception_num]) 
            requires 
                // Stack Pointer is valid and can grow downwards 20 bytes
                sp_can_handle_exception_entry(cpu)
                &&
                // and Stack Pointer used on exit is valid and can grow upwards 20 bytes
                sp_can_handle_exception_exit(cpu, exception_num)
            ensures self: Armv7m { new_cpu: new_cpu == cpu_post_exception_exit(cpu, exception_num) }
    )]
    pub fn preempt(
        &mut self,
        exception_number: u8,
    ) {
        // stack
        self.exception_entry(exception_number);
        // call isr
        let ret_value = self.run_isr(exception_number);
        // unstack
        self.exception_exit(ret_value);
    } 
}
