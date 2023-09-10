.global _main
.balign 4

_main:
    adr x1, helloworld
    mov x2, #12
    ; b _print
    ; b _expr
    b _reboot
    b _terminate

_expr:
    mov x9, #2
    mov x10, #2
    add x7, x10, x9
    str x7, [x1]
    mov x2, #1
    b _print

// The byte to be printed will be placed on the stack as an arg
// Args: 
// - x1: the bytes to be written
// - x2: the number of bytes to be written
_print:
    mov  x0,  #1                      // the print part 
    // adr x1, helloworld
    // mov  x1,  sp                   // x1 points to the byte to be written
    //mov  x2,  #12                     // The byte length of the buffer(or sp later)
    mov  x16,  #4                     // syscall will be a print
    svc  #0                           // syscall

_reboot:
    mov x0, #1                        // instant reboot
    mov x16, #55                      // reboot the machine
    svc 0                             // syscall
_terminate:                              
    // add  sp, sp, #16               // restore stack before returning
    mov x0, #0                        // return 0
    mov x16, #1                       // terminate
    svc #0                             // syscall

helloworld: .ascii "hello world\n"