#include"vector.h"
#include"tokenizer.h"
#include"error.h"

typedef enum Types {
    TYPE_INT,
    TYPE_CHAR
} Types;

typedef enum Tok {
    TOK_PROGRAM,
    TOK_DECLARE,
    TOK_NUM,
    TOK_ADD,
    TOK_MUL,
    TOK_SUB,
    TOK_DIV,
    TOK_ASSIGN,
    TOK_ID,
} Tok;

typedef struct Token {
    Tok type;
    Vec* children;
} Token;

Error program(char* string);

Error declare(Tokenizer* t, Token* program_tok);

Error var_id(Tokenizer* t, Token* dec_tok);

Error expr(Tokenizer* t, Token* parent);

void print_token(Token* tok);

Token* new_token(Tok type);