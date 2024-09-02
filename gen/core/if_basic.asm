.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: y, offset: 8 (wrong for arrays)
	mov x9, #10
	str x9, [x15, #-8]!
	
	
	; if statement
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x9, #10
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L3

.L2:
	; after if statement scope
	
	; if statement

.L3:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	mov x9, #97
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; if return
	add x15, x29, #8
	ldr x29, [x29]
	b .L2
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x9, #10
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L5

.L4:
	; after if statement scope
	mov x9, #117
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

.L5:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	mov x9, #100
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; if return
	add x15, x29, #8
	ldr x29, [x29]
	b .L4