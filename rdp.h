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
    TOK_FOR,
} Tok;

typedef struct Token {
    Tok type;
    Vec* children;
} Token;

Error program(char* strin, long file_size);

Error declare(Tokenizer* t, Token* parent, Vec* id_list);

Error var_id(Tokenizer* t, Token* parent, Vec* id_list);

Error expr(Tokenizer* t, Token* parent, Vec* id_list);

Error statement(Tokenizer* t, Token* parent, Vec* id_list);

Error conditional(Tokenizer* t, Token* parent, Vec* id_list);

Error condition(Tokenizer* t, Token* parent, Vec* id_list);

Error loop(Tokenizer* t, Token* parent, Vec* id_list);

void print_token(Token* tok);

Token* new_token(Tok type);

int kwck(char* word);

int idck(Vec* id_list, char* word);