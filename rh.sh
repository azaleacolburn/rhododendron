#cargo run -- tests/core/$1.rh $1.asm
as -arch arm64 -o $1.o $1.asm
ld -o $1 $1.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e .main -arch arm64
rm $1.o
