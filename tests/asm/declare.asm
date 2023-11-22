mov x1, #0
mov x2, #0
mov x1, 7
str x1, [sp, #-16]
mov x1, #0
mov x2, #0
mov x1, 42
ldr x2,= ; stack position
add x1, x1, x2
str x1, [sp, #-16]