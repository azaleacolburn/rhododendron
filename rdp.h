#include"error.h"
#include"tokenizer.h"

typedef struct TokenNode {
    Token* token;
    Vec* children;
} TokenNode;

typedef enum Types {
    TYPE_INT,
    TYPE_CHAR
} Types;

Error program(char* strin, long file_size);

Error declare(Tokenizer* t, TokenNode* parent, Vec* id_list);

Error assign(Tokenizer* t, TokenNode* parent, Vec* id_list);

Error var_id(Tokenizer* t, TokenNode* parent, Vec* id_list);

Error expr(Tokenizer* t, TokenNode* parent, Vec* id_list);

Error op_expr(TokenNode* parent, Vec* ops, int i);

Error val_expr(TokenNode* parent, Vec* vals, int i, Vec* id_list);

Vec* format_expression(Tokenizer* t, Vec* id_list);

Error statement(Tokenizer* t, TokenNode* parent, Vec* id_list);

Error conditional(Tokenizer* t, TokenNode* parent, Vec* id_list);

Error condition(Tokenizer* t, TokenNode* parent, Vec* id_list);

Error loop(Tokenizer* t, TokenNode* parent, Vec* id_list);

int multidlen(char** arr);

void print_token(TokenNode* tok);