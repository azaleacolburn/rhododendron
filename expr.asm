.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	mov x9, #7
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	mov x9, #10
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; var dec: my_mul, offset: 16
	mov x9, #4
	str x9, [x15, #-8]!
	mov x10, #7
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	mul x9, x9, x10
	str x9, [x15, #-8]!
	
	
	; var dec: my_sub, offset: 24
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	mov x10, #15
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	
	
	; var dec: my_div, offset: 32
	mov x9, #60
	str x9, [x15, #-8]!
	mov x10, #5
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	udiv x9, x9, x10
	str x9, [x15, #-8]!
	
	
	; var dec: my_sum, offset: 40
	ldr x9, [x29, #-40]
	str x9, [x15, #-8]!
	mov x10, #60
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	add x9, x9, x10
	str x9, [x15, #-8]!
	
	
	; var dec: my_res, offset: 48
	ldr x9, [x29, #-48]
	str x9, [x15, #-8]!
	ldr x10, [x29, #-32]
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	add x9, x9, x10
	str x9, [x15, #-8]!
	
	
	; var dec: my_int, offset: 56
	mov x9, #60
	str x9, [x15, #-8]!
	mov x10, #5
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	udiv x9, x9, x10
	str x9, [x15, #-8]!
	mov x10, #60
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	add x9, x9, x10
	str x9, [x15, #-8]!
	mov x9, #4
	str x9, [x15, #-8]!
	mov x10, #7
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	mul x9, x9, x10
	str x9, [x15, #-8]!
	mov x10, #15
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	add x9, x9, x10
	str x9, [x15, #-8]!
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	mov x9, #10
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-56]
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	mov x9, #10
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	mov x9, #47
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
	mov x10, #48
	str x10, [x15, #-8]!
	
	; load from stack
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	add x9, x9, x10
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