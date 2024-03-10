- the stack grows down
- r29 is used to store the current stack frame base
- each stack frame base is a pointer to the previous stfb
- the calling convention for functions is: 

        stfb
        ret
        a*


