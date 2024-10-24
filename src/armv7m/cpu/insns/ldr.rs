use crate::armv7m::lang::GeneralPurposeRegister;

use super::super::Armv7m;

impl Armv7m {
    // LDR (literal) see p. A7-248 in the manual
    //
    // Load Register (literal) calculates an address from the PC value and an immediate offset, loads a word from memory,
    // and writes it to a register. See Memory accesses on page A7-184 for information about memory accesses.
    //
    // Here is the pseudo code supplied by ARM:
    //
    // if ConditionPassed() then
    //   EncodingSpecificOperations();
    //   base = Align(PC,4);
    //   address = if add then (base + imm32) else (base - imm32);
    //   data = MemU[address,4];
    //   if t == 15 then
    //      if address<1:0> == '00' then LoadWritePC(data); else UNPREDICTABLE;
    //   else
    //      R[t] = data;

    #[flux_rs::trusted]
    pub fn pseudo_ldr(&mut self, register: GeneralPurposeRegister, value: u32) {
        // Note the non pseudo instruction would do this:
        //
        //      fn align(value: u32, alignment: u32) -> u32 {
        //          alignment * (value / alignment)
        //      }
        //      let base = Self::align(self.pc, 4);
        //      let addr = base + value;
        //      let data = self.mem.read(addr);
        //      VTOCK TODO: Deal with PC update here
        //      self.update_general_reg_with_u32(register, data);
        //
        // but since dealing with offsets to the PC isn't supported right
        // now we'll just encode the pseudo instruction as a mov
        self.movw_imm(register, value);
    }
}