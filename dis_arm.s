Assembly for section .strtab:

	cmp r6, #0
	str r4, [r6, #0x54]
	strb r0, [r7, #0x11]
	cmp r6, #0
	ldr r3, [r4, #0x74]
	ldr r5, [r5, #0x54]
	ldr r5, [r4, #0x64]
	lsl r4, r6, #1
	asr r6, r5
	ldr r5, [pc, #0x148]
	str r6, [r5, #0x10]
	strb r4, [r6, #0x11]
	ldr r2, [r6, #0x14]
	strb r2, [r4, #0x15]
	str r4, [r6, #0x54]
	lsl r3, r6, #1
	str r6, [r5, #0x70]
	ldr r5, [r4, #0x64]
	strb r5, [r4, #9]
	str r1, [r5, #0x34]
	ldr r7, [r3, #0x14]
	strb r3, [r6, #9]
	str r7, [r3, #0x14]
	ldr r2, [r6, #0x54]
	strb r7, [r3, #0x19]
	ldr r7, [r6, #0x50]
	cmp r6, #0
	ldr r6, [r5, #0x74]
	str r4, [r6, #0x54]
	bxns r5
	strb r6, [r1, r5]
	strb r5, [r5, #0xc]
	str r4, [r6, #0x14]
	ldr r3, [r4, #0x34]
	cmp r6, #0
	str r2, [r6, #0x54]
	cmp r6, #0x6c
	strh r1, [r0, r1]
	cmp r6, #0x4d
	ldrb r5, [r4, #1]
	str r1, [r5, #0x44]
	cmp r6, #0x78
	str r4, [r6, #0x54]
	strb r0, [r7, #0x11]
	strb r6, [r5, #8]
	strb r5, [r6, #0xd]
	ldrsh r4, [r6, r5]
	str r2, [r4, #0x54]
	ldr r7, [r4, #0x14]
	ldrsh r6, [r5, r5]
	ldr r5, [r6, #0x64]
	ldr r7, [r6, #0x14]
	str r6, [r5, #0x44]
	cmp r6, #0
	strb r3, [r6, #0x11]
	strb r2, [r6, #0x11]
	str r1, [r4, #0x24]
	cmp r6, #0
	ldrb r3, [r6, #5]
	strb r5, [r5, #0x11]
	str r1, [r4, #0x24]
	mov r4, #0
	cmp r6, #0x64
	movs r2, r6
	strb r4, [r4, #0x10]
	add r1, #0x2e
	str r0, [r0, #0x10]
	ldr r2, [r6, #0x54]
	str r7, [r3, #0x14]
	strb r3, [r6, #0xd]
	ldr r5, [r4, #0x54]
	sub r0, #0x2e
	str r5, [r4, #0x54]
	add r3, #0x38
	sub r1, #0x33
	add r4, #0x62
	str r5, [r4, #0x54]
	str r0, [r7, #0x10]
	str r2, [r6, #0x40]
	cmp r5, #0x38
	str r3, [r4, #0x74]
	cmp r6, #0x75
	movs r0, r6
	strb r4, [r4, #0x10]
	add r0, #0x2e

Assembly for section .generic_isr_arm_v7m:

	mov r0, #0
	msr control, r0
	isb sy
	mvn lr, #6
	mrs r0, ipsr
	and r0, r0, #0xff
	sub r0, r0, #0x10
	lsr r2, r0, #5
	mov r3, #1
	and r0, r0, #0x1f
	lsl r0, r3, r0
	ldr r3, [pc, #0xc]
	str r0, [r3, r2, lsl #2]
	ldr r3, [pc, #0xc]
	str r0, [r3, r2, lsl #2]
	bx lr
	movs r0, r0
	b #0x33c
	b #0x3e
	b #0x440
	b #0x42

Assembly for section .text.rust_begin_unwind:

	b #0

Assembly for section .ARM.exidx.text.rust_begin_unwind:

	movs r0, r0
	movs r0, r0
	movs r1, r0
	movs r0, r0

Assembly for section .rel.ARM.exidx.text.rust_begin_unwind:

	movs r0, r0
	movs r0, r0
	lsl r2, r5, #0xc
	movs r0, r0

Assembly for section .comment:

	strb r0, [r0, #8]
	strb r5, [r6, #0xd]
	str r4, [r6, #0x34]
	strb r0, [r4, #0x18]
	strb r5, [r4, #9]
	ldr r3, [r6, #0x14]
	ldr r7, [r5, #0x64]
	add r1, #0x20
	sub r0, #0x2e
	cmp r6, #0x31
	mov r0, #0x30
	str r0, [r5, #0x50]
	str r5, [r4, #0x24]
	add r0, #0x39
	str r3, [r4, #0x44]
	add r1, #0x61
	add r2, #0x20
	add r2, #0x30
	cmp r5, #0x34
	sub r1, #0x30
	add r0, #0x2d
	cmp r1, #0x34

Assembly for section .ARM.attributes:

	add r1, #0x41
	movs r0, r0
	str r0, [r0, #0x10]
	str r5, [r4, #0x14]
	ldr r2, [r4, #0x14]
	lsl r0, r0, #4
	movs r7, r4
	movs r0, r0
	add r2, #0x43
	add r0, #0x2e
	movs r1, r7
	lsr r6, r0, #8
	ldr r5, [pc, #0x1c]
	movs r0, r1
	lsl r1, r1, #8
	movs r6, r1
	lsl r1, r2, #4
	lsl r4, r2, #4
	lsl r5, r2, #4
	lsl r7, r2, #0xc
	lsl r0, r3, #4
	lsl r1, r3, #4
	lsl r6, r3, #8
	lsl r2, r4, #4
	lsl r6, r4, #4

Assembly for section .symtab:

	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	lsl r5, r0, #2
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r4, r0
	vcge d16, d22, #0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r3, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r3, r0
	movs r4, r0
	lsl r0, r0, #2
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r4, r0
	lsl r3, r7, #1
	movs r0, r0
	movs r0, r7
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r3, r0
	movs r1, r4
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r0
	movs r0, r2
	movs r3, r0
	lsl r1, r3, #1
	movs r0, r0
	movs r1, r0
	movs r0, r0
	movs r2, r0
	movs r0, r0
	lsl r2, r2, #8
	movs r4, r0
