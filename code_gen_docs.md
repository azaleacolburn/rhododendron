- the stack grows down
- r29 is used to store the current stack frame base
- each stack frame base is a pointer to the previous stfb
- the calling convention for functions is:

        sfb
        a*

- when a function is unloaded, it leaves the return value on the top of the stack, where the sfb was

# Issue:

- In if statements, if you reference a variable from the parent scope, it looks up the offset from the previous sfb for the parent scope, then tries to grab the variable by offsetting from the current sfb, not the parent one.
