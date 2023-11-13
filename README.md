# rhododendron
Just a C-like compiler for fun
(Rewritten in Rust in one night :P)

test.asm represents the desired ARM64 to be generated from compiling test.txt

References:
- [Compiler Explorer](https://godbolt.org/)

Todo:
- [x] Tokenizing
- [x] Parsing to AST
- [ ] Support for functions and gotos
- [ ] Code Generation

This will include non-C features such as
- Built-in Tuple Packing and Unpacking

As well as not every feature C has.

The big question is whether or not I'll write a linker(yes).

I still haven't decided on a lexical analysis strategy