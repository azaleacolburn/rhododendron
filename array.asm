.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: h, offset: 8 (wrong for arrays)
	; new array
	sub x11, x15, #8; anchor ptr
	
	mov x9, #54
	str x9, [x15, #-8]!
	
	mov x9, #53
	str x9, [x15, #-8]!
	
	mov x9, #52
	str x9, [x15, #-8]!
	
	mov x9, #51
	str x9, [x15, #-8]!
	
	mov x9, #50
	str x9, [x15, #-8]!
	
	mov x9, #49
	str x9, [x15, #-8]!
	
	mov x9, #48
	str x9, [x15, #-8]!
	
	str x11, [x15, #-8]! ; str array anchor TOS
	
	
	; var dec: y, offset: 72 (wrong for arrays)
	; new array
	sub x11, x15, #8; anchor ptr
	
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	; empty array section
	mov x9, #0
	str x9, [x15, #-8]!
	str x11, [x15, #-8]! ; str array anchor TOS
	
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-152]
	str x9, [x15, #-8]!
	mov x9, #0
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
	mov x9, #54
	str x9, [x15, #-8]!
	ldr x10, [x15], #8 ; pop res
	ldr x11, [x15], #8 ; pop adr
	mov x9, x10
	str x9, [x11]
	
	; var dec: j, offset: 160 (wrong for arrays)
	mov x9, #49
	str x9, [x15, #-8]!
	
	ldr x9, [x29, #-152]
	str x9, [x15, #-8]!
	mov x9, #0
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
	
	ldr x9, [x29, #-64]
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
	
	ldr x9, [x29, #-64]
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #2
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #3
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #4
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #5
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #6
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
	
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #55
	str x9, [x15, #-8]!
	ldr x10, [x15], #8 ; pop res
	ldr x11, [x15], #8 ; pop adr
	mov x9, x10
	str x9, [x11]
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #8
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	sub x9, x9, x10
	str x9, [x15, #-8]!
	mov x9, #54
	str x9, [x15, #-8]!
	ldr x10, [x15], #8 ; pop res
	ldr x11, [x15], #8 ; pop adr
	mov x9, x10
	str x9, [x11]
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #2
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
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #3
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
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #4
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
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #5
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
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #6
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
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #0
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #1
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #2
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #3
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #4
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #5
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #6
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
	
	; variable assignment
	
	; deref assignment
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #0
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
	mov x9, #98
	str x9, [x15, #-8]!
	ldr x10, [x15], #8 ; pop res
	ldr x11, [x15], #8 ; pop adr
	mov x9, x10
	str x9, [x11]
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #0
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
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #1
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
	
	ldr x9, [x29, #-160]
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x9, #6
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
	ldr x9, [x29, #-64]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80

.L2:
	; function declaration: print_array
	
	; save link reg
	str lr, [x15, #-8]!
	
	; var dec: i, offset: 24 (wrong for arrays)
	mov x9, #0
	str x9, [x15, #-8]!
	
	
	; while statement
	b .L4

.L3:
	; after while statement scope
	; void function return
	ldr lr, [x29, #-16]
	add x15, x29, #8
	ldr x29, [x29]
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

.L5:
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
	b .L4