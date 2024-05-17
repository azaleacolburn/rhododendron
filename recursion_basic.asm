.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: j, offset: 8 (wrong for arrays)
	mov x9, #106
	str x9, [x15, #-8]!
	
	
	; var dec: k, offset: 16 (wrong for arrays)
	mov x9, #107
	str x9, [x15, #-8]!
	
	
	; var dec: ptr_j, offset: 24 (wrong for arrays)
	; getting the adr of: j
	mov x9, x29
	mov x10, #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	
	
	; var dec: ptr_k, offset: 32 (wrong for arrays)
	; getting the adr of: k
	mov x9, x29
	mov x10, #16
	sub x9, x9, x10
	str x9, [x15, #-8]!
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	mov x9, #48
	str x9, [x15, #-8]!
	mov x9, #51
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
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
	
	
	; if statement
	
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	; void function return
	ldr lr, [x29, #-24]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L3:
	str lr, [x15, #-8]!
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	str lr, [x15, #-8]!
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x29, #-8]
	add x9, x9, x10
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
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	ret