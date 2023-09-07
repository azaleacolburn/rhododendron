.global _main
.balign 4
_main:
    mov w0, 1
    mov w1, 1

    ret

print:
    mov  x0,  #1                   /* the print part */
    mov  x1,  sp                   /* XXX x1 points to the byte to be written */
    mov  x2,  #1
    mov  w8,  #64
    svc  #0
exit:                                  /* XXX */
    add  sp, sp, #16               /* XXX restore stack before returning */
    ret  