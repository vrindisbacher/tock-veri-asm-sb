# Armv7m Status

Most of the work done so far has been on the Arm Thumb Instruction Set used by Cortex M devices. We have verified the ISR for ARMv7 using the simplest encoding possible. Below are details on our Rust / Flux representation of this architecture and the ISR we verified.

At a high level, we are currently able to...

1. Prove an assembly program using flux annotations (on a subset of Thumb instructions)
2. Prove that flux annotations for these programs are correct given our emulation

Importantly, our emulation has semantics which are taken from the ARM hardware spec. 

## Verified Armv7m ISR

We are able to lift Tock's ARM ISR to our framework and refine it as follows.

```rust
flux_rs::defs! {
    fn isr_bit_loc(old_cpu: Armv7m) -> B32 {
        bv32((to_int(get_special_reg(ipsr(), old_cpu)) - 16) % 32)
    }
    
    fn isr_r0(old_cpu: Armv7m) -> B32 {
        left_shift(
            bv32(1), 
            isr_bit_loc(old_cpu)
        )
    }

    fn isr_r2(old_cpu: Armv7m) -> B32 {
        bv32((to_int(get_special_reg(ipsr(), old_cpu)) - 16) / 32)
    }

    fn isr_offset(old_cpu: Armv7m) -> int {
        to_int(isr_r2(old_cpu)) * 4
    }
}

// Here is disassembly of the armv7m program. Note that the .w specifies "wide"
// for the 32 bit version of the instruction
//
//
//   0:   f04f 0000       mov.w   r0, #0
//   4:   f380 8814       msr     CONTROL, r0
//   8:   f3bf 8f6f       isb     sy
//   c:   f06f 0e06       mvn.w   lr, #6
//   10:   f3ef 8005       mrs     r0, IPSR
//   14:   f000 00ff       and.w   r0, r0, #255    @ 0xff
//   18:   f1a0 0010       sub.w   r0, r0, #16
//   1c:   0942            lsrs    r2, r0, #5
//   1e:   2301            movs    r3, #1
//   20:   f000 001f       and.w   r0, r0, #31
//   24:   fa03 f000       lsl.w   r0, r3, r0
//   28:   4b03            ldr     r3, [pc, #12]   @ (38 <generic_isr_arm_v7m+0x38>)
//   2a:   f843 0022       str.w   r0, [r3, r2, lsl #2]
//   2e:   4b03            ldr     r3, [pc, #12]   @ (3c <generic_isr_arm_v7m+0x3c>)
//   30:   f843 0022       str.w   r0, [r3, r2, lsl #2]
//   34:   4770            bx      lr
//   38:   e000e180        .word   0xe000e180
//   3c:   e000e200        .word   0xe000e200
#[flux_rs::sig(
    fn (self: &strg Armv7m[@old_cpu]) 
    // VTOCK TODO:
    //
    // Note we need to say that the IPSR is more than 16
    // This is guaranteed by exception entry but we should
    // probably formalize that somehow
    requires to_int(get_special_reg(ipsr(), old_cpu)) >= 16 
    ensures self: Armv7m { new_cpu:
        get_gpr(r0(), new_cpu) == isr_r0(old_cpu)
        &&
        get_gpr(r2(), new_cpu) == isr_r2(old_cpu)
        && 
        nth_bit_is_set(
            get_mem_addr(
                0xe000_e180 + isr_offset(old_cpu),
                new_cpu.mem
            ),
            isr_bit_loc(old_cpu)
        )
        &&
        nth_bit_is_set(
            get_mem_addr(
                0xe000_e200 + isr_offset(old_cpu),
                new_cpu.mem
            ),
            isr_bit_loc(old_cpu)
        )
    }
)]
pub fn generic_isr_armv7m(armv7m: &mut Armv7m) {
    // r0 = 0
    armv7m.movw_imm(GPR::R0, B32::from(0));
    // control = r0 = 0
    armv7m.msr(SpecialRegister::Control, GPR::R0);
    // isb
    armv7m.isb(Some(IsbOpt::Sys));
    // NOTE: using pseudo instr here
    // lr = 0xFFFFFFF9
    armv7m.pseudo_ldr_special(SpecialRegister::Lr, B32::from(0xFFFFFFF9));
    // r0 = ipsr
    armv7m.mrs(GPR::R0, SpecialRegister::IPSR);
    // Note: this seems to be a useless instruction?
    armv7m.and_imm(GPR::R0, B32::from(0xff));
    // r0 = ipsr - 16
    armv7m.subw_imm(GPR::R0, GPR::R0, B32::from(16));
    // r2 = r0 >> 5 ---> (ipsr - 16 / 32)
    armv7m.lsrs_imm(GPR::R2, GPR::R0, B32::from(5));
    // r3 = 1
    armv7m.movs_imm(GPR::R3, B32::from(1));
    // r0 = r0 & 31
    armv7m.and_imm(GPR::R0, B32::from(31));
    // r0 = r3 << r0
    //      -     -
    //      1     (ipsr - 16 & 31)
    armv7m.lslw_reg(
        GPR::R0,
        GPR::R3,
        GPR::R0,
    );
    // Note: Ignoring the dissasembled version of this because dealing with program counter is
    // annoying
    //
    // Gonna encode this as a pseudo instruction for now
    armv7m.pseudo_ldr(GPR::R3, B32::from(0xe000_e180));
    // r0 = 1 << (ipsr - 16 & 31)
    // r3 = 0xe000_e180
    // r2 = (ipsr - 16 >> 5) 
    armv7m.strw_lsl_reg(
        GPR::R0,
        GPR::R3,
        GPR::R2,
        B32::from(2),
    );
    // Note: Ignoring the dissasembled version of this because dealing with program counter is
    // annoying
    //
    // Gonna encode this as a pseudo instruction for now
    armv7m.pseudo_ldr(GPR::R3, B32::from(0xe000_e200));
    // r0 = 1 << (ipsr - 16 & 31)
    // r3 = 0xe000_e200
    // r2 = (ipsr - 16 >> 5) << 2
    //
    // mem[0xe000_e200 + ((ipsr - 16 >> 5) << 2)] = (1 << (ipsr - 16) & 31) i.e. "bit for the ipsr # is set"
    armv7m.strw_lsl_reg(
        GPR::R0,
        GPR::R3,
        GPR::R2,
        B32::from(2),
    );
    armv7m.bx(SpecialRegister::Lr);
}
```

This spec effectively says, "The proper bit (meaning the bit specified by the exception number in the ispr) in NVIC ISPR and ICER are set". Below are details on our implementation.

## General Framework

### The B32 Type

In order to represent registers in a manner that allows for efficient verification, we use a special type `B32`. Internally, `B32` is a simple wrapper around `u32` refined (in Flux) by a bitvector. We then implement most bitwise operands and arithmetic operations for this type (see below for a small example). 

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[flux_rs::opaque]
#[flux_rs::refined_by(x: bitvec<32>)]
pub struct B32(u32);

impl B32 {
    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@x], B32[@y]) -> B32[bv_add(x, y)])]
    pub fn wrapping_add(self, other: B32) -> B32 {
        B32(self.0.wrapping_add(other.0))
    }
}

impl BitAnd for B32 {
    type Output = B32;

    #[flux_rs::trusted]
    #[flux_rs::sig(fn (B32[@x], B32[@y]) -> B32[bv_and(x, y)])]
    fn bitand(self, rhs: Self) -> B32 {
        B32(self.0 & rhs.0)
    }
}
```

We could simply represent registers as `u32`, but this would require us to traverse in and out of the theory of bitvectors in order to reason about bitwise operands. This is a major performance overhead. 
 
__Note that using `B32` rather than `u32` resulted in Flux checking [our tests](https://github.com/vrindisbacher/tock-veri-asm-sb/blob/051efa260909288683e3cfb1524b8644e92ba84c/src/lib.rs#L115) in ~ 4 seconds rather than ~ 25 seconds.__

### Maps for 'Functional' Updates

By default, Flux does not provide functional updates on structs. This means you need to manually enforce that struct fields left untouched by operations are the same after a strong update on that field.

We have [opened an issue](https://github.com/flux-rs/flux/issues/870) to add support for functional updates on structs. This should help future cases that require 'stateful' reasoning (e.g. when info about what is in a struct field should be preserved across operations).

In the meantime, we use Maps (which allow functional updates in Z3, the SMT backend for Flux). Specifically, we define a special type `Regs`...

```rust
#![allow(dead_code)]

flux_rs::defs! {
    fn map_set<K, V>(m:Map<K, V>, k: K, v: V) -> Map<K, V> { map_store(m, k, v) }
    fn map_get<K, V>(m: Map<K, V>, k:K) -> V { map_select(m, k) }
}

use std::hash::Hash;

/// define a type indexed by a map
#[derive(Debug)]
#[flux_rs::opaque]
#[flux_rs::refined_by(vals: Map<K, V>)]
pub struct Regs<K, V> {
    inner: std::collections::HashMap<K, V>,
}

#[flux_rs::generics(K as base, V as base)]
impl<K, V> Regs<K, V> {
    #[flux_rs::trusted]
    #[flux_rs::sig(fn(self: &strg Regs<K,V>[@m], k: K, v: V) ensures self: Regs<K,V>[map_set(m.vals, k, v)])]
    pub fn set(&mut self, k: K, v: V)
    where
        K: Eq + Hash,
    {
        self.inner.insert(k, v);
    }

    #[flux_rs::trusted]
    #[flux_rs::sig(fn(&Regs<K, V>[@m], &K[@k]) -> Option<&V[map_get(m.vals, k)]>)]
    pub fn get(&self, k: &K) -> Option<&V>
    where
        K: Eq + Hash,
    {
        self.inner.get(k)
    }
}
```

### Core Representation

Using `B32` and `Regs`, we can build our core representation:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[flux_rs::refined_by(n: int)]
pub enum GPR {
    #[variant(GPR[0])]
    R0,
    #[variant(GPR[1])]
    R1,
    #[variant(GPR[2])]
    R2,
    #[variant(GPR[3])]
    R3,
    #[variant(GPR[4])]
    R4,
    #[variant(GPR[5])]
    R5,
    #[variant(GPR[6])]
    R6,
    #[variant(GPR[7])]
    R7,
    #[variant(GPR[8])]
    R8,
    #[variant(GPR[9])]
    R9,
    #[variant(GPR[10])]
    R10,
    #[variant(GPR[11])]
    R11,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[flux_rs::refined_by(n : int)]
pub enum SpecialRegister {
    #[variant(SpecialRegister[12])]
    // R12 is used for IPC
    R12,
    #[variant(SpecialRegister[13])]
    Sp,
    #[variant(SpecialRegister[14])]
    Lr,
    #[variant(SpecialRegister[15])]
    Pc,
    #[variant(SpecialRegister[16])]
    Control,
    // PSR and one of the sub register (IPSR)
    #[variant(SpecialRegister[17])]
    PSR,
    #[variant(SpecialRegister[18])]
    IPSR,
}

pub type ArmGeneralRegs = Regs<GPR, B32>;
pub type ArmSpecialRegs = Regs<SpecialRegister, B32>;

#[flux_rs::refined_by(
    general_regs: Map<GPR, B32>,
    special_regs: Map<SpecialRegister, B32>,
    mem: Memory
)]
pub struct Armv7m {
    // General Registers r0 - r11
    #[field(Regs<GPR, B32>[general_regs])]
    pub general_regs: ArmGeneralRegs,
    // Special Registers
    #[field(Regs<SpecialRegister, B32>[special_regs])]
    pub special_regs: ArmSpecialRegs,
    // Memory
    #[field(Memory[mem])]
    pub mem: Memory,
}
```

General and Special purpose registers are represented as a map from enums (that enumerate the possible registers) to a value of type `B32`. Finally, we have a notion of memory, used to reason about memory mapped registers like those for the NVIC (Nested Vector Interrupt Controller). This is implemented as a map from addresses to `B32`. 

```rust
pub type Mem = Regs<u32, B32>;

#[derive(Debug)]
#[flux_rs::refined_by(
    mem: Map<int, B32>
)]
pub struct Memory {
    #[field(Regs<u32, B32>[mem])]
    mem: Mem,
}
```

This representation is overly simplified but works because we restrict the use of `Memory` to reading / writing memory mapped registers. This implementation would need to be changed in the future if we wanted to reason about writing to other addresses.

The memory that is read from / written to is enforced by a series of flux constraints. For example, here is the flux signature on the function used to `read` from addresses.

```rust
#[flux_rs::sig(
    fn (&Memory[@mem], u32[@addr]) -> B32[get_mem_addr(addr, mem)] 
        requires is_valid_read_addr(addr) 
)]
```

Here, `is_valid_read_addr` checks that the address being read from is one we expect. For example, here is a flux function that encodes the valid read addresses for the MPU.

```rust
fn is_valid_mpu_read_addr(address: int) -> bool {
    address == MPU_TYPE_ADDR
        || address == MPU_CTRL_ADDR
        || address == MPU_RNR_ADDR
        || address == MPU_RBAR_ADDR
        || address == MPU_RASR_ADDR
        || address == MPU_RBAR_A1_ADDR
        || address == MPU_RASR_A1_ADDR
        || address == MPU_RBAR_A2_ADDR
        || address == MPU_RASR_A2_ADDR
        || address == MPU_RBAR_A3_ADDR
        || address == MPU_RASR_A3_ADDR
}
```

### Instructions

We can now implement the necessary instructions using our core representation. For example, here is the implementaion of `movw` which moves an immediate word sized value into a general purpose register with no flag updates.

```rust
impl Armv7m {
    // Move Immediate (see p. A7-291 of the manual)
    //
    // Pseudo code provided by arm:
    // if ConditionPassed() then
    //   EncodingSpecificOperations();
    //   result = imm32;
    //   R[d] = result;
    //   if setflags then
    //       APSR.N = result<31>;
    //       APSR.Z = IsZeroBit(result);
    //       APSR.C = carry;
    //       // APSR.V unchanged
    #[flux_rs::sig(
        fn (self: &strg Armv7m[@old_cpu], GPR[@reg], B32[@val]) 
            ensures self: Armv7m { new_cpu: general_purpose_register_updated(reg, old_cpu, new_cpu, val) && new_cpu.special_regs == old_cpu.special_regs && new_cpu.mem == old_cpu.mem }
    )]
    fn update_general_reg_with_b32(&mut self, register: GPR, value: B32) {
        self.general_regs.set(register, value);
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GPR[@reg], B32[@val]) 
        ensures self: Armv7m { 
            new_cpu: 
                general_purpose_register_updated(reg, old_cpu, new_cpu, val) 
                &&
                old_cpu.special_regs == new_cpu.special_regs
                &&
                old_cpu.mem == new_cpu.mem
        }
    )]
    pub fn movw_imm(&mut self, register: GPR, value: B32) {
        // Corresponds to encoding T2 of Mov immediate
        //
        // Specific encoding ops are:
        //      d = UInt(Rd);  setflags = (S == '1');  (imm32, carry) = ThumbExpandImm_C(i:imm3:imm8, APSR.C);
        //      if d IN {13,15} then UNPREDICTABLE;
        //
        // We already know d (register above), setflags is false because no S bit

        // VTOCK TODO:
        // Look at ThumbExpandImm_C
        self.update_general_reg_with_b32(register, value);
    }
}
```

Now, using this instruction and it's corresponding annotation, we can reason about a program. For example...

```rust
#[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu: 
    get_gpr(r0(), new_cpu) == bv32(0)
    &&
    get_gpr(r1(), new_cpu) == bv32(1)
})]
fn two_movs(armv7m: &mut Armv7m) {
    armv7m.movw_imm(GPR::R0, B32::from(0));
    armv7m.movw_imm(GPR::R1, B32::from(1));
}
```

Here we update register `r0` with immediate value `0`, and `r1` with immediate value `1`. Our flux refinement annotation checks that both registers contain the proper values after running the program.

## Next Steps

1. Ensure that our emulation has semantics that are exactly the same as the hardware spec.
    - Certain ops need to be implemented fully (i.e. flag updates and other side effects)
2. Encode register modes & exception handling for Arm.
    - This is important for our ISR reasoning as we need to reason about arbitrary processes being pre-empted by an interrupt
3. Work on ARMv6m and Risc-V 
    -
