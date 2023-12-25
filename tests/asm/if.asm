.start
    mov x1, #5
    str x1, [sp, #-16]
    ldr x1, [sp, #16]
    mov x2, #2
    cmp x1, x2
    beq .L1
    mov x2, #4
    str x2, [sp, #-16] ; c
    ldr x1, [sp, #16] ; c
    ldr x2, [sp, #16] ; num
    cmp x1, x2
    beq .L2
    ret
    
.L1
    mov x1, #3
    str x1, [sp, #-16]
    ret

.L2
    mov x1, #5
    str x1, [sp, #-16]
    ret
