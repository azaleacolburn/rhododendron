#include<stdio.h>
#include<stdlib.h>
#include <string.h>

typedef struct Token Token;
typedef enum Tok Tok;

char* readFile(char* path);

Token* parse(char* input);

void slice(const char* str, char* result, size_t start, size_t end);
