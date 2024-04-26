.global _main
main:
	mov x29, sp
	mov x19, 1
	str x19, [x29, #-4]!
	ldr x19, [x29], #4
	str x19, [x29, #-4]!
	mov x19, 4
	str x19, [x29, #-4]!
	ldr x19, [x29], #4
	str x19, [x29, #-4]!
	ldr x19, [x29, #20]
	ldr x20, [x29, #4]
	cmp x19, x20
	beq .L1
	ldr x19, [x29, #4]
	mov x20, 3
	cmp x19, x20
	beq .L1
	mov x7, #1
	mov x0, #0
	svc 0

.balign 4
.L1:
	mov x19, 5
	str x19, [x29, #-4]!
	ldr x19, [x29], #4
	ldr x20, [x29, #4]
	xor x19, x19, x20
	str x19, [x29, #4]!