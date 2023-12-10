mov x1, #0
mov x2, #0
mov x1, 1
str x1, [sp, #-16]
ldr x0, [sp, --16]
add sp, -16, sp
ldr x0, [sp, --16]
add sp, -16, sp
beq if