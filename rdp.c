#include<string.h>
#include<stdlib.h>
#include<stdio.h>
#include"rdp.h"

#define is_num(c) ((c) >= '0' && (c) <= '9')
#define is_letter(c) ((c) <= 'a' && (c) <= 'z' || (c) >= 'A' && (c) <= 'Z')
#define is_typename(s) (is_char(s) || is_int(s))
#define is_char(s) (strcmp(s, "char"))
#define is_int(s) (strcmp(s, "int"))
#define is_keyword(s) (kwck(s))
#define is_expr(t) (t == TOK_ADD, t == TOK_DIV, t == TOK_SUB, t == TOK_MUL, t == TOK_NUM, t == TOK_ID)
#define args() Tokenizer *t, Token* parent, Vec* id_list

static char* keywords[6] = {"if", "for", "while", "int", "char", "()"}; // Tshis is a bug that will disallow things like wh and ch as names

Error program(char* string, long file_size) {
    printf("here with content:\n%s\n", string);
    Token* program_tok = new_token(TOK_PROGRAM);
    Tokenizer* t = new_tokenizer(string);
    // Vec* error_list = new_vec(2); // Might use this later
    // Every starting token should have a function here
    Vec* id_list = new_vec(2);
    while (*t->cursor < file_size) { // Infinite loop
        Error declare_result = declare(t, program_tok, id_list);
        if (declare_result == ERR_NOT) {
            Error expression_result = expr(t, program_tok, id_list);
            if (expression_result == ERR_NOT) {
                Error statement_result = statement(t, program_tok, id_list);
                if (statement_result == ERR_NOT) {
                    printf("Expected Declaration, Statement or Expression");
                } else if (statement_result != ERR_NONE) {
                    return statement_result;
                }
            } else if (expression_result != ERR_NONE) {
                return expression_result;
            }
        } else if (declare_result != ERR_NONE) {
            return declare_result;
        }
    }
    free_tokenizer(t);
    return ERR_NONE;
}

Error declare(args()) {
    printf("declare\n");
    char* str_token = get_next_token(t);
    printf("after getting token\n");
    if (is_typename(str_token)) {
        Token* dec_tok = new_token(TOK_DECLARE);
        if (is_char(str_token)) 
            push_vec(dec_tok->children, "char");
        else if (is_int(str_token))
            push_vec(dec_tok->children, "int");
        printf("Declaration found and token pushed to tree\n");
        var_id(t, dec_tok, id_list);
        char* comparitor_tok = get_next_token(t);
        printf("parent: %d\n", parent->type);
        printf("token that might be =: %s\n", comparitor_tok);
        if (*comparitor_tok == '=') { // Needs to be updating for operations like +=
            
            return expr(t, dec_tok, id_list);
        } else {
            return ERR_MISSING_DECLARATION;
        }
    } else {
        printf("No declaration found");
        return ERR_NOT;
    }
}

Error assign(args()) {
    printf("assign\n");
    char* str_token = get_next_token(t);
    Token* assign_tok = new_token(TOK_ASSIGN);
    if (strcmp(str_token, "\0") || strcmp(str_token, ";")) return ERR_NOT;
    if (!is_keyword(str_token)) {
        if (idck(id_list, str_token)) {
            Token* id_tok = new_token(TOK_ID);
            push_vec(id_tok->children, str_token);
            push_vec(assign_tok->children, id_tok);
            char* change_tok = get_next_token(t);
            if (*change_tok == '=') {
                Error expr_result = expr(t, assign_tok, id_list);
                if (expr_result == ERR_NOT) return ERR_EXPECTED_EXPR;
                else if (expr_result == ERR_NONE) {
                    push_vec(parent->children, assign_tok);
                    return ERR_NONE;
                }
                else return expr_result;
            } else return ERR_EXPECTED_ASSIGNMENT;
        } else return ERR_ID_NOT_VALID;
    } else return ERR_NOT;
}

// Parent is decl_token
Error var_id(args()) {
    printf("started variable iding\n");
    int type = *(int*)get_vec(parent->children, 0); // Found segfault
    printf("this wasn't unsafe\n");
    if (type == TYPE_CHAR || type == TYPE_INT) {
        char* str_token = get_next_token(t);
        if (!is_keyword(str_token)) {
            Token* id_tok = new_token(TOK_ID);
            push_vec(id_list, str_token);
            push_vec(id_tok->children, str_token);
            return ERR_NONE;
        } else {
            return ERR_KEYWORD_PLACEMENT;
        }
    } else {
        return ERR_TYPE;
    }
}

// Todo: Allow variable ids
// Ideas: keep a vector of legal ids
Error expr(args()) {
    Tok parent_type = parent->type;
    // Token* expr_tok = malloc(sizeof(Token));
    // expr_tok->type = TOK_EXPR;
    // expr_tok->children = new_vec(0);
    char* str_token = get_next_token(t);
    if (*str_token == ';') {
        return ERR_NONE;
    }
    if (!is_keyword(str_token)) {
        if (is_num(*str_token)) {
            Token* num_tok = new_token(TOK_NUM);
            push_vec(num_tok->children, str_token);
            push_vec(parent->children, num_tok);
            return 1;
        } else {
            Token* tok;
            if (*str_token == '+')
                tok = new_token(TOK_ADD);
            else if (*str_token == '-')
                tok = new_token(TOK_SUB);
            else if (*str_token == '*')
                tok = new_token(TOK_MUL);
            else if (*str_token == '/')
                tok = new_token(TOK_DIV);
            else
                return 1;
            expr(t, tok, id_list);
            push_vec(parent->children, tok);
            return 0;
        }
    } else {
        return 1;
    }
}

Error statement(args()) {
    Tok parent_type = parent->type;
    Token* statement_tok = new_token(TOK_STATEMENT);
    char* str_token = get_next_token(t);
    if (*str_token == ';') {
        return ERR_NONE;
    }
    if (is_keyword(str_token)) {
        printf("keyword detected");
        Error conditional_result = conditional(t, parent, id_list);
        Error loop_result = loop(t, parent, id_list);
        if (conditional_result == ERR_NOT) {
            if (loop_result == ERR_NOT) {
                printf("Expected loop or conditional");
            } else if (loop_result != ERR_NONE) {
                return loop_result;
            }
        } else if (loop_result != ERR_NONE) {
            return conditional_result;
        }
        return ERR_NONE;
    }
    return ERR_NOT;
}

Error conditional(args()) {
    char* str_token = get_next_token(t);
    if (strcmp(str_token, "if")) {
        if (*get_next_token(t) == '(') {
            Token* conditional_tok = new_token(TOK_CONDITION);
            Error condition_result = condition(t, conditional_tok, id_list);
            if (condition_result == ERR_NONE) {
                Error statement_result = statement(t, conditional_tok, id_list);
                Error declare_result = declare(t, conditional_tok, id_list);
                if (statement_result == ERR_NONE || statement_result == ERR_NOT) {
                    if (declare_result == ERR_NONE || declare_result == ERR_NOT) {
                        push_vec(parent->children, conditional_tok);
                        return ERR_NONE;
                    }
                    return declare_result;
                } else {
                    return statement_result;
                }
            } else {
                return condition_result;
            } 
        } else {
            return ERR_EXPECTED_CONDITION;
        }
        if (*get_next_token(t) == ')') {
            return ERR_NONE;
        }
    } else {
        return ERR_NOT;
    }
}

Error condition(args()) {
    Error expr_result = expr(t, parent, id_list);
    if (expr_result == ERR_NONE) {
        char* comparitor = get_next_token(t);
        printf("comparitor: %s", comparitor);
        Token* tok;
        if (strcmp(comparitor, "==")) {
            tok = new_token(TOK_EQ);
        } else if (strcmp(comparitor, "!=")) {
            tok = new_token(TOK_NEQ);
        } else {
            return ERR_EXPECTED_COMPARITOR;
        }
        push_vec(parent->children, tok);
        return expr(t, parent, id_list);
    }
    return expr_result;
}

Error loop(args()) {
    char* str_token = get_next_token(t);
    if (strcmp(str_token, "while")) {
        Token* tok = new_token(TOK_WHILE);
        if (*get_next_token(t) == '(') {
            condition(t, tok, id_list);
        } else {
            return ERR_EXPECTED_CONDITION;
        }
        if (*get_next_token(t) == ')') {
            return ERR_NONE;
        }
    } else if (strcmp(str_token, "for")) {
        Token* tok = new_token(TOK_FOR);
        if (*get_next_token(t) == '(') {
            Error declare_result = declare(t, tok, id_list);
            if (declare_result != ERR_NONE) {
                printf("Expected delcaration\n");
                return declare_result;
            } else if (*get_next_token(t) == ';') {
                Error condition_result = condition(t, tok, id_list);
                if (condition_result != ERR_NONE) {
                    printf("Expected condition\n");
                    return condition_result;
                } else if (*get_next_token(t) == ';') {
                    // Figure out how to represent incrementing
                    // Expression should be fixed for this to work
                    return 0;
                }
            }
        } else {
            return ERR_EXPECTED_CONDITION;
        }
    }
    return ERR_NOT;
}

void print_token(Token* tok) {
    printf("Token Type: %d\n", tok->type);
    printf("Token children: ");
    for (int i = 0; i < tok->children->len; i++) {
        print_token(get_vec(tok->children, i));
    }
    printf("Token printing complete\n");
}

Token* new_token(Tok type) {
    Token* tok = malloc(sizeof(Token));
    tok->type = type;
    tok->children = new_vec(2);
    return tok;
}

// Figure out how to check the contents of the vector. Maybe length. goto may lead to something
void free_token(Token* root) {
    for (int i = 0; i < root->children->capacity; i++) {
        void* child = get_vec(root->children, i);
        // if ()
        free_token(get_vec(root->children,i));
    }
    free(root->children);
    free(root);
}

void print_tok(Tok type) {
    switch (type) {
        case TOK_PROGRAM:
            printf("Program\n");
            break;
        case TOK_DECLARE:
            printf("Delcaration\n");
            break;
        case TOK_NUM:
            printf("Num\n");
            break;
        case TOK_ADD:
            printf("+\n");
            break;
        case TOK_MUL:
            printf("*\n");
            break;
        case TOK_SUB:
            printf("-\n");
            break;
        case TOK_DIV:
            printf("/\n");
            break;
        case TOK_ASSIGN:
            printf("Assignment\n");
            break;
        case TOK_ID:
            printf("Id\n");
            break;
        case TOK_STATEMENT:
            printf("Statement\n");
            break;
        case TOK_CONDITION:
            printf("Condition\n");
            break;
        case TOK_EQ:
            printf("==\n");
            break;
        case TOK_NEQ:
            printf("!=\n");
        case TOK_WHILE:
            printf("While\n");
            break;
        case TOK_FOR:
            printf("For\n");
            break;
    }
}

int kwck(char* word) {
    for (int i = 0; i < 6; i++) {
        if (strcmp(word, keywords[i])) return 1;
    }
    return 0;
}

int idck(Vec* id_list, char* word) {
    for (int i = 0; i < id_list->len; i++) {
        if (strcmp(word, (char*)get_vec(id_list, i)))
            return 1;
    }
    return 0;
}