// Ask Andrew about this, I think I'm doing this wrong
#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include"parser.h"
#include"register.h"

#define add_comma(ret) ret = strcat(ret, ", ")
// Inits new "ret"
#define mov(reg, val) \
    char* ret = strcat("mov ", reg); \
    ret = strcat(ret, ", "); \
    ret = strcat(ret, val); \

// generates a single mov statement
// recurses or calls other functions if it needs to evaluate an expression
// recursion is key
char* declr_code_gen(RegisterTracker* reg, TokenNode* node) {
    char* reg = itoa(assign_register(reg, REG_W));
    mov(reg, (char*)node->token->value);
    printf("ret: %s", ret);
    free(reg);
    // free(ret);
    return ret;
}