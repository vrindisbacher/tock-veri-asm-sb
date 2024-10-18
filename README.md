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

As part of broader work verifying the Tock OS, we propose verifying the top half interrupt handlers for ARMv6m, ARMv6m, and RISC-V architectures. There is also the option of verifying other assembly (e.g. context switching from the kernel to run processes which involves assembly that saves and restores register values). In fact, this may be necessary to reason about the ISRs themselves.

This will likely involve two lines of work. 

1. Reasoning about preconditions prior to entering ISRs or other bits of assembly
    - For example, the ARMv6m ISR assumes that register R1 is pointing to the previously executing process struct's stored registers field. It is not immediately obvious why this is always the case.
2. Reasoning about bits of assembly 
    - We must make sure that we do not clobber registers or memory accidentally, must make sure the assembly aligns with the hardware's handling of interrupts, and must make sure we can actually deal with ISRs being pre-empted.


## Long Term Goals

One potentially interesting future artifact would be a fully verified ARM or RISC-V emulator. This could be useful for folks needing to write bits of inline assembly in Rust programs. One could first write specifications in a lifted Rust version and add some refinement annotations. For example, here is a really simple contract on a program that moves the value `0` into register `r0`.

```rust
#[flux_rs::sig(fn (arm: &strg Armv7m[@cpu]) ensures arm: Armv7m { v: v.r0 == 0 })]
fn simple_mov_program(armv7m: &mut Armv7m) {
    armv7m.mov(GeneralPurposeRegister::R0, Value::Value(0));
}
```

An example of the specification for `mov` on an ARMv7m emulator might look roughly like this:

```rust

flux_rs::defs! {
    fn get_reg(reg: int, cpu: Armv7m) -> int {
        if reg == 0 {
            cpu.r0
        } else if reg == 1 {
            cpu.r1
        } else if reg == 2 {
            cpu.r2
        } else if reg == 3 {
            cpu.r3
        } else if reg == 4 {
            cpu.r4
        } else if reg == 5 {
            cpu.r5
        } else if reg == 6 {
            cpu.r6
        } else if reg == 7 {
            cpu.r7
        } else if reg == 8 {
            cpu.r8
        } else if reg == 9 {
            cpu.r9
        } else if reg == 10 {
            cpu.r10
        } else if reg == 11 {
            cpu.r11
        } else if reg == 12 {
            cpu.r12
        } else if reg == 13 {
            cpu.r13
        } else if reg == 14 { 
            cpu.r14
        } else {
            cpu.r15
        }
    }

    fn value_into_u32(value: Value, cpu: Armv7m) -> int {
        if value.is_reg {
            get_reg(value.val, cpu)
        } else {
            value.val
        }
    }
}

#[flux_rs::refined_by(n: int)]
pub enum GeneralPurposeRegister {
    #[variant(GeneralPurposeRegister[0])]
    R0,
    #[variant(GeneralPurposeRegister[1])]
    R1,
    #[variant(GeneralPurposeRegister[2])]
    R2,
    #[variant(GeneralPurposeRegister[3])]
    R3,
    #[variant(GeneralPurposeRegister[4])]
    R4,
    #[variant(GeneralPurposeRegister[5])]
    R5,
    #[variant(GeneralPurposeRegister[6])]
    R6,
    #[variant(GeneralPurposeRegister[7])]
    R7,
    #[variant(GeneralPurposeRegister[8])]
    R8,
    #[variant(GeneralPurposeRegister[9])]
    R9,
    #[variant(GeneralPurposeRegister[10])]
    R10,
    #[variant(GeneralPurposeRegister[11])]
    R11,
    #[variant(GeneralPurposeRegister[12])]
    R12,
    #[variant(GeneralPurposeRegister[13])]
    R13,
    // Link Register
    #[variant(GeneralPurposeRegister[14])]
    R14,
    #[variant(GeneralPurposeRegister[15])]
    R15,
}

#[flux_rs::refined_by(is_reg: bool, val: int)]
pub enum Value {
    #[variant({GeneralPurposeRegister[@n]} -> Value[true, n])]
    Register(GeneralPurposeRegister),
    #[variant({u32[@n]} -> Value[false, n])]
    Value(u32),
}

#[derive(Debug)]
#[flux_rs::refined_by(
    r0: int, 
    r1: int,
    r2: int, 
    r3: int,
    r4: int,
    r5: int,
    r6: int,
    r7: int,
    r8: int,
    r9: int,
    r10: int, 
    r11: int,
    r12: int,
    r13: int,
    r14: int,
    r15: int,
    psr: int,
    primask: int,
    basepri: int,
    faultmask: int,
    control: int,
)]
pub struct Armv7m {
    #[field(u32[r0])]
    pub r0: u32,
    #[field(u32[r1])]
    r1: u32,
    #[field(u32[r2])]
    r2: u32,
    #[field(u32[r3])]
    r3: u32,
    #[field(u32[r4])]
    r4: u32,
    #[field(u32[r5])]
    r5: u32,
    #[field(u32[r6])]
    r6: u32,
    #[field(u32[r7])]
    r7: u32,
    #[field(u32[r8])]
    r8: u32,
    #[field(u32[r9])]
    r9: u32,
    #[field(u32[r10])]
    r10: u32,
    #[field(u32[r11])]
    r11: u32,
    #[field(u32[r12])]
    r12: u32,
    // r13 is the stack pointer
    #[field(u32[r13])]
    r13: u32,
    // r14 is the link register
    #[field(u32[r14])]
    r14: u32,
    // r15 is the program counter
    #[field(u32[r15])]
    r15: u32,
    //
    // Special Registers below
    //
    // PSR has 3 sub registers:
    //
    // APSR, IPSR, EPSR
    #[field(u32[psr])]
    psr: u32,
    // Mask registers
    //
    // primask is 1 bit - the rest being reserved
    #[field(u32[primask])]
    primask: u32,
    // basepri is 8 bit - the rest being reserved
    #[field(u32[basepri])]
    basepri: u32,
    // faultmask is 1 bit - the rest being reserved
    #[field(u32[faultmask])]
    faultmask: u32,
    // Control register (2 bit or 3 bit) depending on the specific processor
    #[field(u32[control])]
    control: u32,
}

impl Armv7m {

    #[flux_rs::sig(fn (&Armv7m[@cpu], &GeneralPurposeRegister[@reg]) -> u32[get_reg(reg, cpu)])]
    fn get_value_from_reg(&self, register: &GeneralPurposeRegister) -> u32 {
        match register {
            GeneralPurposeRegister::R0 => self.r0,
            GeneralPurposeRegister::R1 => self.r1,
            GeneralPurposeRegister::R2 => self.r2,
            GeneralPurposeRegister::R3 => self.r3,
            GeneralPurposeRegister::R4 => self.r4,
            GeneralPurposeRegister::R5 => self.r5,
            GeneralPurposeRegister::R6 => self.r6,
            GeneralPurposeRegister::R7 => self.r7,
            GeneralPurposeRegister::R8 => self.r8,
            GeneralPurposeRegister::R9 => self.r9,
            GeneralPurposeRegister::R10 => self.r10,
            GeneralPurposeRegister::R11 => self.r11,
            GeneralPurposeRegister::R12 => self.r12,
            GeneralPurposeRegister::R13 => self.r13,
            GeneralPurposeRegister::R14 => self.r14,
            GeneralPurposeRegister::R15 => self.r15,
        }
    }

    #[flux_rs::sig(fn (&Armv7m[@cpu], Value[@val]) -> u32[value_into_u32(val, cpu)])]
    fn value_into_u32(&self, value: Value) -> u32 {
        match value {
            Value::Register(register) => self.get_value_from_reg(&register),
            Value::Value(v) => v,
        }
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], u32[@new_val]) ensures self: Armv7m { new_cpu: get_reg(reg, new_cpu) == new_val })] 
    fn update_reg_with_u32(&mut self, register: GeneralPurposeRegister, value: u32) {
        match register {
            GeneralPurposeRegister::R0 => self.r0 = value,
            GeneralPurposeRegister::R1 => self.r1 = value,
            GeneralPurposeRegister::R2 => self.r2 = value,
            GeneralPurposeRegister::R3 => self.r3 = value,
            GeneralPurposeRegister::R4 => self.r4 = value,
            GeneralPurposeRegister::R5 => self.r5 = value,
            GeneralPurposeRegister::R6 => self.r6 = value,
            GeneralPurposeRegister::R7 => self.r7 = value,
            GeneralPurposeRegister::R8 => self.r8 = value,
            GeneralPurposeRegister::R9 => self.r9 = value,
            GeneralPurposeRegister::R10 => self.r10 = value,
            GeneralPurposeRegister::R11 => self.r11 = value,
            GeneralPurposeRegister::R12 => self.r12 = value,
            GeneralPurposeRegister::R13 => self.r13 = value,
            GeneralPurposeRegister::R14 => self.r14 = value,
            GeneralPurposeRegister::R15 => self.r15 = value,
        }
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], Value[@val]) 
        ensures self: Armv7m { 
            new_cpu: get_reg(reg, new_cpu) == value_into_u32(val, old_cpu) 
        }
    )]
    pub fn mov(&mut self, register: GeneralPurposeRegister, value: Value) {
        // Move immediate - writes a value into destination register
        // This does not cause a flag update
        let val = self.value_into_u32(value);
        self.update_reg_with_u32(register, val);
    }
}
```
