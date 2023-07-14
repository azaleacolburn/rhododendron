#include<string.h>
#include<stdlib.h>
#include<stdio.h>
#include"rdp.h"

#define is_num(c) ((c) >= '0' && (c) <= '9')
#define is_letter(c) ((c) <= 'a' && (c) <= 'z' || (c) >= 'A' && (c) <= 'Z')
#define is_typename(s) (is_char(s) || is_int(s))
#define is_char(s) (strcmp(s, "char"))
#define is_int(s) (strcmp(s, "int"))
#define is_keyword(s) (strpbrk(s, keywords))
#define is_expr(t) (t == TOK_ADD, t == TOK_DIV, t == TOK_SUB, t == TOK_MUL, t == TOK_NUM, t == TOK_ID)

static char* keywords = "if for while int char ()"; // This is a bug that will disallow things like wh and ch as names

Error program(char* string) {
    printf("here with content:\n%s\n", string);
    Token* program_tok = new_token(TOK_PROGRAM);
    Tokenizer* t = malloc(sizeof(Tokenizer));
    t->cursor = 0;
    t->string = string;
    // Every starting token should have a function here
    return declare(t, program_tok);
}

Error declare(Tokenizer* t, Token* program_tok) {
    printf("declare\n");
    char* str_token = get_next_token(*t);
    if (strcmp(str_token, "\0")) {
        return ERR_NONE;
    }
    printf("after getting token\n");
    if (is_typename(str_token)) {
        Token* dec_tok = new_token(TOK_DECLARE);
        if (is_char(str_token)) 
            push_vec(dec_tok->children, (void*)TYPE_CHAR);
        else if (is_int(str_token))
            push_vec(dec_tok->children, (void*)TYPE_INT);
        printf("Declaration found and token pushed to tree\nMoved on to ident");
        var_id(t, dec_tok);
        if (*get_next_token(*t) == '=') { // Needs to be updating for operations like +=
            return expr(t, dec_tok);
        } else {
            return ERR_MISSING_DECLARATION;
        }
    } else {
        printf("No declaration found");
        return ERR_TYPE;
    }
}

Error var_id(Tokenizer* t, Token* dec_tok) {
    int type = *(int*)get_vec(dec_tok->children, 0);
    if (type == TYPE_CHAR || type == TYPE_INT) {
        char* str_token = get_next_token(*t);
        if (!is_keyword(str_token)) {
            Token* id_tok = new_token(TOK_ID);
            push_vec(id_tok->children, str_token);
            return ERR_NONE;
        } else {
            return ERR_KEYWORD_PLACEMENT;
        }
    } else {
        return ERR_TYPE;
    }
}

// Todo: Allow variable ids
// Ideas: keep a vector of legal ids
Error expr(Tokenizer* t, Token* parent) {
    Tok parent_type = parent->type;
    // Token* expr_tok = malloc(sizeof(Token));
    // expr_tok->type = TOK_EXPR;
    // expr_tok->children = new_vec(0);
    char* str_token = get_next_token(*t);
    if (*str_token == ';') {
        return 0;
    }
    if (!is_keyword(str_token)) {
        if (is_num(*str_token)) {
            Token* num_tok = new_token(TOK_NUM);
            push_vec(num_tok->children, str_token);
            push_vec(parent->children, num_tok);
            return 1;
        } else {
            Token* tok;
            if (*str_token == '+')
                tok = new_token(TOK_ADD);
            else if (*str_token == '-')
                tok = new_token(TOK_SUB);
            else if (*str_token == '*')
                tok = new_token(TOK_MUL);
            else if (*str_token == '/')
                tok = new_token(TOK_DIV);
            else
                return 1;
            expr(t, tok);
            push_vec(parent->children, tok);
            return 0;
        }
    } else {
        return 1;
    }
}

void print_token(Token* tok) {
    printf("Token Type: %d\n", tok->type);
    printf("Token children: ");
    for (int i = 0; i < tok->children->len; i++) {
        print_token(get_vec(tok->children, i));
    }
    printf("Token printing complete\n");
}


Token* new_token(Tok type) {
    Token* tok = malloc(sizeof(Token));
    tok->type = type;
    tok->children = new_vec(2);
    return tok;
}

void print_tok(Tok type) {
    switch (type) {
        case TOK_PROGRAM:
            printf("Program\n");
            break;
        case TOK_DECLARE:
            printf("Delcaration\n");
            break;
        case TOK_NUM:
            printf("Num\n");
            break;
        case TOK_ADD:
            printf("+\n");
            break;
        case TOK_MUL:
            printf("*\n");
            break;
        case TOK_SUB:
            printf("-\n");
            break;
        case TOK_DIV:
            printf("/\n");
            break;
        case TOK_ASSIGN:
            printf("Assignment\n");
            break;
        case TOK_ID:
            printf("Id\n");
            break;
    }
}

// AST parse(char* string) {
//     Vec* toks = new_vec(10);
//     Tokenizer* t = malloc(sizeof(Tokenizer));
//     t->string = string;
//     t->cursor = 0;
//     char* str_token = get_next_token(*t);
//     if (str_token != '\0') {
//         return toks;
//     } else if (is_num(str_token))  {
//         push_vec(toks, num(str_token));
//     }
// }

// AST program(AST ast, char* str_token) {
//     if (keyword(ast, str_token))
// }

// AST keyword(AST ast, char* str_token) {

// }