.global .main
.align 8

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: num, offset: 4 (wrong for arrays)
	mov x9, #58
	str x9, [x15, #-4]!
	
	
	; var dec: c, offset: 8 (wrong for arrays)
	mov x9, #58
	str x9, [x15, #-4]!
	
	
	; if statement
	b .L3

.L2:
	; after if statement scope
	
	; if statement
	b .L6

.L3:
	ldr x9, [x29, #-8]
	str x9, [x15, #-4]!
	ldr x9, [x29, #-4]
	str x9, [x15, #-4]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L4
	b .L2

.L4:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	mov x9, #97
	str x9, [x15, #-4]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; end if
	add x15, x29, #8
	ldr x29, [x29]
	b .L2

.L5:
	; after if statement scope
	
	; if statement
	b .L9

.L6:
	ldr x9, [x29, #-8]
	str x9, [x15, #-4]!
	mov x9, #50
	str x9, [x15, #-4]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L7
	b .L5

.L7:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	mov x9, #117
	str x9, [x15, #-4]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; end if
	add x15, x29, #8
	ldr x29, [x29]
	b .L5

.L8:
	; after if statement scope
	mov x9, #111
	str x9, [x15, #-4]!
	
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

.L9:
	ldr x9, [x29, #-8]
	str x9, [x15, #-4]!
	ldr x9, [x29, #-4]
	str x9, [x15, #-4]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L10
	b .L8
	b .L8

.L10:
	ldr x9, [x29, #-8]
	str x9, [x15, #-4]!
	mov x9, #1
	str x9, [x15, #-4]!
	
	; load from stack
	ldr x10, [x15], #4
	ldr x9, [x15], #4
	add x9, x9, x10
	str x9, [x15, #-4]!
	mov x9, #59
	str x9, [x15, #-4]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L11
	b .L8
	b .L8

.L11:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	mov x9, #116
	str x9, [x15, #-4]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; end if
	add x15, x29, #8
	ldr x29, [x29]
	b .L8