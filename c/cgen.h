#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include<math.h>
#include"register.h"

char* code_gen(TokenNode* parent);

Vec* declare_code_gen(TokenNode* parent, RegisterTracker* tracker);

char* itoa(int num, char* buffer, int base);

Vec* expr_code_gen(TokenNode* parent, RegisterTracker* tracker);