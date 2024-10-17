# Verifying Tock Interrupt Service Routines (and maybe? more assembly)

## Motivation

Tock is an embedded operating system written in the Rust programming language. Rust guarantees memory safety while simultaneously avoiding garbage collection and allowing developers fine control of memory. Unfortunately, kernels inevitably require some unsafe code. One such example is interrupts. Handling interrupts efficiently and correctly is a core responsibility of the kernel, and mistakes in interrupt handlers can cause serious issues such as data loss or device malfunctions.

Tock handles interrupts in a top / bottom half architecture. The top half of an interrupt is responsible for acknowledging an interrupt as fast as possible, and scheduling it for the bottom half. Then, when it is "safer" to do so, the bottom half handles the actual interrupt, executing any necessary logic. Performance is a critical part of these top half handlers, as other interrupts are generally disabled and spending too long in an interrupt handler could quickly degrade system performance. To deal with this, Tock's top half handlers are written in inline assembly.

Writing assembly is hard, and bits of assembly can undermine the type and memory safety - the point of writing a kernel in Rust. A few key challenges are...

- Saving and restoring process state correctly
    - ISRs must not clobber registers used in the previous context. If they do, they must save and restore their state.
- Making sure ISRs account for and interact with hardware semantics correctly
    - See [this long thread](https://groups.google.com/g/tock-dev/c/fZQYq0dpeSQ/m/Xi7oUtuiBAAJ?pli=1) about an interrupt issue with ARM devices in Tock
- Dealing with unclear control flow
    - Interrupts are asynchronous to the main kernel loop. Therefore, understanding what preconditions and postconditions must hold for an interrupt is difficult.
    - Furthermore, the correctness of some ISRs depend on other bits of assembly (for example the assembly responsible for doing context switches)
- Dealing with interrupts interrupting the interrupt handler :)
    - For example, in ARM architectures, it is possible for ISRs to be pre-empted by higher priority interrupts.

## Proposal

As part of broader work verifying the Tock OS, we propose verifying the top half interrupt handlers for ARMv6m, ARMv6m, and RISC-V architectures. There is also the option of verifying other assembly (e.g. context switching from the kernel to run processes which involves assembly that saves and restores register values).

This will likely involve two lines of work. 

1. Reasoning about preconditions prior to entering ISRs or other bits of assembly
    - For example, the ARMv6m ISR assumes that register R1 is pointing to the previously executing process struct's stored registers field. It is not immediately obvious why this is always the case.
2. Reasoning about bits of assembly 
    - We must make sure that we do not clobber registers or memory accidentally, must make sure the assembly aligns with the hardware's handling of interrupts, and must make sure we can actually deal with ISRs being pre-empted.

