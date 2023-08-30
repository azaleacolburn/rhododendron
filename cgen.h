#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include<math.h>
#include"register.h"

// We ned register assignment I think
// We need arithmatic stuff
// We need to read the AST

char* declr_code_gen(RegisterTracker* reg, TokenNode* node);

char* itoa(int num, char* buffer, int base);