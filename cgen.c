// Ask Andrew about this, I really don't know where to start
#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include"parser.h"
#include"register.h"

char* expr_code_gen(RegisterTracker* reg, TokenNode* node) {
    // Ok so this is hyper specific
    char* reg = itoa(assign_register(reg, REG_W));
    char* ret = strcat("mov ", reg);
    ret = strcat(ret, ", ");
    ret = strcat(ret, (char*)node->token->value);
    printf("ret: %s", ret);
    free(reg);
    free(ret);
    return ret;
}