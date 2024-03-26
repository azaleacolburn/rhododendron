.global _main
main:
	mov x29, sp
	mov x19, 7
	str x19, [x29, #-16]!
	ldr x20, [x29], #16
	str x20, [x29, #-16]!
	mov x20, 6
	str x20, [x29, #-16]!
	ldr x19, [x29], #16
	mov x19, 7
	str x19, [x29, #-16]!
	ldr x20, [x29], #16
	mul x20, x19, x20
	str x20, [x29, #-16]!
	ldr x19, [x29], #16
	ldr x19, [x29], #4
	str x19, [x29, #-16]!
	ldr x20, [x29], #16
	add x20, x19, x20
	str x20, [x29, #-16]!
	ldr x19, [x29], #16
	str x19, [x29, #-16]!
	mov x19, 5
	str x19, [x29, #-16]!
	ldr x20, [x29], #16
	str x20, [x29, #-16]!
	ldr x20, [x29], #20
	str x20, [x29, #-16]!
	ldr x19, [x29], #16
	mov x19, 5
	str x19, [x29, #-16]!
	ldr x20, [x29], #16
	add x20, x19, x20
	str x20, [x29, #-16]!
	ldr x19, [x29], #16
	str x19, [x29, 36]
	mov x7, #1
	mov x0, #0
	svc 0