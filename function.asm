.global _main
main:
	; rhododendron programs use x29 as a sp
	mov x29, sp
	mov x19, 8
	str x19, [x29, #-4]!
	ldr x19, [x29], #4
	str x19, [x29, #-4]!
	
	; function declaration
	str x29, [x29, #-4]!
	ldr x20, [x29]
	str #0, [x29, #-4]!
	b .L1
	
	; exit program gracefully
	mov x7, #1
	mov x0, #0
	svc 0

.balign 4
.L1:
	ldr x19, [x29], #32
	str x19, [x29, #-4]!
	ldr x19, [x29], #4
	ldr x20, [x29], #32
	str x20, [x29, #-4]!
	ldr x20, [x29], #4
	mul x19, x19, x20
	str x19, [x29, #-4]!
	ldr x19, [x29], #4
	str x19, [x29, #-4]!
	ldr x19, [x29, #84]
	ldr x20, [x29, #32]
	cmp x19, x20
	beq .L2
	mov x19, 1
	str x19, [x29, #-4]!
	ldr x19, [x29], #4
	str x19, [x29, #84]!
	ldr x19, [x29], #4
	mov x29, [x29]
	ret

.balign 4
.L2:
	mov x19, 0
	str x19, [x29, #-4]!
	ldr x19, [x29], #4
	str x19, [x29, #84]!