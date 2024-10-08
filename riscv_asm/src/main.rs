#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use core::arch::global_asm;

extern "C" {
    // Where the end of the stack region is (and hence where the stack should
    // start), and the start of the stack region.
    static _estack: usize;
    static _sstack: usize;

    // Boundaries of the .bss section.
    static mut _szero: usize;
    static mut _ezero: usize;

    // Where the .data section is stored in flash.
    static mut _etext: usize;

    // Boundaries of the .data section.
    static mut _srelocate: usize;
    static mut _erelocate: usize;

    // The global pointer, value set in the linker script
    #[link_name = "__global_pointer$"]
    static __global_pointer: usize;
}

#[cfg(all(target_arch = "riscv32", target_os = "none"))]
extern "C" {
    /// This is the trap handler function. This code is called on all traps,
    /// including interrupts, exceptions, and system calls from applications.
    ///
    /// Tock uses only the single trap handler, and does not use any vectored
    /// interrupts or other exception handling. The trap handler has to
    /// determine why the trap handler was called, and respond
    /// accordingly. Generally, there are two reasons the trap handler gets
    /// called: an interrupt occurred or an application called a syscall.
    ///
    /// In the case of an interrupt while the kernel was executing we only need
    /// to save the kernel registers and then run whatever interrupt handling
    /// code we need to. If the trap happens while an application was executing,
    /// we have to save the application state and then resume the `switch_to()`
    /// function to correctly return back to the kernel.
    ///
    /// We implement this distinction through a branch on the value of the
    /// `mscratch` CSR. If, at the time the trap was taken, it contains `0`, we
    /// assume that the hart is currently executing kernel code.
    ///
    /// If it contains any other value, we interpret it to be a memory address
    /// pointing to a particular data structure:
    ///
    /// ```text
    /// mscratch           0               1               2               3
    ///  \->|--------------------------------------------------------------|
    ///     | scratch word, overwritten with s1 register contents          |
    ///     |--------------------------------------------------------------|
    ///     | trap handler address, continue trap handler execution here   |
    ///     |--------------------------------------------------------------|
    /// ```
    ///
    /// Thus, process implementations can define their own strategy for how
    /// traps should be handled when they occur during process execution. This
    /// global trap handler behavior is well defined. It will:
    ///
    /// 1. atomically swap s0 and the mscratch CSR,
    ///
    /// 2. execute the default kernel trap handler if s0 now contains `0`
    /// (meaning that the mscratch CSR contained `0` before entering this trap
    /// handler),
    ///
    /// 3. otherwise, save s1 to `0*4(s0)`, and finally
    ///
    /// 4. load the address at `1*4(s0)` into s1, and jump to it.
    ///
    /// No registers other than s0, s1 and the mscratch CSR are to be clobbered
    /// before continuing execution at the address loaded into the mscratch CSR
    /// or the _start_kernel_trap kernel trap handler.  Execution with these
    /// second-stage trap handlers must continue in the same trap handler
    /// context as originally invoked by the trap (e.g., the global trap handler
    /// will not execute an mret instruction). It will not modify CSRs that
    /// contain information on the trap source or the system state prior to
    /// entering the trap handler.
    ///
    /// We deliberately clobber callee-saved instead of caller-saved registers,
    /// as this makes it easier to call other functions as part of the trap
    /// handler (for example to to disable interrupts from within Rust
    /// code). This global trap handler saves the previous values of these
    /// clobbered registers ensuring that they can be restored later.  It places
    /// new values into these clobbered registers (such as the previous `s0`
    /// register contents) that are required to be retained for correctly
    /// returning from the trap handler, and as such need to be saved across
    /// C-ABI function calls. Loading them into saved registers avoids the need
    /// to manually save them across such calls.
    ///
    /// When a custom trap handler stack is registered in `mscratch`, the custom
    /// handler is responsible for restoring the kernel trap handler (by setting
    /// mscratch=0) before returning to kernel execution from the trap handler
    /// context.
    ///
    /// If a board or chip must, for whichever reason, use a different global
    /// trap handler, it should abide to the above contract and emulate its
    /// behavior for all traps and interrupts that are required to be handled by
    /// the respective kernel or other trap handler as registered in mscratch.
    ///
    /// For instance, a chip that does not support non-vectored trap handlers
    /// can register a vectored trap handler that routes each trap source to
    /// this global trap handler.
    ///
    /// Alternatively, a board can be allowed to ignore certain traps or
    /// interrupts, some or all of the time, provided they are not vital to
    /// Tock's execution. These boards may choose to register an alternative
    /// handler for some or all trap sources. When this alternative handler is
    /// invoked, it may, for instance, choose to ignore a certain trap, access
    /// global state (subject to synchronization), etc. It must still abide to
    /// the contract as stated above.
    pub fn _start_trap();
}

#[cfg(all(target_arch = "riscv32", target_os = "none"))]
global_asm!(
    "
            .section .riscv.trap, \"ax\"
            .globl _start_trap
          _start_trap:
            // This is the global trap handler. By default, Tock expects this
            // trap handler to be registered at all times, and that all traps
            // and interrupts occurring in all modes of execution (M-, S-, and
            // U-mode) will cause this trap handler to be executed.
            //
            // For documentation of its behavior, and how process
            // implementations can hook their own trap handler code, see the
            // comment on the `extern C _start_trap` symbol above.

            // Atomically swap s0 and mscratch:
            csrrw s0, mscratch, s0        // s0 = mscratch; mscratch = s0

            // If mscratch contained 0, invoke the kernel trap handler.
            beq   s0, x0, 100f      // if s0==x0: goto 100

            // Else, save the current value of s1 to `0*4(s0)`, load `1*4(s0)`
            // into s1 and jump to it (invoking a custom trap handler).
            sw    s1, 0*4(s0)       // *s0 = s1
            lw    s1, 1*4(s0)       // s1 = *(s0+4)
            jr    s1                // goto s1

          100: // _start_kernel_trap

            // The global trap handler has swapped s0 into mscratch. We can thus
            // freely clobber s0 without losing any information.
            //
            // Since we want to use the stack to save kernel registers, we
            // first need to make sure that the trap wasn't the result of a
            // stack overflow, in which case we can't use the current stack
            // pointer. Use s0 as a scratch register:

            // Load the address of the bottom of the stack (`_sstack`) into our
            // newly freed-up s0 register.
            la s0, {sstack}                     // s0 = _sstack

            // Compare the kernel stack pointer to the bottom of the stack. If
            // the stack pointer is above the bottom of the stack, then continue
            // handling the fault as normal.
            bgtu sp, s0, 200f                   // branch if sp > s0

            // If we get here, then we did encounter a stack overflow. We are
            // going to panic at this point, but for that to work we need a
            // valid stack to run the panic code. We do this by just starting
            // over with the kernel stack and placing the stack pointer at the
            // top of the original stack.
            la sp, {estack}                     // sp = _estack

        200: // _start_kernel_trap_continue

            // Restore s0. We reset mscratch to 0 (kernel trap handler mode)
            csrrw s0, mscratch, zero    // s0 = mscratch; mscratch = 0

            // Make room for the caller saved registers we need to restore after
            // running any trap handler code.
            addi sp, sp, -16*4

            // Save all of the caller saved registers.
            sw   ra, 0*4(sp)
            sw   t0, 1*4(sp)
            sw   t1, 2*4(sp)
            sw   t2, 3*4(sp)
            sw   t3, 4*4(sp)
            sw   t4, 5*4(sp)
            sw   t5, 6*4(sp)
            sw   t6, 7*4(sp)
            sw   a0, 8*4(sp)
            sw   a1, 9*4(sp)
            sw   a2, 10*4(sp)
            sw   a3, 11*4(sp)
            sw   a4, 12*4(sp)
            sw   a5, 13*4(sp)
            sw   a6, 14*4(sp)
            sw   a7, 15*4(sp)

            // Jump to board-specific trap handler code. Likely this was an
            // interrupt and we want to disable a particular interrupt, but each
            // board/chip can customize this as needed.
            jal ra, _start_trap_rust_from_kernel

            // Restore the registers from the stack.
            lw   ra, 0*4(sp)
            lw   t0, 1*4(sp)
            lw   t1, 2*4(sp)
            lw   t2, 3*4(sp)
            lw   t3, 4*4(sp)
            lw   t4, 5*4(sp)
            lw   t5, 6*4(sp)
            lw   t6, 7*4(sp)
            lw   a0, 8*4(sp)
            lw   a1, 9*4(sp)
            lw   a2, 10*4(sp)
            lw   a3, 11*4(sp)
            lw   a4, 12*4(sp)
            lw   a5, 13*4(sp)
            lw   a6, 14*4(sp)
            lw   a7, 15*4(sp)

            // Reset the stack pointer.
            addi sp, sp, 16*4

            // mret returns from the trap handler. The PC is set to what is in
            // mepc and execution proceeds from there. Since we did not modify
            // mepc we will return to where the exception occurred.
            mret
    ",
    estack = sym _estack,
    sstack = sym _sstack,
);
