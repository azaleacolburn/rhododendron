.global _main
main:
    str #7, [sp, #-16]
    ldr w0, [sp, #-16]
    str w0, [sp, #-16]
    str #6, [sp, #-16]
    ldr w0, [sp, #-32]
    str #7, [sp, #-16]
    ldr w0, [sp, #-32]
    ldr w0, [sp, #0]
    ldr w1, [sp, #16]
    mul w0, w0, w1
    str w0, [sp, #0]
    ldr w1, [sp, #0]
    ldr w1, [sp, #-16]
    str, w1, [sp, #0]
    ldr w0, [sp, #0]
    ldr w0, [sp, #32]
    ldr w1, [sp, #48]
    add w0, w0, w1
    str w0, [sp, #32]
    ldr w1, [sp, #32]
    str w0, [sp, #32]
    str #5, [sp, #-16]
    ldr w1, [sp, #16]
    str w0, [sp, #16]
    ret
    mov x7, #1
    mov x0, #0
    svc 0