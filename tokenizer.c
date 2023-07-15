#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include"tokenizer.h"

static char delimiters[] = "\n; \0";

char* get_next_token(Tokenizer* t) {
    // if (!has_more_tokens(*t)) {
    //     printf("cursor too long\n");
    //     (*t).cursor = 0;
    // }
    static char string[100] = "";
    printf("tokenizer string:\n%s\n", (*t).string);
    printf("cursor: %zu\n", *(*t).cursor);
    for (int i = *(*t).cursor; i < strlen((*t).string); i++) {
        if (check_delimeter((*t).string[i])) {
            slice((*t).string, string, *(*t).cursor, i + 1);      
            *(*t).cursor = i;
            printf("token found: %s\n", string);
            return string;
        }
    }
    printf("Tokenizer string ran out\n");
    return "";
}

int has_more_tokens(Tokenizer t){
    if (*t.cursor < strlen(t.string)) {
        return 1;
    } else {
        return 0;
    }
}

Tokenizer* new_tokenizer(char* string) {
    Tokenizer* t = malloc(sizeof(Tokenizer));
    size_t c = 0;
    t->cursor = &c;
    t->string = string;
    return t;
}

void free_tokenizer(Tokenizer* t) {
    free(t->cursor);
    free(t->string);
    free(t);
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