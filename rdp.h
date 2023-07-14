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
    TOK_STATEMENT,
    TOK_CONDITION,
    TOK_EQ,
    TOK_NEQ,
    TOK_WHILE,
    TOK_FOR
} Tok;

typedef struct Token {
    Tok type;
    Vec* children;
} Token;

Error program(char* string);

Error declare(Tokenizer* t, Token* parent);

Error var_id(Tokenizer* t, Token* parent);

Error expr(Tokenizer* t, Token* parent);

Error statement(Tokenizer* t, Token* parent);

Error conditional(Tokenizer* t, Token* parent);

Error condition(Tokenizer* t, Token* parent);

Error loop(Tokenizer* t, Token* parent);

void print_token(Token* tok);

Token* new_token(Tok type);