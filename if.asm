.global _main
main:
    mov w0, #1
    str w0, [sp, #-16]
    ldr w0, [sp, #-16]
    mov w1, 2
    cmp w0, w1
    beq .L1
    mov w0, #4
    str w0, [sp, #-32]
    ldr w0, [sp, #-32]
    ldr w1, [sp, #-16]
    cmp w0, w1
    beq .L1
    ret
.balign 4
    .L1:
    mov w0, #3
    str w0, [sp, #-16]
    ret
.balign 4
    .L2:
    mov w0, #5
    str w0, [sp, #-16]
    ret
