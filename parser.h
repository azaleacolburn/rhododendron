#include"error.h"
#include"tokenizer.h"

#define args() Tokenizer *t, TokenNode* parent, Vec* id_list

typedef struct TokenNode {
    Token* token;
    Vec* children;
} TokenNode;

typedef union ASTReturn {
    TokenNode* ast;
    Error err;
} ASTReturn;

typedef enum Types {
    TYPE_INT,
    TYPE_CHAR
} Types;

Vec* program(char* string, long file_size);

Error declare(args());

Error assign(args());

Error var_id(args());

Error handle_expr(args());

ASTReturn parse_expr(Tokenizer* t);

ASTReturn parse_factor(Tokenizer* t);

TokenNode* parse_term(Tokenizer* t);

Error format_expression(Tokenizer* t, Vec* id_list, Vec* ret_buff);

Error statement(args());

Error conditional(args());

Error while_loop(args());

Error for_loop(args());

Error program_check(args());

void print_token_node(TokenNode* tok);

void free_token_node(TokenNode* node);

TokenNode* new_token_node(Token* tok);