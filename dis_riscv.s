00000000 <_start_trap>:
   0:   34041473                .insn   4, 0x34041473
   4:   00040863                beqz    s0,14 <.Lpcrel_hi0>
   8:   00942023                sw      s1,0(s0)
   c:   00442483                lw      s1,4(s0)
  10:   00048067                jr      s1

00000014 <.Lpcrel_hi0>:
  14:   00000417                auipc   s0,0x0
  18:   00040413                mv      s0,s0
  1c:   00246663                bltu    s0,sp,28 <.Lpcrel_hi1+0x8>

00000020 <.Lpcrel_hi1>:
  20:   00000117                auipc   sp,0x0
  24:   00010113                mv      sp,sp
  28:   34001473                .insn   4, 0x34001473
  2c:   fc010113                add     sp,sp,-64 # ffffffe0 <.Lpcrel_hi1+0xffffffc0>
  30:   00112023                sw      ra,0(sp)
  34:   00512223                sw      t0,4(sp)
  38:   00612423                sw      t1,8(sp)
  3c:   00712623                sw      t2,12(sp)
  40:   01c12823                sw      t3,16(sp)
  44:   01d12a23                sw      t4,20(sp)
  48:   01e12c23                sw      t5,24(sp)
  4c:   01f12e23                sw      t6,28(sp)
  50:   02a12023                sw      a0,32(sp)
  54:   02b12223                sw      a1,36(sp)
  58:   02c12423                sw      a2,40(sp)
  5c:   02d12623                sw      a3,44(sp)
  60:   02e12823                sw      a4,48(sp)
  64:   02f12a23                sw      a5,52(sp)
  68:   03012c23                sw      a6,56(sp)
  6c:   03112e23                sw      a7,60(sp)
  70:   000000ef                jal     70 <.Lpcrel_hi1+0x50>
  74:   00012083                lw      ra,0(sp)
  78:   00412283                lw      t0,4(sp)
  7c:   00812303                lw      t1,8(sp)
  80:   00c12383                lw      t2,12(sp)
  84:   01012e03                lw      t3,16(sp)
  88:   01412e83                lw      t4,20(sp)
  8c:   01812f03                lw      t5,24(sp)
  90:   01c12f83                lw      t6,28(sp)
  94:   02012503                lw      a0,32(sp)
  98:   02412583                lw      a1,36(sp)
  9c:   02812603                lw      a2,40(sp)
  a0:   02c12683                lw      a3,44(sp)
  a4:   03012703                lw      a4,48(sp)
  a8:   03412783                lw      a5,52(sp)
  ac:   03812803                lw      a6,56(sp)
  b0:   03c12883                lw      a7,60(sp)
  b4:   04010113                add     sp,sp,64
  b8:   30200073                mret
