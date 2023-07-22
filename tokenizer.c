#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#include"tokenizer.h"

#define is_num(c) ((c) >= '0' && (c) <= '9')
#define is_letter(c) ((c) <= 'a' && (c) <= 'z' || (c) >= 'A' && (c) <= 'Z')
#define is_typename(s) (is_char(s) || is_int(s))
#define is_char(s) (strcmp(s, "char") == 0)
#define is_int(s) (strcmp(s, "int") == 0)
#define is_expr(t) (t == TOK_ADD, t == TOK_DIV, t == TOK_SUB, t == TOK_MUL, t == TOK_NUM, t == TOK_ID)
#define peek(s, i) s + i

static char* delimiters = " \0";
static char* keywords[3] = {"if", "for", "while"};

Token* get_next_token(Tokenizer* t) {
    char* str;
    printf("tokenizer string:\n%s\n", t->string);
    for (int i = 0; i < strlen(t->string); i++) {
        if (check_delimeter(t->string[i])) {
            str = str_remove(t->string, 0, i);
            printf("str_token: %s\n", str);
            return str_to_tok(str);
        }
    }
    printf("Tokenizer string ran out\n");
    return NULL;
}

// Don't worry about syntax, just tokenize
Token* str_to_tok(char* str_tok) {
    printf("%s\n", str_tok);
    Token* tok;
    TokType type = TOK_NONE;
    void* value;
    TokType kw = kwck(str_tok);
    if (is_num(*str_tok)) {
        type = TOK_NUM;
        value = &str_tok;
        goto DONE;
    } else if (is_typename(str_tok)) {
        type = TOK_DECLARE;
        goto DONE;
    }
    // } else if (idck(id_list, str_tok)) {
    //     type = TOK_ID;
    //     tok->type = str_tok;
    //     goto DONE;
    else if (kw != TOK_NONE) {
        if (kw == TOK_IF) {
            type = TOK_IF;
        } else if (kw == TOK_WHILE) {
            type = TOK_WHILE;
        } else if (kw == TOK_FOR) {
            type = TOK_FOR;
        }
        goto DONE;
    }
    // Check for special characters
    printf("str_tok: %c\n", *str_tok);
    switch (*str_tok) {
        case '(':
            type = TOK_O_PAREN;
            break;
        case ')':
            type = TOK_C_PAREN;
            break;
        case ';':
            // This needs to be fixed
            consume_line(str_tok);
            break;
        case '=':
            if (str_tok[1] == '=') {
                type = TOK_EQ_CMP;
            }
            type = TOK_EQ;
            break;
        case '|':
            printf("or\n");
            if (str_tok[1] == '=') {
                type = TOK_B_OR_EQ;
            }
            type = TOK_B_OR;
            break;
        case '&':
            if (str_tok[1] == '=') {
                type = TOK_B_AND_EQ;
            }
            type = TOK_B_AND;
            break;
        case '^':
            if (str_tok[1] == '=') {
                type = TOK_B_XOR_EQ;
            }
            type = TOK_B_XOR;
            break;
        case '!':
            if (str_tok[1] == '=') {
                type = TOK_NEQ_CMP;
            } else {
                type = TOK_NOT;
            }
            break;
        case '-':
            if (str_tok[1] == '=') {
                type = TOK_SUB_EQ;
                break;
            }
            type = TOK_SUB;
            break;
        case '+':
            if (str_tok[1] == '=') {
                type = TOK_ADD_EQ;
                break;
            }
            type = TOK_ADD;
            break;
        case '/':
            if (str_tok[1] == '=') {
                type = TOK_DIV_EQ;
                break;
            }
            type = TOK_DIV;
            break;
        case '*':
            if (str_tok[1] == '=') {
                type = TOK_MUL_EQ;
                break;
            }
            type = TOK_DIV;
            break;
    }
    if (type == TOK_NONE) {
        type = TOK_ID;
        value = str_tok;
    }
    // if (!type) {
    //     printf("FAILURE\n");
    //     *(int*) 0; // purposeful segfault uwu
    // }
    DONE:
    tok = new_token(type);
    tok->value = value;
    return tok;
}

// Checks if there are tokens left in the line
// 1 is true, 0 is false
int line_left(Tokenizer* t) {
    int len = strlen(t->string);
    if (len > 0) {
        for (int i = 0; i < len; i++) {
            if ((!(t->string[i] == '\n') || !(t->string[i] == ';')) && (i >= 1)) {
                return 1;
            }
        }
    }
    return 0;
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

TokType kwck(char* word) {
    // printf("kwck\n");
    for (int i = 0; i < 3; i++) {
        if (strcmp(word, keywords[i]) == 0) {
            return i + 1; // This is a clever idea done wrong
        }
    }
    return TOK_NONE;
}

int idck(Vec* id_list, char* word) {
    for (int i = 0; i < id_list->len; i++) {
        if (strcmp(word, (char*)get_vec(id_list, i)) == 0)
            return 1;
    }
    return 0;
}

// Leaks memory
void consume_line(char* str) {
    do {
        str++;
    } while (*str != '\n'); 
    str++;
}

// Gets next token without consuming it(maybe)
Token* peek_tok(Tokenizer* t) {
    char str[100]; // tokens over size 10 beware  
    printf("tokenizer string:\n%s\n", t->string);
    for (int i = 0; i < strlen(t->string); i++) {
        if (check_delimeter(t->string[i])) {
            strncpy(str, t->string, (size_t)i);
            printf("fixed\n");
            return str_to_tok(str);
        }
    }
    printf("Tokenizer string ran out\n");
    return NULL;
}