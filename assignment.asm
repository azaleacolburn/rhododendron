.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: h, offset: 8 (wrong for arrays)
	mov x9, #48
	str x9, [x15, #-8]!
	
	
	; var dec: u, offset: 16 (wrong for arrays)
	mov x9, #53
	str x9, [x15, #-8]!
	
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
	
	; variable assignment
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	str x9, [x29, #-8]
	
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
	
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80