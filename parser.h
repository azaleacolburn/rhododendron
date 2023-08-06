#include"error.h"
#include"tokenizer.h"

#define args() Tokenizer *t, TokenNode* parent, Vec* id_list

typedef struct TokenNode {
    Token* token;
    Vec* children;
} TokenNode;

typedef enum Types {
    TYPE_INT,
    TYPE_CHAR
} Types;

Error program(char* strin, long file_size);

Error declare(args());

Error assign(args());

Error var_id(args());

Error expr(args());

Error op_expr(TokenNode* parent, Vec* ops, int i);

Error val_expr(TokenNode* parent, Vec* vals, int i, Vec* id_list);

Error format_expression(Tokenizer* t, Vec* id_list, Vec* ret_buff);

Error statement(args());

Error conditional(args());

Error while_loop(args());

Error for_loop(args());

Error program_check(args());

int multidlen(char** arr);

void print_token_node(TokenNode* tok);

TokenNode* new_token_node(Token* tok);