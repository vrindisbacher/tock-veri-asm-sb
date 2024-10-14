#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use core::arch::global_asm;

#[cfg(all(target_arch = "arm", target_os = "none"))]
extern "C" {
    /// All ISRs are caught by this handler which disables the NVIC and switches to the kernel.
    pub fn generic_isr();
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
global_asm!(
    "
    .section .generic_isr, \"ax\"
    .global generic_isr
    .thumb_func
  generic_isr:
    /* Skip saving process state if not coming from user-space */
    ldr r0, 300f // MEXC_RETURN_PSP
    cmp lr, r0
    bne 100f

    /* We need the most recent kernel's version of r1, which points */
    /* to the Process struct's stored registers field. The kernel's r1 */
    /* lives in the second word of the hardware stacked registers on MSP */
    mov r1, sp
    ldr r1, [r1, #4]
    str r4, [r1, #16]
    str r5, [r1, #20]
    str r6, [r1, #24]
    str r7, [r1, #28]

    push {{r4-r7}}
    mov  r4, r8
    mov  r5, r9
    mov  r6, r10
    mov  r7, r11
    str r4, [r1, #0]
    str r5, [r1, #4]
    str r6, [r1, #8]
    str r7, [r1, #12]
    pop {{r4-r7}}

    ldr r0, 200f // MEXC_RETURN_MSP
100: // _ggeneric_isr_no_stacking
    /* Find the ISR number by looking at the low byte of the IPSR registers */
    mrs r0, IPSR
    movs r1, #0xff
    ands r0, r1
    /* ISRs start at 16, so subtract 16 to get zero-indexed */
    subs r0, r0, #16

    /*
     * High level:
     *    NVIC.ICER[r0 / 32] = 1 << (r0 & 31)
     * */
    /* r3 = &NVIC.ICER[r0 / 32] */
    ldr r2, 101f      /* r2 = &NVIC.ICER */
    lsrs r3, r0, #5   /* r3 = r0 / 32 */
    lsls r3, r3, #2   /* ICER is word-sized, so multiply offset by 4 */
    adds r3, r3, r2   /* r3 = r2 + r3 */

    /* r2 = 1 << (r0 & 31) */
    movs r2, #31      /* r2 = 31 */
    ands r0, r2       /* r0 = r0 & r2 */
    subs r2, r2, #30  /* r2 = r2 - 30 i.e. r2 = 1 */
    lsls r2, r2, r0   /* r2 = 1 << r0 */

    /* *r3 = r2 */
    str r2, [r3]

    /* The pending bit in ISPR might be reset by hardware for pulse interrupts
     * at this point. So set it here again so the interrupt does not get lost
     * in service_pending_interrupts()
     *
     * The NVIC.ISPR base is 0xE000E200, which is 0x20 (aka #32) above the
     * NVIC.ICER base.  Calculate the ISPR address by offsetting from the ICER
     * address so as to avoid re-doing the [r0 / 32] index math.
     */
    adds r3, #32
    str r2, [r3]

    bx lr /* return here since we have extra words in the assembly */

.align 4
101: // NVICICER
  .word 0xE000E180
200: // MEXC_RETURN_MSP
  .word 0xFFFFFFF9
300: // MEXC_RETURN_PSP
  .word 0xFFFFFFFD"
);
