.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: t, offset: 8 (wrong for arrays)
	mov x9, #54
	str x9, [x15, #-8]!
	
	
	; var dec: o, offset: 16 (wrong for arrays)
	mov x9, #55
	str x9, [x15, #-8]!
	
	
	; var dec: ptr, offset: 24 (wrong for arrays)
	; getting the adr of: t
	sub x9, x29, #8
	str x9, [x15, #-8]!
	
	
	; var dec: ptr_two, offset: 32 (wrong for arrays)
	; getting the adr of: o
	sub x9, x29, #16
	str x9, [x15, #-8]!
	
	
	; var dec: h, offset: 40 (wrong for arrays)
	; new array
	sub x11, x15, #8; anchor ptr
	
	mov x9, #48
	str x9, [x15, #-8]!
	
	mov x9, #49
	str x9, [x15, #-8]!
	
	mov x9, #50
	str x9, [x15, #-8]!
	
	str x11, [x15, #-8]! ; str array anchor TOS
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #0
	str x9, [x15, #-8]!
	mov x9, #1
	str x9, [x15, #-8]!
	mov x9, #3
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L6
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
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	mov x9, #54
	str x9, [x15, #-8]!
	mov x9, #56
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L10
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
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-32]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L15
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
	
	mov x9, #108
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
	; function declaration: put_array
	
	; save link reg
	str lr, [x15, #-8]!
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x9, #8
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	mul x9, x9, x10
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
	
	
	; if statement
	b .L4

.L3:
	; after if statement scope
	; void function return
	ldr lr, [x29, #-32]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L4:
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	mov x9, #1
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L5
	b .L3

.L5:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-16]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	mov x9, #1
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	add x9, x9, x10
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-24]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	
	; end if
	add x15, x29, #8
	ldr x29, [x29]
	b .L3

.L6:
	; function declaration: increment_array
	
	; save link reg
	str lr, [x15, #-8]!
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	mov x9, #8
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	mul x9, x9, x10
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8 ; pop res
	ldr x11, [x15], #8 ; pop adr
	ldr x9, [x11]
	add x9, x9, x10
	str x9, [x11]
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	mov x9, #8
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	mul x9, x9, x10
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
	
	
	; if statement
	b .L8

.L7:
	; after if statement scope
	; void function return
	ldr lr, [x29, #-40]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L8:
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-32]
	str x9, [x15, #-8]!
	mov x9, #1
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L9
	b .L7

.L9:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-16]
	ldr x29, [x15], #8
	add x9, x9, x10
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	str x9, [x29, #-16]
	ldr x29, [x15], #8
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-16]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-24]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-32]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L6
	
	; end if
	add x15, x29, #8
	ldr x29, [x29]
	b .L7

.L10:
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
	b .L12

.L11:
	; after if statement scope
	; void function return
	ldr lr, [x29, #-24]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L12:
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L13
	b .L11

.L13:
	; scope of if statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	add x9, x9, x10
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	str x9, [x29, #-8]
	ldr x29, [x15], #8
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-16]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L10
	
	; end if
	add x15, x29, #8
	ldr x29, [x29]
	b .L11

.L14:
	; function declaration: put_ptr
	
	; save link reg
	str lr, [x15, #-8]!
	ldr x9, [x29, #-8]
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
	
	; void function return
	ldr lr, [x29, #-16]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L15:
	; function declaration: nested_call_test
	
	; save link reg
	str lr, [x15, #-8]!
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L14
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L14
	; void function return
	ldr lr, [x29, #-24]
	add x15, x29, #8
	ldr x29, [x29]
	ret