#include<stdio.h>
#include<stdlib.h>
#include <string.h>
#include"parser.h"

typedef struct Token
{
    int* next;
    Tok* value;
} Token;

typedef enum Tok
{
    TOK_START,
    TOK_NUM,
    TOK_END
} Tok;

char* readFile(char* path)
{
    FILE* fp;
    char buff[255];
    fp = fopen(path, "r");
    if (fp == NULL) {
        printf("Error, path: %s doesn't exist\n", path);
        exit(1);
    }
    fscanf(fp, "%s", buff);
    fgets(buff, 255, fp);
    printf("File contents: %s\n", buff);
    return buff;
}

Token* parse(char* input)
{
    // Splits by word
    size_t* last_space = NULL;
    char** split;
    size_t len = strlen(input);
    printf("len: %zu\n", len);
    if (len == 0) 
    {
        printf("Empty Contents\n");
        exit(1);
    }
    for (size_t i, j = 0; i <= len; i++)
    {
        if (input[i] == ' ')
        {
            printf("slicing\n");
            slice(input, split[j], *last_space, i);
            ++j;
            *last_space = i;
        } 
        if (input[i] == '\0') 
        {
            printf("One token\n");
            *split = input; // deref null pointer
            printf("passed\n");
        }
    }

    if (last_space == NULL)
    {
        printf("No spaces in file\n");
    }
    
    for (size_t i = 0; *split[i] != '\0'; ++i) 
    {
        printf("this\n");
        if (strcmp(split[i], "int"))
        {
            printf("int declared\n");
        }
    }
    return NULL;
}
// Params: string, buffer to be copied into, start and end indexed
void slice(const char* str, char* result, size_t start, size_t end)
{
    strncpy(result, str + start, end - start);
}