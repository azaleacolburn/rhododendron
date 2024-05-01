.global .main
.align 4

.main:
    ; x29 stores address of sfb
    mov x29, sp
    ; our stack pointer
    mov x30, sp

    mov x19, #49
    mov x20, #50

    str x19, [x30, #-4]!
    str x20, [x30, #-4]!  

    ; putchar
	mov x0, #1 ; stdout
	mov x1, x30; put from TOS
	mov x2, #1 ; print 1 char
	mov x16, #4 ; write
	svc #0x80

	; exit program gracefully
	mov x0, #0
	mov x16, #1
	svc #0x80

