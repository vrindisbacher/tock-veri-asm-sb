#![allow(unused)]

use armv7m::{
    cpu::Armv7m,
    cpu::SP,
    lang::{SpecialRegister, GPR},
};
use flux_rs::bitvec::BV32;

mod armv7m;
mod flux_support;

#[flux_rs::sig(fn (bool[true]))]
pub fn assert(b: bool) {}

#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu])
        requires switch_to_user_pt1_save_clobbers_precondition(old_cpu)
        ensures self: Armv7m { new_cpu: new_cpu == cpu_post_switch_to_user_pt1_save_clobbers(old_cpu) }
)]
fn switch_to_user_part1_save_clobbers(armv7m: &mut Armv7m) {
    // IMPORTANT NOTE - this cannot overwrite the address that r0 is pointing
    // to or we overwrite the saved process registers

    // push
    // NOTE: pushing lr is because lr holds the value of the next instruction to
    // execute once switch_to_user returns
    armv7m.push(
        GPR::r4(),
        GPR::r5(),
        GPR::r6(),
        GPR::r7(),
        SpecialRegister::lr(),
    );

    // add imm - WTF is this even doing here
    // armv7m.add_imm(GPR::r7(), SpecialRegister::sp(), BV32::from(12)); // sp - 0x18 + 0xc

    // stmdb
    armv7m.stmdb_wback(SpecialRegister::sp(), GPR::r8(), GPR::r10(), GPR::r11());
}

#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu])
        requires switch_to_user_pt1_reg_restores_precondition(old_cpu)
        ensures self: Armv7m { new_cpu: new_cpu == cpu_post_switch_to_user_pt1_reg_restores(old_cpu) }
)]
pub fn switch_to_user_part1_reg_restores(armv7m: &mut Armv7m) {
    // mov
    // NOTE: these two saves are pretty funny: we can't mark them as
    // clobbers directly using rust's register interface but since r6, r7
    // are callee saved registers in ARM and we use them
    // the compiler saves them on the stack anyway (see push_gpr in clobber saving)
    armv7m.mov(GPR::r2(), GPR::r6());
    armv7m.mov(GPR::r3(), GPR::r7());
    // note ip is intraprocedure scratch register - r12
    armv7m.mov(GPR::r12(), GPR::r9());

    // msr
    armv7m.msr(SpecialRegister::psp(), GPR::r0());

    // ldmia
    armv7m.ldmia_w(
        GPR::r1(),
        GPR::r4(),
        GPR::r5(),
        GPR::r6(),
        GPR::r7(),
        GPR::r8(),
        GPR::r9(),
        GPR::r10(),
        GPR::r11(),
    );
}

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
#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu])
        requires switch_to_user_pt1_precondition(old_cpu)
        ensures self: Armv7m { new_cpu: new_cpu == cpu_post_switch_to_user_pt1(old_cpu) }
)]
pub fn switch_to_user_part1(armv7m: &mut Armv7m) {
    switch_to_user_part1_save_clobbers(armv7m);
    switch_to_user_part1_reg_restores(armv7m);
    armv7m.svc(0xff);
}

#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu])
        requires switch_to_user_pt2_save_registers_precondition(old_cpu)
        ensures self: Armv7m { new_cpu: new_cpu == cpu_post_switch_to_user_pt2_save_registers(old_cpu) }
)]
pub fn switch_to_user_part2_save_registers(armv7m: &mut Armv7m) {
    armv7m.stmia_w(
        GPR::r1(),
        GPR::r4(),
        GPR::r5(),
        GPR::r6(),
        GPR::r7(),
        GPR::r8(),
        GPR::r9(),
        GPR::r10(),
        GPR::r11(),
    );
}

#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu])
        requires switch_to_user_pt2_restore_clobbers_precondition(old_cpu)
        ensures self: Armv7m { new_cpu: new_cpu == cpu_post_switch_to_user_pt2_restore_clobbers(old_cpu) }
)]
pub fn switch_to_user_part2_restore_clobbers(armv7m: &mut Armv7m) {
    armv7m.mrs(GPR::r0(), SpecialRegister::psp());
    armv7m.mov(GPR::r6(), GPR::r2());
    armv7m.mov(GPR::r7(), GPR::r3());
    armv7m.mov(GPR::r9(), GPR::r12());
    armv7m.ldmia_w_special(SpecialRegister::sp(), GPR::r8(), GPR::r10(), GPR::r11());
    armv7m.pop(
        GPR::r4(),
        GPR::r5(),
        GPR::r6(),
        GPR::r7(),
        SpecialRegister::pc(),
    );
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
        requires switch_to_user_pt2_precondition(old_cpu)
        ensures self: Armv7m { new_cpu: new_cpu == cpu_post_switch_to_user_pt2(old_cpu) }
)]
pub fn switch_to_user_part2(armv7m: &mut Armv7m) {
    switch_to_user_part2_save_registers(armv7m);
    switch_to_user_part2_restore_clobbers(armv7m);
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
        register_frame_preserved(sp_main(new_cpu.sp), old_cpu, new_cpu)
        &&
        sp_can_handle_exception_entry(new_cpu)
    }
)]
fn process(armv7m: &mut Armv7m) {}

#[flux_rs::sig(fn (&Armv7m[@cpu]) -> BV32[get_gpr(r1(), cpu)])]
fn get_r1(armv7m: &Armv7m) -> BV32 {
    *armv7m.general_regs.get(&GPR::r1()).unwrap()
}

#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu], u8[@exception_num])
       requires
            (exception_num == 11 || exception_num >= 15)
            &&
            mode_is_thread_privileged(old_cpu.mode, old_cpu.control)
            &&
            get_gpr(r0(), old_cpu) == bv32(0x8FFF_FFFF)
            &&
            get_gpr(r1(), old_cpu) == bv32(0x7000_0020)
            &&
            sp_main(old_cpu.sp) == bv32(0x6050_0000)
       ensures self: Armv7m { new_cpu:
           // r0, r2, r3, and r12 are clobbered but are caller saved
           get_gpr(r1(), new_cpu) == get_gpr(r1(), old_cpu)
           &&
           get_gpr(r4(), new_cpu) == get_gpr(r4(), old_cpu)
           &&
           get_gpr(r5(), new_cpu) == get_gpr(r5(), old_cpu)
           &&
           get_gpr(r6(), new_cpu) == get_gpr(r6(), old_cpu)
           &&
           get_gpr(r7(), new_cpu) == get_gpr(r7(), old_cpu)
           &&
           get_gpr(r8(), new_cpu) == get_gpr(r8(), old_cpu)
           &&
           get_gpr(r9(), new_cpu) == get_gpr(r9(), old_cpu)
           &&
           get_gpr(r10(), new_cpu) == get_gpr(r10(), old_cpu)
           &&
           get_gpr(r11(), new_cpu) == get_gpr(r11(), old_cpu)
           &&
           get_special_reg(lr(), new_cpu) == get_special_reg(lr(), old_cpu)
           &&
           get_special_reg(psr(), new_cpu) == get_special_reg(psr(), old_cpu)
           &&
           new_cpu.pc == old_cpu.lr
       }
)]
pub fn tock_control_flow_kernel_to_kernel(armv7m: &mut Armv7m, exception_num: u8) {
    // get r1 at the beginning of this so we can assert some facts with it later
    let original_r1 = get_r1(armv7m);

    // context switch asm
    switch_to_user_part1(armv7m);

    // run a process
    process(armv7m);

    // preempt the process with an arbitrary exception number
    armv7m.preempt(exception_num);

    // r1 can absolutely not change here - otherwise
    // we will save registers to the wrong place
    let curr_r1 = get_r1(armv7m);
    assert(original_r1 == curr_r1);

    // run the rest of the context switch asm
    switch_to_user_part2(armv7m);
}

#[flux_rs::trusted]
#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu])
        requires mode_is_thread_privileged(old_cpu.mode, old_cpu.control)
        ensures self: Armv7m { new_cpu:
            // r1 and r0 need to be saved for the process -
            // in principle this is stored somewhere and reloaded
            // but we'll just fake it here
            get_gpr(r0(), old_cpu) == get_gpr(r0(), new_cpu)
            &&
            get_gpr(r1(), old_cpu) == get_gpr(r1(), new_cpu)
            &&
            // mode is preserved
            mode_is_thread_privileged(new_cpu.mode, new_cpu.control)
            &&
            // fake the resulting sp - so we know there is no overlap
            sp_main(new_cpu.sp) == bv32(0x6040_0000)
            &&
            // sp process is preserved
            sp_process(old_cpu.sp) == sp_process(new_cpu.sp)
            &&
            // and we need to preserve where the process regs are saved
            register_frame_preserved(get_gpr(r1(), old_cpu), old_cpu, new_cpu)
            // and we need to preserve the hardware stacked process registers stack frame
            &&
            register_frame_preserved(sp_process(old_cpu.sp), old_cpu, new_cpu)
        }
)]
fn kernel(armv7m: &mut Armv7m) {}

#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu], u8[@exception_num])
        requires
            (exception_num == 11 || exception_num >= 15)
            &&
            mode_is_thread_unprivileged(old_cpu.mode, old_cpu.control)
            &&
            // the hardware stacked r1 (which has the addr of our stored registers
            // field) is valid and is far enough away from sp_main
            get_mem_addr(bv_add(sp_main(old_cpu.sp), bv32(0x4)), old_cpu.mem) == bv32(0x7000_0020)
            &&
            // sp process and sp main are far apart
            sp_process(old_cpu.sp) == bv32(0x8FFF_DDDD)
            &&
            sp_main(old_cpu.sp) == bv32(0x6050_0000)
        ensures self: Armv7m { new_cpu:
            sp_process(old_cpu.sp) == sp_process(new_cpu.sp)
            &&
            get_gpr(r0(), old_cpu) == get_gpr(r0(), new_cpu)
            &&
            get_gpr(r1(), old_cpu) == get_gpr(r1(), new_cpu)
            &&
            get_gpr(r2(), old_cpu) == get_gpr(r2(), new_cpu)
            &&
            get_gpr(r3(), old_cpu) == get_gpr(r3(), new_cpu)
            &&
            get_gpr(r4(), old_cpu) == get_gpr(r4(), new_cpu)
            &&
            get_gpr(r5(), new_cpu) == get_gpr(r5(), old_cpu)
            &&
            get_gpr(r6(), new_cpu) == get_gpr(r6(), old_cpu)
            &&
            get_gpr(r7(), new_cpu) == get_gpr(r7(), old_cpu)
            &&
            get_gpr(r8(), new_cpu) == get_gpr(r8(), old_cpu)
            &&
            get_gpr(r9(), new_cpu) == get_gpr(r9(), old_cpu)
            &&
            get_gpr(r10(), new_cpu) == get_gpr(r10(), old_cpu)
            &&
            get_gpr(r11(), new_cpu) == get_gpr(r11(), old_cpu)
            &&
            get_gpr(r12(), new_cpu) == get_gpr(r12(), old_cpu)
            &&
            get_special_reg(lr(), new_cpu) == get_special_reg(lr(), old_cpu)
            &&
            get_special_reg(psr(), new_cpu) == get_special_reg(psr(), old_cpu)
        }
)]
pub fn tock_control_flow_process_to_process(armv7m: &mut Armv7m, exception_num: u8) {
    // arbitrary code executing gets preempted
    armv7m.preempt(exception_num);

    // run second half of switch to user code
    switch_to_user_part2(armv7m); // sp process is put into r0

    // random kernel code that blows up our state
    kernel(armv7m); // the kernel has to save r0 here and restore it

    // back to process
    switch_to_user_part1(armv7m);
}

mod arm_test {
    use crate::{
        armv7m::{
            cpu::{Armv7m, SP},
            lang::{SpecialRegister, GPR},
            mem::{flux_defs, Memory},
        },
        assert,
    };
    use flux_rs::bitvec::BV32;

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
            register_frame_preserved(sp_main(new_cpu.sp), old_cpu, new_cpu)
            &&
            sp_can_handle_exception_entry(new_cpu)
        }
    )]
    fn process(armv7m: &mut Armv7m) {}

    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], u8[@exception_num])
           requires
               (exception_num == 11 || exception_num >= 15)
               &&
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
        armv7m.movw_imm(GPR::r0(), BV32::from(10));
        armv7m.preempt(11);
        // process that havocs all state except the main sp and the fact it's in thread mode unprivileged
        process(armv7m);
        // fake sys call
        armv7m.preempt(exception_number);
        // end up back here
        // no more instructions for now
    }
}
