use crate::armv7m::lang::GeneralPurposeRegister;

use super::super::Armv7m;

impl Armv7m {
    // BX (see p. A7-215 in the manual)
    //
    // Branch and Exchange causes a branch to an address and instruction set specified by a register. Armv7-M only
    // supports the Thumb instruction set. An attempt to change the instruction Execution state causes the processor to
    // take an exception on the instruction at the target address.
    // BX can also be used for an exception return, see Exception return behavior on page B1-539.
    //
    // Pseudo code provided by arm:
    //
    // if ConditionPassed() then
    //      EncodingSpecificOperations();
    //      BXWritePC(R[m]);

    fn bx_write_pc(&mut self, address: u32) {
        // VTOCK TODO: Implement this with current mode and mode handler etc.
        // BXWritePC(bits(32) address)
        // if CurrentMode == Mode_Handler && address<31:28> == '1111' then
        //     ExceptionReturn(address<27:0>);
        // else
        //     EPSR.T = address<0>;  // if EPSR.T == 0, a UsageFault('Invalid State')
        //                           // is taken on the next instruction
        //     BranchTo(address<31:1>:'0');
    }

    #[flux_rs::trusted]
    pub fn bx(&mut self, register: GeneralPurposeRegister) {
        // Corresponds to Encoding T1
    }
}
