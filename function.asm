.global .main
.align 4

.main:
	; x29 is our sfb
	; x28 is our sp
	mov x29, sp
	mov x28, sp
	
	; variable declaration
	mov x19, #50
	str x19, [x28, #-4]!
	
	; variable declaration
	mov x19, #49
	str x19, [x28, #-4]!
	
	ldr x19, [x29]
	str x19, [x28, #-4]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x28 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80

	ldr x19, [x29, #-4]
	str x19, [x28, #-4]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x28 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80
