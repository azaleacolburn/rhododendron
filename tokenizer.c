#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include"tokenizer.h"

static char* delimiters = "\n; \0";

char* get_next_token(Tokenizer* t) {
    char* ret;
    printf("tokenizer string:\n%s\n", t->string);
    for (int i = 0; i < strlen(t->string); i++) {
        printf("here\n");
        if (check_delimeter(t->string[i])) {
            ret = str_remove(t->string, 0, i);
            printf("token found: %s\n", ret);
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

// Start index inclusive, end index exclusive
// Returns the removed substring
char* str_remove(char* str, int start_index, int end_index) {
    printf("here2\n");
    if (start_index < end_index) {
        char* ret;
        strncpy(ret, str + start_index, end_index - start_index);
        printf("str copied\n");
        memmove(&str[start_index], &str[end_index], strlen(str) - (end_index - start_index));
        return ret; // ret should be on the heap
    }
    printf("str_remove failed");
    return "";
}

