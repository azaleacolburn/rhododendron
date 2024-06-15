.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: n, offset: 8 (wrong for arrays)
	mov x9, #7
	str x9, [x15, #-8]!
	
	
	; var dec: num, offset: 16 (wrong for arrays)
	mov x9, #6
	str x9, [x15, #-8]!
	mov x9, #7
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	mul x9, x9, x10
	str x9, [x15, #-8]!
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	add x9, x9, x10
	str x9, [x15, #-8]!
	
	
	; var dec: u, offset: 24 (wrong for arrays)
	mov x9, #5
	str x9, [x15, #-8]!
	
	; variable assignment
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	mov x9, #5
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	add x9, x9, x10
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	str x9, [x29, #-24]
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80