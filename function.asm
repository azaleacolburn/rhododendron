.global _main
main:
	mov x29, sp
	mov x19, 0
	str x19, [x29, #-16]!
	ldr x20, [x29], #16
	str x20, [x29, #-16]!
	str x29, [sp, #-32]!
	ldr x29, sp
	ldr x20, [x29]
	b .L1
	mov x7, #1
	mov x0, #0
	svc 0

.balign 4
.L1:
	ldr x20, [x29], #20
	str x20, [x29, #-16]!
	ldr x19, [x29], #16
	ldr x19, [x29], #20
	str x19, [x29, #-16]!
	ldr x20, [x29], #16
	mul x20, x19, x20
	str x20, [x29, #-16]!
	ldr x19, [x29], #16
	str x19, [x29, #-16]!
	ret