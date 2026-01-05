.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: auto_ptr, offset: 8 (wrong for arrays)
	; new array
	sub x11, x15, #8; anchor ptr
	
	mov x9, #97
	str x9, [x15, #-8]!
	
	mov x9, #117
	str x9, [x15, #-8]!
	
	mov x9, #116
	str x9, [x15, #-8]!
	
	mov x9, #111
	str x9, [x15, #-8]!
	
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	str x11, [x15, #-8]! ; str array anchor TOS
	
	
	; var dec: desk_ptr, offset: 56 (wrong for arrays)
	; new array
	sub x11, x15, #8; anchor ptr
	
	mov x9, #100
	str x9, [x15, #-8]!
	
	mov x9, #101
	str x9, [x15, #-8]!
	
	mov x9, #115
	str x9, [x15, #-8]!
	
	mov x9, #107
	str x9, [x15, #-8]!
	
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	str x11, [x15, #-8]! ; str array anchor TOS
	
	
	; var dec: auto_len, offset: 104 (wrong for arrays)
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-48]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	; assume ret is TOS
	
	ldr x9, [x29, #-104]
	str x9, [x15, #-8]!
	mov x9, #48
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
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-48]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L6
	
	; var dec: mo, offset: 112 (wrong for arrays)
	; new array
	sub x11, x15, #8; anchor ptr
	
	mov x9, #109
	str x9, [x15, #-8]!
	
	mov x9, #111
	str x9, [x15, #-8]!
	
	str x11, [x15, #-8]! ; str array anchor TOS
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-48]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-128]
	str x9, [x15, #-8]!
	mov x9, #2
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
	ldr x9, [x29, #-48]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L6
	
	; var dec: desk_len, offset: 136 (wrong for arrays)
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-96]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	; assume ret is TOS
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-96]
	str x9, [x15, #-8]!
	mov x9, #114
	str x9, [x15, #-8]!
	ldr x9, [x29, #-136]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L14
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-96]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L6
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80

.L2:
	; function declaration: strlen
	
	; save link reg
	str lr, [x15, #-8]!
	
	; var dec: i, offset: 24 (wrong for arrays)
	mov x9, #0
	str x9, [x15, #-8]!
	
	
	; while statement
	b .L4

.L3:
	; after while statement scope
	
	; evaluate return statement and place on stack
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	; function return
	ldr x9, [x15], #8
	ldr lr, [x29, #-16]
	add x15, x29, #8
	ldr x29, [x29]
	str x9, [x15, #-8]!
	ret

.L4:
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-24]
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
	mov x9, #0
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L5
	b .L3
	b .L3

.L5:
	; scope of while statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	str lr, [x15, #-8]!
	
	
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-24]
	ldr x29, [x15], #8
	add x9, x9, x10
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	str x9, [x29, #-24]
	ldr x29, [x15], #8
	
	; while return
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	b .L4

.L6:
	; function declaration: strput
	
	; save link reg
	str lr, [x15, #-8]!
	
	; var dec: i, offset: 24 (wrong for arrays)
	mov x9, #0
	str x9, [x15, #-8]!
	
	
	; while statement
	b .L8

.L7:
	; after while statement scope
	; void function return
	ldr lr, [x29, #-16]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L8:
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-24]
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
	mov x9, #0
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L9
	b .L7
	b .L7

.L9:
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
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-24]
	ldr x29, [x15], #8
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
	
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-24]
	ldr x29, [x15], #8
	add x9, x9, x10
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	str x9, [x29, #-24]
	ldr x29, [x15], #8
	
	; while return
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	b .L8

.L10:
	; function declaration: memset
	
	; save link reg
	str lr, [x15, #-8]!
	
	; var dec: i, offset: 40 (wrong for arrays)
	mov x9, #0
	str x9, [x15, #-8]!
	
	
	; while statement
	b .L12

.L11:
	; after while statement scope
	; void function return
	ldr lr, [x29, #-32]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L12:
	ldr x9, [x29, #-40]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L13
	b .L11
	b .L11

.L13:
	; scope of while statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	str lr, [x15, #-8]!
	
	
	; variable assignment
	
	; deref assignment
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-40]
	ldr x29, [x15], #8
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
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-16]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-40]
	ldr x29, [x15], #8
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
	ldr x10, [x15], #8 ; pop res
	ldr x11, [x15], #8 ; pop adr
	mov x9, x10
	str x9, [x11]
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-40]
	ldr x29, [x15], #8
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
	
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-40]
	ldr x29, [x15], #8
	add x9, x9, x10
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	str x9, [x29, #-40]
	ldr x29, [x15], #8
	
	; while return
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	b .L12

.L14:
	; function declaration: fill
	
	; save link reg
	str lr, [x15, #-8]!
	
	; var dec: i, offset: 40 (wrong for arrays)
	mov x9, #0
	str x9, [x15, #-8]!
	
	
	; while statement
	b .L16

.L15:
	; after while statement scope
	; void function return
	ldr lr, [x29, #-32]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L16:
	ldr x9, [x29, #-40]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L17
	b .L15
	b .L15

.L17:
	; scope of while statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	str lr, [x15, #-8]!
	
	
	; variable assignment
	
	; deref assignment
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-40]
	ldr x29, [x15], #8
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
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-16]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	ldr x10, [x15], #8 ; pop res
	ldr x11, [x15], #8 ; pop adr
	mov x9, x10
	str x9, [x11]
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-40]
	ldr x29, [x15], #8
	add x9, x9, x10
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	str x9, [x29, #-40]
	ldr x29, [x15], #8
	
	; while return
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	b .L16

.L18:
	; function declaration: strcat
	
	; save link reg
	str lr, [x15, #-8]!
	
	; var dec: len_one, offset: 40 (wrong for arrays)
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	; assume ret is TOS
	
	
	; var dec: len_two, offset: 48 (wrong for arrays)
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	; assume ret is TOS
	
	
	; var dec: i, offset: 56 (wrong for arrays)
	mov x9, #0
	str x9, [x15, #-8]!
	
	
	; while statement
	b .L20

.L19:
	; after while statement scope
	; variable assignment
	mov x9, #0
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	str x9, [x29, #-56]
	
	; while statement
	b .L23

.L20:
	ldr x9, [x29, #-56]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-40]
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L21
	b .L19
	b .L19

.L21:
	; scope of while statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	str lr, [x15, #-8]!
	
	
	; variable assignment
	
	; deref assignment
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-56]
	ldr x29, [x15], #8
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
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-16]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-56]
	ldr x29, [x15], #8
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
	ldr x10, [x15], #8 ; pop res
	ldr x11, [x15], #8 ; pop adr
	mov x9, x10
	str x9, [x11]
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-56]
	ldr x29, [x15], #8
	add x9, x9, x10
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	str x9, [x29, #-56]
	ldr x29, [x15], #8
	
	; while return
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	b .L20

.L22:
	; after while statement scope
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L6
	; void function return
	ldr lr, [x29, #-32]
	add x15, x29, #8
	ldr x29, [x29]
	ret
	            

.L23:
	ldr x9, [x29, #-56]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-48]
	str x9, [x15, #-8]!
	mov x9, #1
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	add x9, x9, x10
	str x9, [x15, #-8]!
	ldr x9, [x15], #8
	ldr x10, [x15], #8
	cmp x9, x10
	bne .L24
	b .L22
	b .L22

.L24:
	; scope of while statement
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x29, x15
	str lr, [x15, #-8]!
	
	
	; variable assignment
	
	; deref assignment
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-8]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-56]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-40]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	add x9, x9, x10
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
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-24]
	ldr x29, [x15], #8
	str x9, [x15, #-8]!
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-56]
	ldr x29, [x15], #8
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
	ldr x10, [x15], #8 ; pop res
	ldr x11, [x15], #8 ; pop adr
	mov x9, x10
	str x9, [x11]
	; variable assignment
	mov x9, #1
	str x9, [x15, #-8]!
	ldr x10, [x15], #8
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	ldr x9, [x29, #-56]
	ldr x29, [x15], #8
	add x9, x9, x10
	
	; getting var from prev scope: 1
	str x29, [x15, #-8]!
	ldr x29, [x29]
	str x9, [x29, #-56]
	ldr x29, [x15], #8
	
	; while return
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	b .L23