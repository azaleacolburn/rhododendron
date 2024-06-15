.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	mov x9, #57
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
	mov x29, x10
	bl .L2
	; assume ret is TOS
	
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
	
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80

.L2:
	; function declaration: base
	
	; save link reg
	str lr, [x15, #-8]!
	
	; evaluate return statement and place on stack
	mov x9, #50
	str x9, [x15, #-8]!
	; function return
	ldr x9, [x15], #8
	ldr lr, [x29, #-8]
	add x15, x29, #8
	ldr x29, [x29]
	str x9, [x15, #-8]!
	ret