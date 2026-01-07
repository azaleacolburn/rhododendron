This is not currently complete documentation

# Notes
- The stack grows down
- `r29` is used to store the current stack frame base
- Each stack frame base is a pointer to the previous stfb
- The calling convention for functions is:

        sfb
        args*

- When a function is unloaded, it leaves the return value on the top of the stack, where the sfb was

# Known Issue:
