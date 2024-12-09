#![allow(unused)]

pub mod armv7m;
mod flux_support;

mod arm_test {
    use crate::{
        armv7m::{
            cpu::{Armv7m, SP},
            lang::{SpecialRegister, GPR},
            mem::{flux_defs, Memory},
        },
        flux_support::bv32::BV32,
    };

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) 
        ensures self: Armv7m[{
            general_regs: set_gpr(
                r0(), 
                old_cpu,
                bv32(to_int(get_special_reg(ipsr(), old_cpu)) % 32)
            ),
            ..old_cpu
        }] 
    )]
    fn simple_mod(armv7m: &mut Armv7m) {
        // r0 = ipsr
        armv7m.mrs(GPR::R0, SpecialRegister::IPSR);
        // r0 = r0 & 31
        armv7m.and_imm(GPR::R0, BV32::from(31));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: get_gpr(r0(), new_cpu) ==  right_shift(bv32(1 % 32), bv32(5)) })]
    fn simple_shift(armv7m: &mut Armv7m) {
        // r0 = 1
        armv7m.movs_imm(GPR::R0, BV32::from(1));
        // r0 = r0 & 31
        armv7m.and_imm(GPR::R0, BV32::from(31));
        // r0 = r0 >> 5
        armv7m.lsrs_imm(GPR::R0, GPR::R0, BV32::from(5));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) 
        ensures self: Armv7m { new_cpu: get_mem_addr(0xE000_E184, new_cpu.mem) == bv32(1) }
    )]
    fn lsl_store_nvic(armv7m: &mut Armv7m) {
        // 0xE000_E180 + 1 * 4 = 1
        armv7m.pseudo_ldr(GPR::R3, BV32::from(0xE000_E180));
        armv7m.movw_imm(GPR::R0, BV32::from(1));
        armv7m.movw_imm(GPR::R1, BV32::from(1));
        armv7m.strw_lsl_reg(GPR::R0, GPR::R3, GPR::R1, BV32::from(2));
    }

    #[flux_rs::should_fail]
    // Sanity check that we the postcondition here specifies the wrong
    // register (should be 0xE000_E184)
    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) 
        ensures self: Armv7m { new_cpu: get_mem_addr(0xE000_E180, new_cpu.mem) == bv32(1)  }
    )]
    fn lsl_store_nvic_wrong(armv7m: &mut Armv7m) {
        armv7m.pseudo_ldr(GPR::R3, BV32::from(0xE000_E180));
        armv7m.movw_imm(GPR::R0, BV32::from(1));
        armv7m.movw_imm(GPR::R1, BV32::from(1));
        armv7m.strw_lsl_reg(GPR::R0, GPR::R3, GPR::R1, BV32::from(2));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) 
        ensures self: Armv7m[{
            general_regs: set_gpr(r0(), old_cpu, bv32(0)), ..old_cpu
        }]
    )]
    fn movw_r0(armv7m: &mut Armv7m) {
        armv7m.movw_imm(GPR::R0, BV32::from(0));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) 
        ensures self: Armv7m[{
            general_regs: set_gpr(r1(), old_cpu, bv32(1)), ..old_cpu
        }] 
    )]
    fn movw_r1(armv7m: &mut Armv7m) {
        armv7m.movw_imm(GPR::R1, BV32::from(1));
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
        get_gpr(r0(), new_cpu) == bv32(0)
        &&
        get_gpr(r1(), new_cpu) == bv32(1)
    })]
    fn two_movs_by_call(armv7m: &mut Armv7m) {
        movw_r0(armv7m);
        movw_r1(armv7m);
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
        get_gpr(r0(), new_cpu) == bv32(0)
        &&
        get_gpr(r1(), new_cpu) == bv32(1)
    })]
    fn two_movs(armv7m: &mut Armv7m) {
        armv7m.movw_imm(GPR::R0, BV32::from(0));
        armv7m.movw_imm(GPR::R1, BV32::from(1));
    }



    // process havocs everything except for the fact the sp can take an update

    flux_rs::defs! {
        fn sp_main(sp: SP) -> BV32 {
            sp.sp_main 
        }

        fn sp_process(sp: SP) -> BV32 {
            sp.sp_process
        }
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) 
           // process MUST be running in mode thread unprivileged
           requires mode_is_thread_unprivileged(old_cpu.mode, old_cpu.control)
           ensures self: Armv7m { new_cpu: 
            sp_main(new_cpu.sp) == sp_main(old_cpu.sp) 
            &&
            mode_is_thread_unprivileged(new_cpu.mode, new_cpu.control)
            && 
            sp_process(new_cpu.sp) == bv32(0x8FFF_FFDD)
            &&
            kernel_register_stack_frame_preserved(int(sp_main(new_cpu.sp)), old_cpu, new_cpu)
            &&
            sp_can_handle_exception_entry(new_cpu)
        }
    )]
    fn process(armv7m: &mut Armv7m) {}

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (self: &strg Armv7m[@cpu]) ensures self: Armv7m { new_cpu: sp_can_handle_exception_entry(new_cpu) })]
    fn prepare_for_exception(armv7m: &mut Armv7m)  {}

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], u8[@exception_num]) 
           requires 
               mode_is_thread_privileged(old_cpu.mode, old_cpu.control) 
               && 
               sp_can_handle_exception_entry(old_cpu)
               &&
               // Here: 
               //   1. sp main will grow downwards by 0x20
               //   2. sp_process will grow upwards by 0x20 
               //   3. sp_process will grow downwards by 0x20
               //   4. sp_main will grow upwards by 0x20
               //
               (
                   // sp main needs a buffer of 0x20 bytes on sp_process to grow downwards
                   sp_main(old_cpu.sp) == bv32(0x6000_0020)
                   &&
                   sp_process(old_cpu.sp) == bv32(0x8FFF_FFFF)
                   // sp_main(old_cpu.sp) > bv_add(sp_process(old_cpu.sp), bv32(0x20))
                   // ||
                   // or sp process needs a buffer of 0x20 bytes on sp process to grow upwards
                   // sp_process(old_cpu.sp) < bv_sub(sp_main(old_cpu.sp), bv32(0x20))
               )
               && sp_can_handle_exception_exit(old_cpu, exception_num)
           ensures self: Armv7m { new_cpu:
               sp_main(new_cpu.sp) == sp_main(old_cpu.sp) && get_gpr(r0(), new_cpu) == bv32(10) 
            }
    )]
    fn full_circle(armv7m: &mut Armv7m, exception_number: u8) {
        // executes some kernel logic
        armv7m.movs_imm(GPR::r0(), BV32::from(10));
        armv7m.preempt(11);
        // process that havocs all state except the main sp and the fact it's in thread mode unprivileged
        process(armv7m);
        // fake sys call
        armv7m.preempt(exception_number);
        // end up back here
        // no more instructions for now
    }

    #[flux_rs::trusted] 
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu]) 
           requires mode_is_thread_privileged(old_cpu.mode, old_cpu.control) && sp_can_handle_exception_entry(old_cpu)
           ensures self: Armv7m { new_cpu: sp_main(new_cpu.sp) == sp_main(old_cpu.sp) && get_gpr(r0(), new_cpu) == bv32(10) }
    )]
    fn kernel_preempt(armv7m: &mut Armv7m) {
        // executes some kernel logic
        armv7m.movs_imm(GPR::r0(), BV32::from(10));
        // interrupt that sends us back to kernel
        armv7m.preempt(20);
        // end up back here
        // no more instructions for now
    }
}
