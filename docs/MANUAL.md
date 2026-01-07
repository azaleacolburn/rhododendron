# Rhododendron User Manual

The Rhododendron language is a simple, Turing complete, C based language.

This compiler only targets `arm64-darwin` machines.

# Compiling

To compile a program, run the binary with the path to the file and path to the output file as arguments. The compiler will output assembly code to that file, which will still need to be assembled and linked to run. The file extension for rhododendron programs is `.rh`.

```zsh
cargo run -- test.rh test.asm 
```

See (or just use) the `compile.sh` script to fully compile, assemble, and link a program in one step.

# Variables

Declare and use variables with C syntax:
```c
int num = 4 + 3 * 8;
int status = num / 8;
```

# If Statements

Declare and use while if statements with C syntax:
```c
int num = 0;
if (num < 10) {
    num += 1;
}
// `num` should be 1
```

# While Loops

Declare and use while loops with C syntax:
```c
int num = 0;
while (num < 10) {
    num += 1;
}
// `num` should be 10
```

> [!NOTE]
> `for` loops are currently not supported

# Functions

Declare and use functions with C syntax:
```c
int square(int x) {
    return(x * x)
}

int n2 = square(8);
```

Unlike in C, `main` is not the entrypoint for C code, the main function will not run unless you call it. Additionally, the `ret` assembly instructure will not be included by default. If you want the function to return, state so, otherwise, make the function return `void`.

```c
void main() {
    int num = 9;
    int eighty_one square(num);
}

main();
```

# I/O

Rhododendron does not incluce a linker, so you must use the builtin I/O functions to write to the standard output instead of stdlib.

```c
put('s'); // Will print 's'
put(48); // Will print '0'
put(49); // Will print '1'
```
