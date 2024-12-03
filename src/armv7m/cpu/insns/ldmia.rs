use crate::armv7m::{cpu::Armv7m, lang::GPR};

impl Armv7m {
    // flattening the list into 8 regs because we know this is r4 - r11
    pub fn ldmia_w(
        &mut self,
        rd: GPR,
        rm1: GPR,
        rm2: GPR,
        rm3: GPR,
        rm4: GPR,
        rm5: GPR,
        rm6: GPR,
        rm7: GPR,
        rm8: GPR,
    ) {
        let mut addr = self.get_value_from_general_reg(&rd).into();
        self.mem.write(addr, self.get_value_from_general_reg(&rm1));
        addr -= 0x4;
        self.mem.write(addr, self.get_value_from_general_reg(&rm2));
        addr -= 0x4;
        self.mem.write(addr, self.get_value_from_general_reg(&rm3));
        addr -= 0x4;
        self.mem.write(addr, self.get_value_from_general_reg(&rm4));
        addr -= 0x4;
        self.mem.write(addr, self.get_value_from_general_reg(&rm5));
        addr -= 0x4;
        self.mem.write(addr, self.get_value_from_general_reg(&rm6));
        addr -= 0x4;
        self.mem.write(addr, self.get_value_from_general_reg(&rm7));
        addr -= 0x4;
        self.mem.write(addr, self.get_value_from_general_reg(&rm8));
    }
}
