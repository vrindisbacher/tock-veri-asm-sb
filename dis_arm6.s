   0:   4815            ldr     r0, [pc, #84]   @ (58 <generic_isr+0x58>)
   2:   4586            cmp     lr, r0
   4:   d110            bne.n   28 <generic_isr+0x28>
   6:   4669            mov     r1, sp
   8:   6849            ldr     r1, [r1, #4]
   a:   610c            str     r4, [r1, #16]
   c:   614d            str     r5, [r1, #20]
   e:   618e            str     r6, [r1, #24]
  10:   61cf            str     r7, [r1, #28]
  12:   b4f0            push    {r4, r5, r6, r7}
  14:   4644            mov     r4, r8
  16:   464d            mov     r5, r9
  18:   4656            mov     r6, sl
  1a:   465f            mov     r7, fp
  1c:   600c            str     r4, [r1, #0]
  1e:   604d            str     r5, [r1, #4]
  20:   608e            str     r6, [r1, #8]
  22:   60cf            str     r7, [r1, #12]
  24:   bcf0            pop     {r4, r5, r6, r7}
  26:   480b            ldr     r0, [pc, #44]   @ (54 <generic_isr+0x54>)
  28:   f3ef 8005       mrs     r0, IPSR
  2c:   21ff            movs    r1, #255        @ 0xff
  2e:   4008            ands    r0, r1
  30:   3810            subs    r0, #16
  32:   4a07            ldr     r2, [pc, #28]   @ (50 <generic_isr+0x50>)
  34:   0943            lsrs    r3, r0, #5
  36:   009b            lsls    r3, r3, #2
  38:   189b            adds    r3, r3, r2
  3a:   221f            movs    r2, #31
  3c:   4010            ands    r0, r2
  3e:   3a1e            subs    r2, #30
  40:   4082            lsls    r2, r0
  42:   601a            str     r2, [r3, #0]
  44:   3320            adds    r3, #32
  46:   601a            str     r2, [r3, #0]
  48:   4770            bx      lr
  4a:   46c0            nop                     @ (mov r8, r8)
  4c:   46c0            nop                     @ (mov r8, r8)
  4e:   46c0            nop                     @ (mov r8, r8)
  50:   e000e180        .word   0xe000e180
  54:   fffffff9        .word   0xfffffff9
  58:   fffffffd        .word   0xfffffffd
