// Ask Andrew about this, I really don't know where to start
#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include"parser.h"
#include"register.h"

char* expr_code_gen(RegisterTracker* reg, TokenNode* node) {
    char* reg = itoa(assign_register(reg, REG_W));
    char* ret = strcat("mov ", reg);
    free(reg);
    free(ret);
    return ret;
}