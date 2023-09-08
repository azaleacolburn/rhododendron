.global _main
.balign 4

_main:
    // str sp, []
    b _print
    b _terminate

/* The byte to be printed is placed on the stack as an arg */
_print:
    mov  x0,  #1                   /* the print part */
    // mov x1,
    adr x1, helloworld
    // mov  x1,  sp                   /* x1    points to the byte to be written */
    mov  x2,  #12
    mov  x16,  #4
    svc  #0

_reboot:
    mov x0, #1 // instant reboot
    mov x16, #55 // reboot
    svc 0 // syscall
_terminate:                              
    // add  sp, sp, #16               /* restore stack before returning */
    mov x0, #0 // return 0
    mov x16, #1 // terminate
    svc 0 // syscall

helloworld: .ascii "hello world\n"