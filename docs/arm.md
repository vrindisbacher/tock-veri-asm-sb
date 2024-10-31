# Armv7m Status

Most of the work done so far has been on the Arm Thumb Instruction Set used by Cortex M devices. Below are details on our Rust / Flux representation of this architecture.

At a high level, we are currently able to...

1. Prove an assembly program using flux annotations (on a subset of Thumb instructions)
2. Prove that flux annotations for these programs are correct given our emulation

Importantly, our emulation has semantics which are taken from the ARM hardware spec. 

## A Non Trivial Program

Here is an example of a non trivial program we are able to verify: 

```rust
#[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu]) ensures self: Armv7m { new_cpu:
    general_purpose_register_updated(
        r0(), 
        old_cpu,
        new_cpu, 
        // bv32 and to_int convert from ints to bitvectors and back
        bv32(to_int(get_special_reg(ipsr(), old_cpu)) % 32)
    )
})]
fn ipsr_mod_32(armv7m: &mut Armv7m) {
    // r0 = ipsr
    // mrs meaning 'move register special' - moves a special register into a general purpose register
    armv7m.mrs(GeneralPurposeRegister::R0, SpecialRegister::IPSR);
    // r0 = r0 & 31
    // and_imm meaning 'and immediate' - computes the bitwise and of a general purpose register and an immediate
    // value, storing the result in the general purpose register
    armv7m.and_imm(GeneralPurposeRegister::R0, B32::from(31));
}
```

Our annotations check that register `r0` has been updated to `ipsr % 32`. Here, `ipsr` is a sub register of the `psr` (Program Status Register) and is computed as `psr & 0xff`.

These are non trivial operations in the sense that they say more than "I can move a value into a register". Hopefully, this motivates the fact that our framework will eventually be able to reason about complex programs.

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

pub type ArmGeneralRegs = Regs<GeneralPurposeRegister, B32>;
pub type ArmSpecialRegs = Regs<SpecialRegister, B32>;

#[flux_rs::refined_by(
    general_regs: Map<GeneralPurposeRegister, B32>,
    special_regs: Map<SpecialRegister, B32>,
    mem: Memory
)]
pub struct Armv7m {
    // General Registers r0 - r11
    #[field(Regs<GeneralPurposeRegister, B32>[general_regs])]
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
        fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], B32[@val]) 
            ensures self: Armv7m { new_cpu: general_purpose_register_updated(reg, old_cpu, new_cpu, val) && new_cpu.special_regs == old_cpu.special_regs && new_cpu.mem == old_cpu.mem }
    )]
    fn update_general_reg_with_b32(&mut self, register: GeneralPurposeRegister, value: B32) {
        self.general_regs.set(register, value);
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@old_cpu], GeneralPurposeRegister[@reg], B32[@val]) 
        ensures self: Armv7m { 
            new_cpu: 
                general_purpose_register_updated(reg, old_cpu, new_cpu, val) 
                &&
                old_cpu.special_regs == new_cpu.special_regs
                &&
                old_cpu.mem == new_cpu.mem
        }
    )]
    pub fn movw_imm(&mut self, register: GeneralPurposeRegister, value: B32) {
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
    get_general_purpose_reg(r0(), new_cpu) == bv32(0)
    &&
    get_general_purpose_reg(r1(), new_cpu) == bv32(1)
})]
fn two_movs(armv7m: &mut Armv7m) {
    armv7m.movw_imm(GeneralPurposeRegister::R0, B32::from(0));
    armv7m.movw_imm(GeneralPurposeRegister::R1, B32::from(1));
}
```

Here we update register `r0` with immediate value `0`, and `r1` with immediate value `1`. Our flux refinement annotation checks that both registers contain the proper values after running the program.

## Next Steps

1. Ensure that our emulation has semantics that are exactly the same as the hardware spec.
    - Certain ops need to be implemented fully (i.e. flag updates and other side effects)
2. Verify the actual ARMv7 interrupt service routine for tock
    - This has been a challenge because of performance issues with Z3 and bitvectors. However, our refactor to use the `B32` should help quite a bit.
3. Encode register modes & exception handling for Arm.
    - This is important for our ISR reasoning as we need to reason about arbitrary processes being pre-empted by an interrupt
