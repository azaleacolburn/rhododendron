.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; variable declaration
	mov x9, #121
	str x9, [x15, #-8]!
	
	
	; variable declaration
	mov x9, #9
	str x9, [x15, #-8]!
	
	
	; variable declaration
	mov x9, #111
	str x9, [x15, #-8]!
	
	
	; place old sfb
	mov x10, x15
	str x29, [x15, #-8]!
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80

.L2:
	; function declaration
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
	
	ldr x9, [x29, #-32]
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
	; x15 <- x29
	; x29 <- &old_sfb
	mov x15, x29
	ldr x29, [x29]
	ret