- the stack grows down
- r29 is used to store the current stack frame base
- each stack frame base is a pointer to the previous stfb
- the calling convention for functions is:

        sfb
        a*

- when a function is unloaded, it leaves the return value on the top of the stack, where the sfb was

# Known Issue:
