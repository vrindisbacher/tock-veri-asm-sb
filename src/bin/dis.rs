use std::io::Read;

use capstone::{
    arch::{self, BuildsCapstone},
    Capstone, Endian,
};
use elf::{endian::AnyEndian, ElfBytes};

fn unasm(cs: Capstone, buf: &[u8], section_name: &str) {
    let insns = cs.disasm_all(buf, 0x0).expect("Failed to disassemble");
    if insns.len() > 0 {
        println!("\nAssembly for section {}:\n", section_name);
    }
    for i in insns.as_ref() {
        match cs.insn_name(i.id()) {
            None => (),
            Some(name) => {
                let op = i.op_str().unwrap_or("");
                println!("\t{} {}", name, op);
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

fn arm_mclass_capstone() -> Capstone {
    Capstone::new_raw(
        capstone::Arch::ARM,
        capstone::Mode::Thumb,
        vec![capstone::ExtraMode::MClass].into_iter(),
        Some(Endian::Little),
    )
    .unwrap()
}

fn riscv_capstone() -> Capstone {
    Capstone::new()
        .riscv()
        .mode(arch::riscv::ArchMode::RiscV32)
        .build()
        .unwrap()
}

fn unasm_arm() {
    println!("ARM");
    let arm_prgm = read_file_to_bytes("./arm_assem_example.o");
    let file = ElfBytes::<AnyEndian>::minimal_parse(&arm_prgm).expect("Failed to parse arm");
    let all_headers = file
        .section_headers_with_strtab()
        .expect("Expected section headers");
    let header_names = all_headers.1.expect("Expected header names");
    let headers = all_headers.0.expect("Expected headers");
    for header in headers {
        let (data, _) = file
            .section_data(&header)
            .expect("Couldn't get section data");
        let arm_cs = arm_mclass_capstone();
        let section_name = header_names
            .get(header.sh_name as usize)
            .expect("Could not find header name");
        unasm(arm_cs, data, section_name);
    }
}

fn unasm_riscv() {
    println!("\nRISC-V");
    let riscv_prgm = read_file_to_bytes("./riscv_assem_example.o");
    let file = ElfBytes::<AnyEndian>::minimal_parse(&riscv_prgm).expect("Failed to parse riscv");
    let all_headers = file
        .section_headers_with_strtab()
        .expect("Expected section headers");
    let header_names = all_headers.1.expect("Expected header names");
    let headers = all_headers.0.expect("Expected headers");
    for header in headers {
        let (data, _) = file
            .section_data(&header)
            .expect("Couldn't get section data");
        let riscv_cs = riscv_capstone();
        let section_name = header_names
            .get(header.sh_name as usize)
            .expect("Could not find header name");
        unasm(riscv_cs, data, section_name);
    }
}

fn main() {
    unasm_arm();
    unasm_riscv();
}
