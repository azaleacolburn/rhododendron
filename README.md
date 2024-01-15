# rhododendron
Just a C-like compiler for fun
(Rewritten in Rust in one night :P)

Todo:
- [x] Support for declaration,
- [x] Support for while loops and if statements
- [ ] Support for functions
- [ ] Support for structs

This will include non-C features such as
- Built-in Tuple Packing and Unpacking
- Scopes everywhere!
    -sort of like the comma operator in C
- Borrow Checker!
- Rust enums(union-enums)
```
if ({
        statement, 
        statement, 
        statement, 
        condition
    })
{

}
```

As well as not every feature C has(pure unions are useless).

The big question is whether or not I'll write a linker(yes).
