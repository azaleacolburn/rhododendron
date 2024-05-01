.global .main
.align 4

.main:
        sub     sp, sp, #48
        mov     w0, 50
        str     w0, [sp, 44]
        mov     w0, 5
        str     w0, [sp, 40]
        mov     w0, 345
        str     w0, [sp, 36]
        mov     w0, 345
        str     w0, [sp, 32]
        mov     w0, 345
        str     w0, [sp, 28]
        mov     w0, 345
        str     w0, [sp, 24]
        mov     w0, 345
        str     w0, [sp, 20]
        ldr     w0, [sp, 24]
        cmp     w0, 4
        bne     .L2
        mov     w0, 424
        str     w0, [sp, 16]

        ; putchar
	    mov x0, #1 ; stdout
	    mov x1, sp ; put from TOS
        add x1, x1, 16
	    mov x2, #1 ; print 1 char
	    mov x16, #4 ; write
	    svc #0x80
.L2:
        mov     w0, 345
        str     w0, [sp, 12]
        mov     w0, 345
        str     w0, [sp, 8]
        mov     w0, 345
        str     w0, [sp, 4]
        mov     w0, 0
        add     sp, sp, 48
        ret
