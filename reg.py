
addr = 0xE000E100
for i in range(0, 16):
    print(f"{addr:#08x} => Some(self.ipr{i}),")
    addr += 4
