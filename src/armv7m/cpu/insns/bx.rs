use crate::armv7m::lang::SpecialRegister;

use super::super::Armv7m;

use flux_rs::bitvec::BV32;

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

    #[flux_rs::sig(fn (self: &strg Armv7m[@cpu], BV32[@addr]) ensures self: Armv7m[cpu])]
    fn bx_write_pc(&mut self, address: BV32) {
        // VTOCK TODO: Implement this with current mode and mode handler etc.
        // BXWritePC(bits(32) address)
        // if CurrentMode == Mode_Handler && address<31:28> == '1111' then
        //     ExceptionReturn(address<27:0>);
        // else
        //     EPSR.T = address<0>;  // if EPSR.T == 0, a UsageFault('Invalid State')
        //                           // is taken on the next instruction
        //     BranchTo(address<31:1>:'0');
    }

    #[flux_rs::sig(fn (self: &strg Armv7m[@cpu], SpecialRegister[@reg]) ensures self: Armv7m[cpu])]
    pub fn bx(&mut self, register: SpecialRegister) {
        // Corresponds to Encoding T1
        //
        // Which is simply as BxWritePc op
        let addr = self.get_value_from_special_reg(&register);
        self.bx_write_pc(addr);
    }
}
