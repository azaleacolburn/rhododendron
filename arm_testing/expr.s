.global _main
.balign 4

; .text
_main:
    adr x0, helloworld
    mov x1, #12
    ; store them on the stack
    str x0, [sp, #-8]
    str x1, [sp, #-8]
    ; overwrite regs
    mov x0, #0 
    mov x1, #0
    ; ldr x1, =helloworld
    
    b _print
    ; b _stack
    ; b _expr
    ; b _reboot
    b _terminate

_stack:
    mov x0, #2
    str x0, [sp, #-8]! ; decrement the stack by 4
    mov x0, #3
    ldr x0, [sp], #8 ; increment the stak by 4

_expr:
    mov x9, #2
    mov x10, #2
    add x7, x10, x9
    mov x1, #1
    str x7, [x1]
    mov x2, #1
    b _print

// The byte to be printed will be placed on the stack as an arg
// Args: 
// - stac: the bytes to be written
// - stack: the number of bytes to be written
_print:
    ; mov  x0,  #1                      // the print part 
    ldr x1, [sp, #8]
    ldr x0, [sp, #-8]
    // adr x1, helloworld
    // mov  x1,  sp                   // x1 points to the byte to be written
    // mov  x2,  #12                     // The byte length of the buffer(or sp later)
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

; .data
helloworld: .ascii "hello world\n"