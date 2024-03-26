.global _main
main:
	str #7, [x29, #-16]!
	ldr x{x}, [x29], #16
	str x0, [x29, #-16]
	str #6, [x29, #-16]!
	ldr x{x}, [x29], #16
	str #7, [x29, #-16]!
	ldr x{x}, [x29], #16
	mul x{x}, x19, x20
	str x{x}, [x29, #-16]!
	ldr x{x}, [x29], #16
	ldr x1, [x29], #1
	str, x1, [x29, #-16]!
	ldr x{x}, [x29], #16
	add x{x}, x19, x20
	str x{x}, [x29, #-16]!
	ldr x{x}, [x29], #16
	str x1, [x29, #-16]
	str #5, [x29, #-16]!
	ldr x{x}, [x29], #16
	str x1, [x29, #-16]
	ret
	mov x7, #1
	mov x0, #0
	svc 0