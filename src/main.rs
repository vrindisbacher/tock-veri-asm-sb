use std::io::Read;

use capstone::{
    arch::{self, BuildsCapstone, BuildsCapstoneEndian},
    Capstone,
};
use elf::{endian::AnyEndian, section::SectionHeader, ElfBytes};

fn unasm(cs: Capstone, buf: &[u8]) {
    let insns = cs.disasm_all(buf, 0x0).expect("Failed to disassemble");
    println!("Found {} instructions", insns.len());
    for i in insns.as_ref() {
        // println!("{}", i);
        match cs.insn_name(i.id()) {
            None => (),
            Some(name) => {
                let op = i.op_str().unwrap_or("");
                println!("{} {}", name, op);
            }
        }
    }
}

fn read_file_to_bytes(path: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut file = std::fs::File::open(path).unwrap();
    file.read_to_end(&mut buf).unwrap();
    buf
}

fn arm_capstone() -> Capstone {
    Capstone::new()
        .arm()
        .mode(arch::arm::ArchMode::Thumb)
        .endian(capstone::Endian::Little)
        .build()
        .unwrap()
}

fn riscv_capstone() -> Capstone {
    Capstone::new()
        .riscv()
        .mode(arch::riscv::ArchMode::RiscV32)
        .endian(capstone::Endian::Little)
        .build()
        .unwrap()
}

fn unasm_arm() {
    println!("ARM");
    let arm_prgm = read_file_to_bytes("./arm_assem_example.o");
    let arm_cs = arm_capstone();
    unasm(arm_cs, &arm_prgm);
}

fn unasm_riscv() {
    println!("\nRISC-V");
    let riscv_prgm = read_file_to_bytes("./riscv_assem_example.o");
    let file = ElfBytes::<AnyEndian>::minimal_parse(&riscv_prgm).expect("Failed to parse riscv");
    let trap_section_header: SectionHeader = file
        .section_header_by_name(".riscv.trap")
        .expect("section table should be parseable")
        .expect("file should have a .riscv.trap section");

    let (data, _) = file
        .section_data(&trap_section_header)
        .expect("Couldn't get section data");
    let riscv_cs = riscv_capstone();
    unasm(riscv_cs, data);
}

fn main() {
    unasm_arm();
    unasm_riscv();
}
