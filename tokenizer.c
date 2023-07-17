#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include"tokenizer.h"

static char* delimiters = " \0";

char* get_next_token(Tokenizer* t) {
    char* ret;  
    printf("tokenizer string:\n%s\n", t->string);
    for (int i = 0; i < strlen(t->string); i++) {
        if (check_delimeter(t->string[i])) {
            ret = str_remove(t->string, 0, i);
            return ret; // is returning this a segfault
        }
    }
    printf("Tokenizer string ran out\n");
    return "";
}

Tokenizer* new_tokenizer(char* string) {
    Tokenizer* t = malloc(sizeof(Tokenizer));
    t->string = malloc(sizeof(char) * strlen(string));
    strcpy(t->string, string);
    return t;
}

void free_tokenizer(Tokenizer* t) {
    free(t->string);
    free(t);
    t = NULL;
}

void reset_tokenizer(Tokenizer* t) {
    strncpy(t->string, t->original, strlen(t->original));
}

// Params: string, buffer to be copied into, start and end indexed
void slice(const char* str, char* result, size_t start, size_t end) {
    strncpy(result, str + start, end - start);
}

int check_delimeter(char c) {
    for (int i = 0; i < strlen(delimiters); i++) {
        if (delimiters[i] == c) return 1;
    }
    return 0;
}

char* str_remove(char* str, int start_index, int end_index) {
    if (start_index < end_index) {
        char* ret = malloc(sizeof(char) * (end_index - start_index));
        strncpy(ret, str + start_index , end_index - start_index); // problem
        memmove(&str[start_index - 1], &str[end_index], strlen(str) - start_index - 1);
        return ret;
    } else {
        printf("str_remove failed\n");
        return "";
    }
}