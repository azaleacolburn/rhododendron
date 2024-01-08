.global _main
main:
    str #6, [sp, #-16]
    ldr w0, [sp, #-16]
    str w0, [sp, #-16]
    b .L1
    mov x7, #1
    mov x0, #0
    svc 0

.balign 4
.L1:
    ldr w0, [sp, #-16]
    mov w1, 6
    cmp w0, w1
    beq .L2
    ret

.balign 4
.L2:
    str #1, [sp, #-16]
    ldr w0, [sp, #-32]
    ldr w1, [sp, #-16]
    add w0, w0, w1
    str w0, [sp, #-16]
    b .L3
    ret

.balign 4
.L3:
    str #42, [sp, #-16]
    ldr w0, [sp, #-32]
    str w0, [sp, #-32]
    ret