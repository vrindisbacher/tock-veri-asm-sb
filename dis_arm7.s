   0:   f04f 0000       mov.w   r0, #0
   4:   f380 8814       msr     CONTROL, r0
   8:   f3bf 8f6f       isb     sy
   c:   f06f 0e06       mvn.w   lr, #6
  10:   f3ef 8005       mrs     r0, IPSR
  14:   f000 00ff       and.w   r0, r0, #255    @ 0xff
  18:   f1a0 0010       sub.w   r0, r0, #16
  1c:   0942            lsrs    r2, r0, #5
  1e:   2301            movs    r3, #1
  20:   f000 001f       and.w   r0, r0, #31
  24:   fa03 f000       lsl.w   r0, r3, r0
  28:   4b03            ldr     r3, [pc, #12]   @ (38 <generic_isr_arm_v7m+0x38>)
  2a:   f843 0022       str.w   r0, [r3, r2, lsl #2]
  2e:   4b03            ldr     r3, [pc, #12]   @ (3c <generic_isr_arm_v7m+0x3c>)
  30:   f843 0022       str.w   r0, [r3, r2, lsl #2]
  34:   4770            bx      lr
  36:   0000            movs    r0, r0
  38:   e000e180        .word   0xe000e180
  3c:   e000e200        .word   0xe000e200
