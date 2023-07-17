#include<string.h>
#include<stdlib.h>
#include<stdio.h>
#include"rdp.h"

#define is_num(c) ((c) >= '0' && (c) <= '9')
#define is_letter(c) ((c) <= 'a' && (c) <= 'z' || (c) >= 'A' && (c) <= 'Z')
#define is_typename(s) (is_char(s) || is_int(s))
#define is_char(s) (strcmp(s, "char"))
#define is_int(s) (strcmp(s, "int"))
#define is_keyword(s) (kwck(s) == 1)
#define is_expr(t) (t == TOK_ADD, t == TOK_DIV, t == TOK_SUB, t == TOK_MUL, t == TOK_NUM, t == TOK_ID)
#define args() Tokenizer *t, Token* parent, Vec* id_list

static char* keywords[7] = {"if", "for", "while", "int", "char", "(", ")"};

Error program(char* string, long file_size) {
    printf("here with content:\n%s\n", string);
    Token* program_tok = new_token(TOK_PROGRAM);
    Tokenizer* t = new_tokenizer(string);
    // Vec* error_list = new_vec(2); // Might use this later
    // Every starting token should have a function here
    Vec* id_list = new_vec(2);
    while (strlen(t->string) > 0) { // Infinite loop
        Error declare_result = declare(t, program_tok, id_list);
        if (declare_result == ERR_NOT) {
            reset_tokenizer(t);
            Error expression_result = expr(t, program_tok, id_list);
            if (expression_result == ERR_NOT) {
                reset_tokenizer(t);
                Error statement_result = statement(t, program_tok, id_list);
                if (statement_result == ERR_NOT) {
                    printf("Expected Declaration, Statement or Expression");
                } return statement_result;
            } return expression_result;
        } else return declare_result;
    }
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
        Error assignment_result = assign(t, dec_tok, id_list);
        return assignment_result;
    } else {
        printf("No declaration found");
        return ERR_NOT;
    }
}

Error assign(args()) {
    Token* assign_tok = new_token(TOK_ASSIGN);
    Error var_id_result = var_id(t, assign_tok, id_list);
    if (var_id_result == ERR_NONE) {
        char* change_tok = get_next_token(t);
        if (*change_tok == '=') {
            Error expr_result = expr(t, assign_tok, id_list);
            if (expr_result == ERR_NOT) return ERR_EXPECTED_EXPR;
            else if (expr_result == ERR_NONE)
                push_vec(parent->children, assign_tok);
            return expr_result;
        } else return ERR_EXPECTED_ASSIGNMENT;
    } else return var_id_result;
}

// Parent is decl_token
// For processing new ids only
Error var_id(args()) {
    printf("started variable iding\n");
    Types type = get_vec(parent->children, 0);
    printf("this was safe\n");
    if (type == TYPE_CHAR || type == TYPE_INT) {
        char* var_id_str = get_next_token(t);
        printf("var_id tok: %s\n", var_id_str);
        if (!is_keyword(var_id_str)) {
            Token* id_tok = new_token(TOK_ID);
            push_vec(id_list, var_id_str);
            push_vec(id_tok->children, var_id_str);
            return ERR_NONE;
        } else return ERR_KEYWORD_PLACEMENT;
    } else return ERR_TYPE;
}

// TOK_NUMs are leaves
// Operators are branches
// 10*(2+5)
//  TOK_MUL 
// /        \
// TOK_NUM   TOK_ADD
// /        /        \
// 10      TOK_NUM    TOK_NUM
//         /          /
//        2           5

// Idea: reshape expression into operators before hand
Error expr(args()) {
    Tok parent_type = parent->type;
    char* str_token = get_next_token(t);
    if (*str_token == ';') {
        return ERR_NOT;
    }
    if (!is_keyword(str_token)) {
        if (is_num(*str_token)) {
            if (parent_type == TOK_NUM || parent_type == TOK_ID)
                return ERR_ARITHMETIC_OPERATOR;
            Token* num_tok = new_token(TOK_NUM);
            push_vec(num_tok->children, str_token);
            if (expr(t, num_tok, id_list) == ERR_NOT) {
                return ERR_NONE;
            }
        } else if (!idck(id_list, str_token)) { // Need to check for valid ids vs operators
            if (parent_type == TOK_NUM || parent_type == TOK_ID) {
                Token* op_tok;
                if (*str_token == '+')
                    op_tok = new_token(TOK_ADD);
                else if (*str_token == '-')
                    op_tok = new_token(TOK_SUB);
                else if (*str_token == '*')
                    op_tok = new_token(TOK_MUL);
                else if (*str_token == '/')
                    op_tok = new_token(TOK_DIV);
                else
                    return ERR_ARITHMETIC_OPERATOR;
                push_vec(op_tok->children, parent);
                expr(t, op_tok, id_list);
            } else // If it's an operator
                return ERR_ARITHMETIC_OPERATOR;

            // expr(t, tok, id_list);
            // push_vec(parent->children, tok);
            // return 0;
        } else { // if we currently have an id
            if (parent_type == TOK_NUM || parent_type == TOK_ID)
                return ERR_ARITHMETIC_OPERATOR;
            Token* id_tok = new_token(TOK_ID);
            push_vec(parent->children, id_tok);
            return expr(t, parent, id_list);
        }
    } else {
        return ERR_KEYWORD_PLACEMENT;
    }
}
// Figure out Error handling with Token* returns
// Real/test expr
Error expr2(args()) {
    Token* temp_parent = parent;
    Vec* expr = format_expression(t);
    char* ops = get_vec(expr, 0);
    char* vals = get_vec(expr, 1);
    Token* op_tree = op_expr(parent, ops, 0);
}

// Recursive
// Builds a tree of ops
Token* op_expr(Token* parent, char* ops, int i) {
    Token* op_tok;
    if (strlen(ops) == i) return parent;
    if (ops[i] == '+') {
        op_tok = new_token(TOK_ADD);
    } else if (ops[i] == '-') {
        op_tok = new_token(TOK_SUB);
    } else if (ops[i] == '/') {
        op_tok = new_tok(TOK_DIV);
    } else if (ops[i] == '*' ) {
        op_tok = new_tok(TOK_MUL);
    } else return ERR_ARITHMETIC_OPERATOR;
    push_vec(parent->children, op_tok);
    if (parent->children->len - 1 == 0)
        return op_expr(parent, ops, i + 1);
    else if (parent->children->len - 1 == 1)
        return op_expr(op_tok, ops, i + 1);
    else return ERR_FORMATTED_AST_WRONG;
}

// Recursive
Token* val_expr(Token* parent, char* vals, int i, Vec* id_list) {
    Token* val_tok;
    if (is_keyword(vals[i]))
        return ERR_KEYWORD_PLACEMENT;
    if (parent->children->len == 2) {
        return val_expr(get_vec(parent->children, 0), vals, i, id_list);
    } else if (parent->children->len == 1) {
        if (idck(id_list, vals[i]))
            val_tok = new_token(TOK_ID);
        else
            val_tok = new_token(TOK_NUM);
        push_vec(val_tok->children, vals[i]);
        push_vec(parent->children, val_tok);
        val_expr(parent, vals, i + 1, id_list);
    } else if (parent->children->len == 0) {
        if (idck(id_list, vals[i]))
            val_tok = new_token(TOK_ID);
        else
            val_tok = new_token(TOK_NUM);
        push_vec(val_tok->children, vals[i]);
        push_vec(parent->children, val_tok);
        val_expr(parent, vals, i + 1, id_list);
    }
}

Vec* format_expression (Tokenizer* t) {
    // Rewrite as ops first
    char* str_tok = get_next_token(t);
    char* ops;
    char* values;
    while (str_tok[strlen(str_tok) - 1] != '\n' || 
            str_tok[strlen(str_tok) - 1] != ';') 
    {
        if (*str_tok == '+' ||
            *str_tok == '-' ||
            *str_tok == '/' ||
            *str_tok == '*') 
            ops[strlen(ops)] = *str_tok; 
        else 
            values[strlen(values)] = str_tok;
    }
    Vec* ret = new_vec(2);
    push_vec(ret, ops);
    push_vec(ret, values);
}

Error statement(args()) {
    Tok parent_type = parent->type;
    Token* statement_tok = new_token(TOK_STATEMENT);
    char* str_token = get_next_token(t);
    if (is_keyword(str_token)) {
        printf("keyword detected");
        Error conditional_result = conditional(t, parent, id_list);
        Error loop_result = loop(t, parent, id_list);
        if (conditional_result == ERR_NOT) {
            if (loop_result == ERR_NOT)
                printf("Expected loop or conditional");
            else if (loop_result != ERR_NONE)
                return loop_result;
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
    for (int i = 0; i < 7; i++) {
        printf("word: %s, keyword: %s\n", word, keywords[i]);
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