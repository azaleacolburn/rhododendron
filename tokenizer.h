#include"tokens.h"
#include"vector.h"

typedef struct Tokenizer {
    char* string;
} Tokenizer;

Token* get_next_token(Tokenizer* t);

Token* str_to_tok(char* str_tok);

Tokenizer* new_tokenizer(char* string);

void free_tokenizer(Tokenizer* t);

// Params: string, buffer to be copied into, start and end indexed
void slice(const char* str, char* result, size_t start, size_t end);

int check_delimeter(char c);

char* str_remove(char* str, int start_index, int end_index);

TokType kwck(char* word);

int idck(Vec* id_list, char* word);

Token* peek_tok(Tokenizer* t);