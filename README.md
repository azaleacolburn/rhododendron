# rhododendron

A compiler for a C-like language (just for fun)

The latest commits on the main branch don't work since I'm refactoring at the moment
Look at the `stable` branch for currently working version.

Currently working on [sonder](https://github.com/azaleacolburn/sonder), where there's a more updated version of the parser.

Todo:

- [x] Variables
- [x] Conditionals
- [x] Pointers / Arrays
- [x] Functions \*
- [ ] Structs
- [ ] AST and code gen optimizations
- [ ] Linker

## Known Issues

4: Unusable

3: Major inconvinience

2: Minor inconvinience

1: "Feature"

### Returning from functions within if statements

Symptom: Results in segmentation fault.

Suspected Cause: Returning from a stack frame to another stack frame that isn't its parent causes a misalignment in the stack.

Potential Fix: Make if statements return in a different way or make them a different kind of frame

Level 3

This is a huge limitation that will be addressed ASAP.
