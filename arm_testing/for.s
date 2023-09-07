.loop:
    mov w0, 0
    mov w1, 1
    add w2, w1
    ble w2

main: 
    jmp: .loop

string: .asciz "Hello World\n"