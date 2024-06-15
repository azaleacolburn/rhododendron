.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	mov x9, #105
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	
	; var dec: y, offset: 8 (wrong for arrays)
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	mov x9, #48
	str x9, [x15, #-8]!
	mov x9, #51
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L5
	; assume ret is TOS
	
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80

.L2:
	; function declaration: basic_recursion_test
	
	; save link reg
	str lr, [x15, #-8]!
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
	
	
	; if statement
	b .L4
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L4

.L3:
	b .L3
	; after if statement scope
	; void function return
	ldr lr, [x29, #-24]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L4:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	ldr x9, [x29, #-8]
	add x9, x9, x10
	str x9, [x29, #-8]
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	
	; if return
	add x15, x29, #8
	ldr x29, [x29]
	b .L3

.L5:
	; function declaration: recursion_return
	
	; save link reg
	str lr, [x15, #-8]!
	
	; if statement
	b .L7
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L7

.L6:
	b .L6
	; after if statement scope
	
	; evaluate return statement and place on stack
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	; function return
	ldr x9, [x15], #8
	ldr lr, [x29, #-24]
	add x15, x29, #8
	ldr x29, [x29]
	str x9, [x15, #-8]!
	ret

.L7:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	ldr x9, [x29, #-8]
	add x9, x9, x10
	str x9, [x29, #-8]
	
	; var dec: y, offset: 48 (wrong for arrays)
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L5
	; assume ret is TOS
	
	
	; evaluate return statement and place on stack
	ldr x9, [x29, #-48]
	str x9, [x15, #-8]!
	; function return
	ldr x9, [x15], #8
	ldr lr, [x29, #-24]
	add x15, x29, #8
	ldr x29, [x29]
	str x9, [x15, #-8]!
	ret
	
	; if return
	add x15, x29, #8
	ldr x29, [x29]
	b .L6