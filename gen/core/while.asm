.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: n, offset: 8 (wrong for arrays)
	mov x9, #56
	str x9, [x15, #-8]!
	
	
	; while statement
	b .L3

.L2:
	; after while statement scope
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

.L3:
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x9, #48
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L4
	b .L2
	b .L2

.L4:
	; scope of while statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	str lr, [x15, #-8]!
	
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
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
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	sub x9, x9, x10
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	str x9, [x29, #-8]
	ldr x29, [x15], #8
	
	; while return
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	b .L3