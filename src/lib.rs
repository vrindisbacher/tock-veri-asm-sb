#![allow(unused)]

use armv7m::{cpu::Armv7m, lang::{SpecialRegister, GPR}, cpu::SP};
use flux_support::bv32::BV32;

pub mod armv7m;
mod flux_support;

// Part 1: 
//   0:   b5f0            push    {r4, r5, r6, r7, lr}
//   2:   af03            add     r7, sp, #12
//   4:   e92d 0d00       stmdb   sp!, {r8, sl, fp}
//   8:   4632            mov     r2, r6
//   a:   463b            mov     r3, r7
//   c:   46cc            mov     ip, r9
//   e:   f380 8809       msr     PSP, r0
//  12:   e891 0ff0       ldmia.w r1, {r4, r5, r6, r7, r8, r9, sl, fp}
//  16:   dfff            svc     255     @ 0xff
#[flux_rs::trusted]
#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu])
        // requires that r1 is a valid read addr and r0 is a valid write addr
        requires 
            mode_is_thread_privileged(old_cpu.mode, old_cpu.control)
            &&
            is_valid_ram_addr(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control))
            &&
            is_valid_ram_addr(bv_sub(get_sp(old_cpu.sp, old_cpu.mode, old_cpu.control), bv32(0x24)))
            &&
            is_valid_ram_addr(get_gpr(r1(), old_cpu))
            &&
            is_valid_ram_addr(bv_add(get_gpr(r1(), old_cpu), bv32(0x1c)))
        ensures self: Armv7m { new_cpu: new_cpu == cpu_post_switch_to_user_pt1(old_cpu) }
)]
pub fn switch_to_user_part1(armv7m: &mut Armv7m) {
    // push onto stack
    armv7m.push_gpr(GPR::r4());  // sp - 0x4
    armv7m.push_gpr(GPR::r5());  // sp - 0x8
    armv7m.push_gpr(GPR::r6());  // sp - 0xc
    armv7m.push_gpr(GPR::r7());  // sp - 0x10
    // NOTE: This is because lr holds the value of the next instruction to
    // execute once switch_to_user returns
    armv7m.push_spr(SpecialRegister::lr()); // sp - 0x14

    // add imm - WTF is this even doing here
    // armv7m.add_imm(GPR::r7(), SpecialRegister::sp(), BV32::from(12)); // sp - 0x18 + 0xc

    // some stmdb stuff
    armv7m.stmdb_no_wback(SpecialRegister::sp(), GPR::r8());
    // sl - r10
    armv7m.stmdb_no_wback(SpecialRegister::sp(), GPR::r10());
    // fp - r11
    armv7m.stmdb_no_wback(SpecialRegister::sp(), GPR::r11());
    // mov
    armv7m.mov(GPR::r2(), GPR::r6()); // not sure about this - already saved?  
    armv7m.mov(GPR::r3(), GPR::r7()); // // not sure about this - already saved?  
    // note ip is intraprocedure scratch register - r12
    armv7m.mov(GPR::r12(), GPR::r9());

    // msr
    armv7m.msr(SpecialRegister::psp(), GPR::r0());

    // ldmia
    armv7m.ldmia_w(GPR::r1(), GPR::r4(), GPR::r5(), GPR::r6(), GPR::r7(), GPR::r8(), GPR::r9(), GPR::r10(), GPR::r11()); 

    // svc
    armv7m.svc(0xff);
}

// Part 2:
//  18:   e881 0ff0       stmia.w r1, {r4, r5, r6, r7, r8, r9, sl, fp}
//  1c:   f3ef 8009       mrs     r0, PSP
//  20:   4616            mov     r6, r2
//  22:   461f            mov     r7, r3
//  24:   46e1            mov     r9, ip
//  26:   e8bd 0d00       ldmia.w sp!, {r8, sl, fp}
//  2a:   bdf0            pop     {r4, r5, r6, r7, pc}
#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu])
        requires 
            mode_is_thread_privileged(old_cpu.mode, old_cpu.control)
            &&
            is_valid_ram_addr(get_gpr(r1(), old_cpu))
            && 
            is_valid_ram_addr(bv_sub(get_gpr(r1(), old_cpu), bv32(0x20)))
        ensures self: Armv7m { new_cpu: 
            new_cpu.mem == mem_post_switch_to_user_pt2(old_cpu)
            && 
            get_gpr(r0(), new_cpu) == sp_process(old_cpu.sp)
            &&
            get_gpr(r6(), new_cpu) == get_gpr(r2(), old_cpu)
            &&
            get_gpr(r7(), new_cpu) == get_gpr(r3(), old_cpu)
            &&
            get_gpr(r9(), new_cpu) == get_gpr(r12(), old_cpu)
        }
        // { new_cpu: new_cpu == Armv7m { }
    // }
)]
#[flux_rs::trusted]
pub fn switch_to_user_part2(armv7m: &mut Armv7m) {
    armv7m.stmia_w(GPR::r1(), GPR::r4(), GPR::r5(), GPR::r6(), GPR::r7(), GPR::r8(), GPR::r9(), GPR::r10(), GPR::r11()); 
    armv7m.mrs(GPR::r0(), SpecialRegister::psp());
    armv7m.mov(GPR::r6(), GPR::r2()); // this is seemingly useless?
    armv7m.mov(GPR::r7(), GPR::r3()); // this is also useless?
    armv7m.mov(GPR::r9(), GPR::r12());
    armv7m.ldmia_w_special(SpecialRegister::Sp, GPR::r8(), GPR::r10(), GPR::r11()); 
    armv7m.pop_gpr(GPR::r4());
    armv7m.pop_gpr(GPR::r5());
    armv7m.pop_gpr(GPR::r6());
    armv7m.pop_gpr(GPR::r7());
    // NOTE: This is because we previously pushed lr (which contains the return address
    // for the next instruction after switch_to_user finishes)
    // and we want to branch to it
    armv7m.pop_spr(SpecialRegister::pc())
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
        kernel_register_stack_frame_preserved(sp_main(new_cpu.sp), old_cpu, new_cpu)
        &&
        sp_can_handle_exception_entry(new_cpu)
    }
)]
fn process(armv7m: &mut Armv7m) {}


#[flux_rs::trusted]
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
           // && sp_can_handle_exception_exit(old_cpu, 11)
       ensures self: Armv7m 
)]
pub fn tock_control_flow(armv7m: &mut Armv7m, exception_num: u8) {
    // context switch asm
    switch_to_user_part1(armv7m);
    // run a process
    process(armv7m);
    // preempt the process with an arbitrary exception number
    armv7m.preempt(exception_num);
    // run the rest of the context switch asm
    switch_to_user_part2(armv7m);
}

mod arm_test {
    use crate::{
        armv7m::{
            cpu::{Armv7m, SP},
            lang::{SpecialRegister, GPR},
            mem::{flux_defs, Memory},
        },
        flux_support::bv32::BV32,
    };

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
            kernel_register_stack_frame_preserved(sp_main(new_cpu.sp), old_cpu, new_cpu)
            &&
            sp_can_handle_exception_entry(new_cpu)
        }
    )]
    fn process(armv7m: &mut Armv7m) {}

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
               && sp_can_handle_preempt_exception_exit(old_cpu, exception_num)
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
}
