Assembly for section .riscv.trap:

	csrrw s0, mscratch, s0
	beq s0, 0x10
	sw s1, 0(s0)
	lw s1, 4(s0)
	jalr s1
	auipc s0, 0
	addi s0, s0
	bltu s0, sp, 0xc
	auipc sp, 0
	addi sp, sp
	csrrw s0, mscratch, zero
	addi sp, sp, -0x40
	sw ra, 0(sp)
	sw t0, 4(sp)
	sw t1, 8(sp)
	sw t2, 0xc(sp)
	sw t3, 0x10(sp)
	sw t4, 0x14(sp)
	sw t5, 0x18(sp)
	sw t6, 0x1c(sp)
	sw a0, 0x20(sp)
	sw a1, 0x24(sp)
	sw a2, 0x28(sp)
	sw a3, 0x2c(sp)
	sw a4, 0x30(sp)
	sw a5, 0x34(sp)
	sw a6, 0x38(sp)
	sw a7, 0x3c(sp)
	jal 0
	lw ra, 0(sp)
	lw t0, 4(sp)
	lw t1, 8(sp)
	lw t2, 0xc(sp)
	lw t3, 0x10(sp)
	lw t4, 0x14(sp)
	lw t5, 0x18(sp)
	lw t6, 0x1c(sp)
	lw a0, 0x20(sp)
	lw a1, 0x24(sp)
	lw a2, 0x28(sp)
	lw a3, 0x2c(sp)
	lw a4, 0x30(sp)
	lw a5, 0x34(sp)
	lw a6, 0x38(sp)
	lw a7, 0x3c(sp)
	addi sp, sp, 0x40
	mret 

Assembly for section .text.rust_begin_unwind:

	jal 0
