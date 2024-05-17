.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: num, offset: 8 (wrong for arrays)
	mov x9, #20
	str x9, [x15, #-8]!
	
	
	; var dec: c, offset: 16 (wrong for arrays)
	mov x9, #20
	str x9, [x15, #-8]!
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	mov x29, x10
	bl .L5
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

.L2:
	; function declaration: check
	
	; save link reg
	str lr, [x15, #-8]!
	
	; if statement
	
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L3
	
	; void function return
	ldr lr, [x29, #-24]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L3:
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x9, #20
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L4
	
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	mov x9, #21
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L4

.L4:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	str lr, [x15, #-8]!
	
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
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	ret

.L5:
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
	
	mov x9, #80
	str x9, [x15, #-8]!
	mov x9, #80
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	beq .L6
	
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
	            

.L6:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	str lr, [x15, #-8]!
	
	
	; var dec: b, offset: 16 (wrong for arrays)
	mov x9, #117
	str x9, [x15, #-8]!
	
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
	
	
	; if return
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	ret