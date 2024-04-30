.global .main
.align 4

.main:
	; rhododendron programs use x29 for stack frame base
	mov x29, sp
	
	; variable declaration
	mov x19, #50
	str x19, [sp, #-4]!
	
	ldr x19, [x29, #4]
	str x19, [sp, #-4]!
	
	; putchar
	mov x0, #1 ; stdout
	mov x1, sp ; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80
	;str xzr, [sp, #4]! ; free stack memory
	
	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80