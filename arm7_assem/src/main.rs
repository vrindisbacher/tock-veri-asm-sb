#![no_std]
#![no_main]

#[allow(dead_code)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
#[inline(never)]
pub fn do_stuff(user_stack: *const usize, process_regs: &mut [usize; 8]) -> *const usize {
    unsafe { switch_to_user_arm_v7m(user_stack, process_regs) }
}

#[cfg(any(doc, all(target_arch = "arm", target_os = "none")))]
#[inline(never)]
pub unsafe fn switch_to_user_arm_v7m(
    mut user_stack: *const usize,
    process_regs: &mut [usize; 8],
) -> *const usize {
    use core::arch::asm;
    asm!(
    "
    // Rust `asm!()` macro (as of May 2021) will not let us mark r6, r7 and r9
    // as clobbers. r6 and r9 is used internally by LLVM, and r7 is used for
    // the frame pointer. However, in the process of restoring and saving the
    // process's registers, we do in fact clobber r6, r7 and r9. So, we work
    // around this by doing our own manual saving of r6 using r2, r7 using r3,
    // r9 using r12, and then mark those as clobbered.
    mov r2, r6                        // r2 = r6
    mov r3, r7                        // r3 = r7
    mov r12, r9                       // r12 = r9

    // The arguments passed in are:
    // - `r0` is the bottom of the user stack
    // - `r1` is a reference to `CortexMStoredState.regs`

    // Load bottom of stack into Process Stack Pointer.
    msr psp, r0                       // PSP = r0

    // Load non-hardware-stacked registers from the process stored state. Ensure
    // that the address register (right now r1) is stored in a callee saved
    // register.
    ldmia r1, {{r4-r11}}              // r4 = r1[0], r5 = r1[1], ...

    // Generate a SVC exception to handle the context switch from kernel to
    // userspace. It doesn't matter which SVC number we use here as it is not
    // used in the exception handler. Data being returned from a syscall is
    // transferred on the app's stack.
    svc 0xff

    // When execution returns here we have switched back to the kernel from the
    // application.

    // Push non-hardware-stacked registers into the saved state for the
    // application.
    stmia r1, {{r4-r11}}              // r1[0] = r4, r1[1] = r5, ...

    // Update the user stack pointer with the current value after the
    // application has executed.
    mrs r0, PSP                       // r0 = PSP

    // Need to restore r6, r7 and r12 since we clobbered them when switching to
    // and from the app.
    mov r6, r2                        // r6 = r2
    mov r7, r3                        // r7 = r3
    mov r9, r12                       // r9 = r12
    ",
    inout("r0") user_stack,
    in("r1") process_regs,
    out("r2") _, out("r3") _, out("r4") _, out("r5") _, out("r8") _, out("r10") _,
    out("r11") _, out("r12") _);

    user_stack
}

// #[cfg(all(target_arch = "arm", target_os = "none"))]
// extern "C" {
//     /// Generic interrupt handler for ARMv7-M instruction sets.
//     ///
//     /// For documentation of this function, see `CortexMVariant::GENERIC_ISR`.
//     pub fn generic_isr_arm_v7m();
// }
// #[cfg(all(target_arch = "arm", target_os = "none"))]
// global_asm!(
//         "
//     .section .generic_isr_arm_v7m, \"ax\"
//     .global generic_isr_arm_v7m
//     generic_isr_arm_v7m:
//     // Use the CONTROL register to set the thread mode to privileged to ensure
//     // we are executing as the kernel. This may be redundant if the interrupt
//     // happened while the kernel code was executing.
//     //
//     // CONTROL[1]: Stack status
//     //   0 = Default stack (MSP) is used
//     //   1 = Alternate stack is used
//     // CONTROL[0]: Mode
//     //   0 = Privileged in thread mode
//     //   1 = User state in thread mode
//     mov r0, #0                        // r0 = 0
//     msr CONTROL, r0                   // CONTROL = 0
//     // CONTROL writes must be followed by an Instruction Synchronization Barrier
//     // (ISB). https://developer.arm.com/documentation/dai0321/latest
//     isb
//
//     // Set the link register to the special EXC_RETURN value of 0xFFFFFFF9 which
//     // instructs the CPU to run in thread mode with the main (kernel) stack.
//     ldr lr, =0xFFFFFFF9               // LR = 0xFFFFFFF9
//
//     // Now need to disable the interrupt that fired in the NVIC to ensure it
//     // does not trigger again before the scheduler has a chance to handle it. We
//     // do this here in assembly for performance.
//     //
//     // The general idea is:
//     // 1. Get the index of the interrupt that occurred.
//     // 2. Set the disable bit for that interrupt in the NVIC.
//
//     // Find the ISR number (`index`) by looking at the low byte of the IPSR
//     // registers.
//     mrs r0, IPSR                      // r0 = Interrupt Program Status Register (IPSR)
//     and r0, #0xff                     // r0 = r0 & 0xFF; Get lowest 8 bits
//     sub r0, #16                       // r0 = r0 - 16;   ISRs start at 16, so subtract 16 to get zero-indexed.
//
//     // Now disable that interrupt in the NVIC.
//     // High level:
//     //    r0 = index
//     //    NVIC.ICER[r0 / 32] = 1 << (r0 & 31)
//     lsrs r2, r0, #5                   // r2 = r0 / 32
//     // r0 = 1 << (r0 & 31)
//     movs r3, #1                       // r3 = 1
//     and r0, r0, #31                   // r0 = r0 & 31
//     lsl r0, r3, r0                    // r0 = r3 << r0
//
//     // Load the ICER register address.
//     ldr r3, =0xe000e180               // r3 = &NVIC.ICER
//
//     // Here:
//     // - `r2` is index / 32
//     // - `r3` is &NVIC.ICER
//     // - `r0` is 1 << (index & 31)
//     str r0, [r3, r2, lsl #2]          // *(r3 + r2 * 4) = r0
//
//     // The pending bit in ISPR might be reset by hardware for pulse interrupts
//     // at this point. So set it here again so the interrupt does not get lost in
//     // `service_pending_interrupts()`.
//     ldr r3, =0xe000e200               // r3 = &NVIC.ISPR
//     str r0, [r3, r2, lsl #2]          // *(r3 + r2 * 4) = r0
//
//     // Now we can return from the interrupt context and resume what we were
//     // doing. If an app was executing we will switch to the kernel so it can
//     // choose whether to service the interrupt.
//     bx lr
// ");
