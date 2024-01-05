.global _main
ain:
    str #1, [sp, #16]
    ldr w0, [sp, #-16]
    str w0, [sp, #-16]
    ldr w0, [sp, #-16]
    mov w1, 2
    cmp w0, w1
    beq .L1
    str #4, [sp, #16]
    ldr w0, [sp, #-32]
    str w0, [sp, #-32]
    ldr w0, [sp, #-32]
    ldr w1, [sp, #-16]
    cmp w0, w1
    beq .L1
    ret
.balign 4
    .L1:
    str #3, [sp, #16]
    ldr w0, [sp, #-32]
    str w0, [sp, #-16]
    ret
.balign 4
    .L2:
    str #5, [sp, #16]
    ldr w0, [sp, #-48]
    str w0, [sp, #-16]
    ret
