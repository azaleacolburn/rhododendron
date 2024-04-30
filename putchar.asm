.global .main
;; Provide program starting address to linker
.align 4            
 
 .main:
    mov x19, #49
    str x19, [sp, #-4]! ; store 49 on stack
    bl .putchar
    bl .exit

; char arg placed on top of stack
.putchar:
    mov x0, #1 ; stdout
    mov x1, sp ; pointer to TOS
    mov x2, #1 ; 1 char long
    mov x16, #4 ; write
    svc #0x80
    ret
    
.exit:
    mov x0, #0 ; return code
    mov x16, #1 ; sys_call num to terminate
    svc #0x80

helloworld:      .ascii  "Hello World!\n"
