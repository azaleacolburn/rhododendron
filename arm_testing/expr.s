.global _main
.balign 4

; .text
_main:
    mov x2, #12
    adr x1, helloworld
    ; mov x5, #3
    ; store args on the stack
    str x2, [sp, #-4]
    str x1, [sp, #-4]
    ; overwrite regs
    ; mov x0, #0 
    ; mov x1, #0
    ; ldr x1, =helloworld
    
    b _print
    ; b _stack_test
    ; b _expr
    ; b _reboot
    b _terminate

_stack_test:
    mov x0, #2
    str x0, [sp, #-8] ; decrement the stack by 4
    mov x0, #3
    ldr x0, [sp], #8 ; increment the stack by 4

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
// In this order on the stack
// This means length first, address second
// - stack: the address of the first byte
// - stack: the number of bytes to be written
_print:
    ldr x1, [sp, #4]
    ldr x2, [sp, #4]
    str x0, [sp, #-4]
    mov  x0,  #1
    adr x1, helloworld
    ; mov  x1,  sp                   // x1 points to the byte to be written
    ; mov  x2,  #12                   // The byte length of the buffer(or sp later)
    mov  x16,  #4                     // syscall will be a print
    svc  #0                           // syscall
    ldr x0, [sp], #4

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