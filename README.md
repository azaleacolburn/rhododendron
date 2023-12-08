# rhododendron
Just a C-like compiler for fun
(Rewritten in Rust in one night :P)

Todo:
- [x] Lexing
- [x] Parsing to AST
- [x] Code Generation
- [ ] Support for functions and gotos

This will include non-C features such as
- Built-in Tuple Packing and Unpacking
- Scopes!
    - sort of like the comma operator in C
c```
if ({
        // scope
        statement, 
        statement, 
        statement, 
        condition
    }) {
    //scope
}
```

As well as not every feature C has.

The big question is whether or not I'll write a linker(yes).