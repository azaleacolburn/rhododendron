#include"error.h"
#include"tokenizer.h"

#define args() Tokenizer *t, TokenNode* parent, Vec* id_list

typedef struct TokenNode {
    Token* token;
    Vec* children;
} TokenNode;

typedef enum ASTReturnTag {TAG_ERR, TAG_AST} ASTReturnTag;

typedef union ASTReturnValue {TokenNode* ast; Error err;} ASTReturnValue;

typedef struct ASTReturn {
    ASTReturnTag tag;
    ASTReturnValue value;
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

ASTReturn* parse_expr(Tokenizer* t);

ASTReturn* parse_factor(Tokenizer* t);

ASTReturn* parse_term(Tokenizer* t);

Error format_expression(Tokenizer* t, Vec* id_list, Vec* ret_buff);

Error statement(args());

Error conditional(args());

Error while_loop(args());

Error for_loop(args());

Error program_check(args());

void print_token_node(TokenNode* tok);

void free_token_node(TokenNode* node);

TokenNode* new_token_node(Token* tok);

ASTReturn* new_err_return (Error err);

ASTReturn* new_ast_return(TokenNode* node);