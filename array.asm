.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: h, offset: 8 (wrong for arrays)
	; new array
	mov x11, x15 ; anchor ptr
	sub x11, x11, #8
	mov x9, #54
	str x9, [x15, #-8]!
	
	mov x9, #53
	str x9, [x15, #-8]!
	
	mov x9, #52
	str x9, [x15, #-8]!
	
	mov x9, #51
	str x9, [x15, #-8]!
	
	str x11, [x15, #-8]! ; str array anchor TOS
	
	
	; var dec: j, offset: 48 (wrong for arrays)
	mov x9, #49
	str x9, [x15, #-8]!
	
	ldr x9, [x29, #-40]
	str x9, [x15, #-8]!
	; deref expr
	ldr x9, [x15], #8
	ldr x10, [x9]
	str x10, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-40]
	str x9, [x15, #-8]!
	mov x9, #8
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	; deref expr
	ldr x9, [x15], #8
	ldr x10, [x9]
	str x10, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-40]
	str x9, [x15, #-8]!
	mov x9, #16
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	; deref expr
	ldr x9, [x15], #8
	ldr x10, [x9]
	str x10, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-40]
	str x9, [x15, #-8]!
	mov x9, #24
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	; deref expr
	ldr x9, [x15], #8
	ldr x10, [x9]
	str x10, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	mov x9, #50
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-48]
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