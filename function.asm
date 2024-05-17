.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
	; var dec: y, offset: 8 (wrong for arrays)
	mov x9, #97
	str x9, [x15, #-8]!
	
	
	; var dec: t, offset: 16 (wrong for arrays)
	mov x9, #117
	str x9, [x15, #-8]!
	
	
	; var dec: o, offset: 24 (wrong for arrays)
	mov x9, #116
	str x9, [x15, #-8]!
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-16]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-24]
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L4
	
	; var dec: j, offset: 32 (wrong for arrays)
	mov x9, #111
	str x9, [x15, #-8]!
	
	
	; var dec: d, offset: 40 (wrong for arrays)
	mov x9, #100
	str x9, [x15, #-8]!
	
	
	; var dec: e, offset: 48 (wrong for arrays)
	mov x9, #101
	str x9, [x15, #-8]!
	
	
	; var dec: s, offset: 56 (wrong for arrays)
	mov x9, #115
	str x9, [x15, #-8]!
	
	
	; var dec: k, offset: 64 (wrong for arrays)
	mov x9, #107
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
	
	ldr x9, [x29, #-40]
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-48]
	str x9, [x15, #-8]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	; unload the TOS
	add x15, x15, #8
	
	ldr x9, [x29, #-56]
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
	mov x9, #10
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L2
	; assume ret is TOS
	
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
	; function declaration: square
	
	; save link reg
	str lr, [x15, #-8]!
	
	; evaluate return statement and place on stack
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	mul x9, x9, x10
	str x9, [x15, #-8]!
	        
	; function return
	ldr x9, [x15], #8
	ldr lr, [x29, #-16]
	add x15, x29, #8
	ldr x29, [x29]
	str x9, [x15, #-8]!
	ret

.L3:
	; function declaration: count
	
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
	
	
	; place old sfb
	str x29, [x15, #-8]!
	mov x10, x15
	ldr x9, [x29, #-8]
	str x9, [x15, #-8]!
	mov x9, #1
	str x9, [x15, #-8]!
	
	; load from stack
	ldr x10, [x15], #8
	ldr x9, [x15], #8
	add x9, x9, x10
	str x9, [x15, #-8]!
	mov x29, x10
	bl .L3

.L4:
	; function declaration: putchars
	
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
	
	ldr x9, [x29, #-24]
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
	ldr lr, [x29, #-32]
	add x15, x29, #8
	ldr x29, [x29]
	ret