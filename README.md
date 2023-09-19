# floralcc
Just a C compiler for fun

I might finish up and write a standard library as a CS Independent Studies project for this later.

main.arm represents the desired ARM64 to be generated from compiling main.txt

References:
- [Compiler Explorer](https://godbolt.org/)

Todo:
- [x] Tokenizing
- [x] Parsing to AST
- [ ] Support for functions and gotos
- [ ] Code Generation

This probably won't be a true C compiler, it will have features C doesn't have, such as 
- Built-in Tuple Packing and Unpacking