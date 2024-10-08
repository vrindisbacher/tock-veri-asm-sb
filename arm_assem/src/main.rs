#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use core::arch::global_asm;

#[cfg(all(target_arch = "arm", target_os = "none"))]
extern "C" {
    /// Generic interrupt handler for ARMv7-M instruction sets.
    ///
    /// For documentation of this function, see `CortexMVariant::GENERIC_ISR`.
    pub fn generic_isr_arm_v7m();
}
#[cfg(all(target_arch = "arm", target_os = "none"))]
global_asm!(
        "
    .section .generic_isr_arm_v7m, \"ax\"
    .global generic_isr_arm_v7m
    generic_isr_arm_v7m:
    // Use the CONTROL register to set the thread mode to privileged to ensure
    // we are executing as the kernel. This may be redundant if the interrupt
    // happened while the kernel code was executing.
    //
    // CONTROL[1]: Stack status
    //   0 = Default stack (MSP) is used
    //   1 = Alternate stack is used
    // CONTROL[0]: Mode
    //   0 = Privileged in thread mode
    //   1 = User state in thread mode
    mov r0, #0                        // r0 = 0
    msr CONTROL, r0                   // CONTROL = 0
    // CONTROL writes must be followed by an Instruction Synchronization Barrier
    // (ISB). https://developer.arm.com/documentation/dai0321/latest
    isb

    // Set the link register to the special EXC_RETURN value of 0xFFFFFFF9 which
    // instructs the CPU to run in thread mode with the main (kernel) stack.
    ldr lr, =0xFFFFFFF9               // LR = 0xFFFFFFF9

    // Now need to disable the interrupt that fired in the NVIC to ensure it
    // does not trigger again before the scheduler has a chance to handle it. We
    // do this here in assembly for performance.
    //
    // The general idea is:
    // 1. Get the index of the interrupt that occurred.
    // 2. Set the disable bit for that interrupt in the NVIC.

    // Find the ISR number (`index`) by looking at the low byte of the IPSR
    // registers.
    mrs r0, IPSR                      // r0 = Interrupt Program Status Register (IPSR)
    and r0, #0xff                     // r0 = r0 & 0xFF; Get lowest 8 bits
    sub r0, #16                       // r0 = r0 - 16;   ISRs start at 16, so subtract 16 to get zero-indexed.

    // Now disable that interrupt in the NVIC.
    // High level:
    //    r0 = index
    //    NVIC.ICER[r0 / 32] = 1 << (r0 & 31)
    lsrs r2, r0, #5                   // r2 = r0 / 32
    // r0 = 1 << (r0 & 31)
    movs r3, #1                       // r3 = 1
    and r0, r0, #31                   // r0 = r0 & 31
    lsl r0, r3, r0                    // r0 = r3 << r0

    // Load the ICER register address.
    ldr r3, =0xe000e180               // r3 = &NVIC.ICER

    // Here:
    // - `r2` is index / 32
    // - `r3` is &NVIC.ICER
    // - `r0` is 1 << (index & 31)
    str r0, [r3, r2, lsl #2]          // *(r3 + r2 * 4) = r0

    // The pending bit in ISPR might be reset by hardware for pulse interrupts
    // at this point. So set it here again so the interrupt does not get lost in
    // `service_pending_interrupts()`.
    ldr r3, =0xe000e200               // r3 = &NVIC.ISPR
    str r0, [r3, r2, lsl #2]          // *(r3 + r2 * 4) = r0

    // Now we can return from the interrupt context and resume what we were
    // doing. If an app was executing we will switch to the kernel so it can
    // choose whether to service the interrupt.
    bx lr
");
