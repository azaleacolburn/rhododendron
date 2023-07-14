#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include"tokenizer.h"

#define is_delimiter(t) strpbrk(delimiters, t) != NULL

static char delimiters[] = "\n; ";

char* get_next_token(Tokenizer t) {
    if (has_more_tokens(t) == 0) {
        return "\0";
    }
    static char string[10] = "";
    for (int i = t.cursor; i < strlen(t.string); i++) {
        if (t.string[i] == ' ') {
            slice(t.string, string, t.cursor, i);         
            t.cursor = i;
            return string;
        }
    }
    return "";
}

// 1 if there are more tokens
// 0 if there aren't
int has_more_tokens(Tokenizer t){
    if (t.cursor < strlen(t.string)) {
        return 1;
    } else {
        return 0;
    }
}

// Params: string, buffer to be copied into, start and end indexed
void slice(const char* str, char* result, size_t start, size_t end) {
    strncpy(result, str + start, end - start);
}