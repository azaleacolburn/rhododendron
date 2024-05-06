.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; variable declaration: y
	mov x9, #121
	str x9, [x15, #-8]!
	
	
	; variable declaration: t
	mov x9, #9
	str x9, [x15, #-8]!
	
	
	; variable declaration: o
	mov x9, #111
	str x9, [x15, #-8]!
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	
	; variable declaration: z
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	mov x9, #8
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L3
	; assume ret is TOS
	
	ldr x9, [x29, #-80]
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80

.L2:
	; function declaration
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; unload stack
	mov x15, x29
	add x15, x15, #8
	ldr x29, [x29]
	ret

.L3:
	; function declaration
	
	; evaluate return statement and place on stack
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x10, [x29, #-8]
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	mul x9, x9, x10
	str x9, [x15, #-8]!
	
	; ldr expr into x9
	ldr x9, [x15], #8
	
	; reset sfb
	mov x15, x29
	ldr x29, [x29]
	str x9, [x15, #-8]!
	ret
	
	; unload stack
	mov x15, x29
	add x15, x15, #8
	ldr x29, [x29]
	ret