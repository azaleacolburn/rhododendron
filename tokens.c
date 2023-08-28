#include<stdio.h>
#include<stdlib.h>
#include"tokens.h"

Token* new_token(TokType type) {
    Token* tok = malloc(sizeof(Token));
    tok->type = type;
    tok->value = NULL;
    return tok;
};

void print_tok_type(TokType type) {
    // printf("hereis\n"); 
    char* p;
    switch (type) {
        case TOK_ADD:
            p = "+";
            break;
        case TOK_ADD_EQ:
            p = "+=";
            break;
        case TOK_ASSIGN: // should this even exist
            p = "assignment";
            break;
        case TOK_EQ:
            p = "=";
            break;
        case TOK_B_AND:
            p = "&";
            break;
        case TOK_B_AND_EQ:
            p = "&=";
            break;
        case TOK_B_OR: 
            p = "|";
            break;
        case TOK_B_OR_EQ:
            p = "|=";
            break;
        case TOK_B_XOR:
            p = "^";
            break;
        case TOK_B_XOR_EQ:
            p = "^=";
            break;
        case TOK_C_CURL:
            p = "}";
            break;
        case TOK_C_PAREN:
            p = ")";
            break;
        case TOK_CONDITION:
            p = "condition: ";
            break;
        case TOK_DECLARE:
            p = "declaration";
            break;
        case TOK_DIV:
            p = "/";
            break;
        case TOK_DIV_EQ:
            p = "/=";
            break;
        case TOK_EQ_CMP:
            p = "==";
            break;
        case TOK_FOR:
            p = "for";
            break;
        case TOK_ID:
            p = "id: ";
            break;
        case TOK_IF:
            p = "if";
            break;
        case TOK_MUL:
            p = "*";
            break;
        case TOK_MUL_EQ:
            p = "*=";
            break;
        case TOK_NEQ_CMP:
            p = "!=";
            break;
        case TOK_NONE:
            p = "";
            break;
        case TOK_NOT:
            p = "!";
            break;
        case TOK_NUM:
            p = "num: ";
            break;
        case TOK_O_CURL:
            p = "{";
            break;
        case TOK_O_PAREN:
            p = "(";
            break;
        case TOK_PROGRAM:
            p = "program start: ";
            break;
        case TOK_STATEMENT:
            p = "statement: ";
            break;
        case TOK_SUB:
            p = "-";
            break;
        case TOK_SUB_EQ:
            p = "-=";
            break;
        case TOK_WHILE:
            p = "while";
            break;
        case TOK_SEMI:
            p = ";";
            break;
        case TOK_ADDRESS:
            p = "*";
            break;
        case TOK_VAR:
            p = "var";
            break;
    }
    printf("Token Type: %s\n", p);
}

void print_token(Token* tok) {
    // tok->type segfaults
    // printf("type: %d\n", tok->type);
    // printf("hherelol\n");
    if (tok == NULL) printf("null token\n"); 
     // or here
    print_tok_type(tok->type); // seg fault here
    // printf("her3e\n");
    if (tok->value != NULL)
        printf("value: %s\n", (char*)tok->value);    
};

// Figure out how to check the contents of the vector. Maybe length. goto may lead to something
void free_token(Token* root) {
    free(root->value);
    free(root);
}