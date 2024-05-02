.global .main
.align 4

.main:
	; x29 is our sfb
	;x15 is our sp
	mov x29, sp
	mov x15, sp
	
    #sub x15, x15, #4
    ;sub x15, x15, #8

	; variable declaration
	mov x9, #50
	str x9, [x15, #-4]!
	
	; variable declaration
	mov x9, #49
	str x9, [x15, #-4]!

	ldr x9, [x29, #-8]
	str x9, [x15, #-4]!

	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
    
    ldr x9, [x29, #-12]
	str x9, [x15, #-4]!
    
	; putchar
	mov x0, #1 ; stdout
	mov x1, x15 ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80
