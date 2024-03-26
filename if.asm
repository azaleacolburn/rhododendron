.global _main
main:
	mov x29, sp
	mov x19, 1
	str x19, [x29, #-16]!
	ldr x20, [x29], #16
	str x20, [x29, #-16]!
	ldr w20, [sp, #4]
	mov w19, 2
	cmp x19, x20
	bleq .L1
	ret
	mov x7, #1
	mov x0, #0
	svc 0

.balign 4
.L1:
	mov x20, 3
	str x20, [x29, #-16]!
	ldr x19, [x29], #16
	str x19, [x29, 4]
	ret
	mov x19, 4
	str x19, [x29, #-16]!
	ldr x20, [x29], #16
	str x20, [x29, #-16]!
	ldr w20, [sp, #20]
	ldr w19, [sp, #4]
	cmp x19, x20
	bleq .L2
	ret

.balign 4
.L2:
	mov x20, 5
	str x20, [x29, #-16]!
	ldr x19, [x29], #16
	str x19, [x29, 4]
	ret