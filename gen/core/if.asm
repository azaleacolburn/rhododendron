.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: c, offset: 8 (wrong for arrays)
	mov x9, #80
	str x9, [x15, #-8]!
	
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
	
	
	; if statement
	b .L10

.L2:
	; function declaration: check
	
	; save link reg
	str lr, [x15, #-8]!
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
	
	
	; if statement
	b .L4
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L4
	b .L3

.L3:
	; after if statement scope
	; void function return
	ldr lr, [x29, #-24]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L4:
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x9, #21
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L5
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	mov x9, #58
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L5
	b .L3
	b .L3
	b .L3

.L5:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
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
	
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
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
	
	
	; if return
	add x15, x29, #8
	ldr x29, [x29]
	b .L3

.L6:
	; function declaration: test
	
	; save link reg
	str lr, [x15, #-8]!
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
	
	
	; if statement
	b .L8
	mov x9, #80
	str x9, [x15, #-8]!
	mov x9, #80
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L8

.L7:
	b .L7
	; after if statement scope
	mov x9, #116
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	; void function return
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L8:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	
	; var dec: b, offset: 32 (wrong for arrays)
	mov x9, #117
	str x9, [x15, #-8]!
	
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
	
	
	; if return
	add x15, x29, #8
	ldr x29, [x29]
	b .L7
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x9, #80
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L10

.L9:
	b .L9
	; after if statement scope
	mov x9, #111
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; var dec: num, offset: 32 (wrong for arrays)
	mov x9, #58
	str x9, [x15, #-8]!
	
	
	; var dec: c, offset: 40 (wrong for arrays)
	mov x9, #58
	str x9, [x15, #-8]!
	
	mov x9, #111
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
	
	mov x9, #111
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

.L10:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
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
	
	mov x9, #116
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
	b .L9