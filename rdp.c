#include<string.h>
#include<stdlib.h>
#include<stdio.h>
#include<stdbool.h>
#include"rdp.h"

#define args() Tokenizer *t, TokenNode* parent, Vec* id_list
#define is_change_tok(t) \
                t == TOK_B_AND_EQ || \
                t == TOK_B_OR_EQ || \
                t == TOK_B_XOR_EQ || \
                t == TOK_ADD_EQ || \
                t == TOK_MUL_EQ || \
                t == TOK_DIV_EQ || \
                t == TOK_SUB_EQ
#define is_op(c) \
        c == '+' || \
        c == '-' || \
        c == '/' || \
        c == '*'
#define filled(type) type == TOK_NUM || type == TOK_ID

Error program(char* string, long file_size) {
    printf("here with content:\n%s\n", string);
    Token* program_tok = new_token(TOK_PROGRAM);
    Tokenizer* t = new_tokenizer(string);
    Vec* id_list = new_vec(10);
    // Vec* error_list = new_vec(2); // Might use this later
    // Every starting token should have a function here
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
    return ERR_NOT;
}

Error declare(args()) {
    printf("declare\n");
    Token* tok = get_next_token(t);
    printf("after getting token\n");
    if (tok->type == TOK_DECLARE) {
        TokenNode* dec_tok = new_token_node(tok);
        push_vec(parent->children, dec_tok);
        printf("Declaration found and token pushed to tree\n");
        Error assignment_result = assign(t, dec_tok, id_list);
        return assignment_result;
    } else {
        printf("No declaration found");
        return ERR_NOT;
    }
}

Error assign(args()) {
    Token* tok = get_next_token(t);
    TokenNode* assign_node = new_token_node(tok);
    Error var_id_result = var_id(t, assign_node, id_list);
    if (var_id_result == ERR_NONE) {
        Token* change_tok = get_next_token(t);
        if (is_change_tok(change_tok)) {
            TokenNode* change_node = new_token_node(change_tok);
            push_vec(assign_node, change_node);
            Error expr_result = expr(t, assign_node, id_list);
            if (expr_result == ERR_NOT) return ERR_EXPECTED_EXPR;
            else if (expr_result == ERR_NONE)
                push_vec(parent->children, assign_node);
            return expr_result;
        } else return ERR_NOT;
    } else return var_id_result;
}

// Can be used for processing any id
Error var_id(args()) {
    printf("started variable iding\n");
    Token* id_tok = get_next_token(t);
    if (id_tok == TOK_ID) {
        if (!kwck(id_tok->value)) {
            if (!idck(id_list, id_tok->value)) {
                printf("new id: %s\n", id_tok->value);
                push_vec(id_list, id_tok->value);
            }
            TokenNode* id_node = new_token_node(id_tok);
            push_vec(parent->children, id_tok);
            return ERR_NONE;
        } else return ERR_KEYWORD_PLACEMENT;
    } else return ERR_EXEPCTED_ID;
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
// Error expr(args()) {
//     Tok parent_type = parent->type;
//     char* str_token = get_next_token(t);
//     if (*str_token == ';') {
//         return ERR_NOT;
//     }
//     if (!is_keyword(str_token)) {
//         if (is_num(*str_token)) {
//             if (parent_type == TOK_NUM || parent_type == TOK_ID)
//                 return ERR_ARITHMETIC_OPERATOR;
//             Token* num_tok = new_token(TOK_NUM);
//             push_vec(num_tok->children, str_token);
//             if (expr(t, num_tok, id_list) == ERR_NOT) {
//                 return ERR_NONE;
//             }
//         } else if (!idck(id_list, str_token)) { // Need to check for valid ids vs operators
//             if (parent_type == TOK_NUM || parent_type == TOK_ID) {
//                 Token* op_tok;
//                 if (*str_token == '+')
//                     op_tok = new_token(TOK_ADD);
//                 else if (*str_token == '-')
//                     op_tok = new_token(TOK_SUB);
//                 else if (*str_token == '*')
//                     op_tok = new_token(TOK_MUL);
//                 else if (*str_token == '/')
//                     op_tok = new_token(TOK_DIV);
//                 else
//                     return ERR_ARITHMETIC_OPERATOR;
//                 push_vec(op_tok->children, parent);
//                 expr(t, op_tok, id_list);
//             } else // If it's an operator
//                 return ERR_ARITHMETIC_OPERATOR;

//             // expr(t, tok, id_list);
//             // push_vec(parent->children, tok);
//             // return 0;
//         } else { // if we currently have an id
//             if (parent_type == TOK_NUM || parent_type == TOK_ID)
//                 return ERR_ARITHMETIC_OPERATOR;
//             Token* id_tok = new_token(TOK_ID);
//             push_vec(parent->children, id_tok);
//             return expr(t, parent, id_list);
//         }
//     } else {
//         return ERR_KEYWORD_PLACEMENT;
//     }
// }
// Parses expressions cleverly, by splitting values and operations up
// Real/test expr
Error expr(args()) {
    Vec* expr = format_expression(t, id_list);
    if (get_vec(expr, 0) == ERR_NOT)
        return ERR_NOT;
    Vec* ops = get_vec(expr, 1);
    Vec* vals = get_vec(expr, 2);
    Error op_tree_result = op_expr(parent, ops, 0);
    Error val_expr_result = val_expr(parent, vals, 0, id_list);
    if (op_tree_result == ERR_NONE)
        return val_expr_result;
    else return op_tree_result;
}

// Builds a tree of ops
Error op_expr(TokenNode* parent, Vec* ops, int i) {
    if (ops->len == i)
        return ERR_NONE;
    Token* op_tok = new_token(get_vec(ops, i));
    push_vec(parent->children, op_tok);
    if (parent->children->len - 1 == 0)
        return op_expr(parent, ops, i + 1);
    else if (parent->children->len - 1 == 1)
        return op_expr(op_tok, ops, i + 1);
    else return ERR_FORMATTED_AST_WRONG;
}

// Adds value leaves to a tree of ops
Error val_expr(TokenNode* parent, Vec* vals, int i, Vec* id_list) {
    if (vals->len == i)
        return ERR_NONE;
    TokenNode* val_tok;
    Token* curr = get_vec(vals, i);
    if (parent->children->len == 2) { // Full
        // This will only ever try left hand nodes
        TokenNode* right = get_vec(parent->children, 0);
        TokenNode* left = get_vec(parent->children, 1);
        if (filled(right->token->type)) {
            if (filled(right->token->type)) {
                return val_expr(parent, vals, i, id_list);
            } return val_expr(right, vals, i, id_list);
        } else return val_expr(get_vec(parent->children, 0), vals, i, id_list);
    }
    else if (parent->children->len == 1) {
        if (curr->type == TOK_ID && !idck(id_list, curr->value))
            return ERR_ID_NOT_VALID;
        val_tok = new_token_node(curr);
        push_vec(parent->children, val_tok);
        return val_expr(parent, vals, i + 1, id_list);
    } else if (parent->children->len == 0) {
        if (curr->type == TOK_ID && !idck(id_list, curr->value))
            return ERR_ID_NOT_VALID;
        val_tok = new_token_node(curr);
        push_vec(parent->children, val_tok);
        return val_expr(parent, vals, i + 1, id_list);
    }
    return ERR_NOT;
}

// Forgot that values are actually their own strings
Vec* format_expression(Tokenizer* t, Vec* id_list) {
    // Rewrite as ops first
    Vec* ret = new_vec(10);
    Token* tok = get_next_token(t);
    
    // Both of these are TokTypes
    Vec* ops;
    Vec* values;
    while (line_left(t)) {
        if (is_op(tok->type)) 
            push_vec(ops, tok);
        else if(idck(id_list, tok)) push_vec(values, tok); // change to else if
        else {
            push_vec(ret, ERR_NONE);
            return ret;
        }
        tok = get_next_token(t);
    }
    push_vec(ret, ERR_NONE);
    push_vec(ret, ops);
    push_vec(ret, values);
    return ret;
}

Error statement(args()) {
    TokType parent_type = parent->type;
    Token* statement_tok = new_token(TOK_STATEMENT);
    char* str_token = get_next_token(t);
    if (kwck(str_token)) {
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

int multidlen(char** arr) {
    int i = 0;
    while (arr != NULL) {
        arr++;
        i++;
    }
    return i;
}

void print_token(TokenNode* tok) {
    printf("Token Type: %d\n", tok->token->type);
    printf("Token children: ");
    for (int i = 0; i < tok->children->len; i++) {
        print_token(get_vec(tok->children, i));
    }
    printf("Token printing complete\n");
}

TokenNode* new_token_node(Token* tok) {
    TokenNode* node = malloc(sizeof(TokenNode));
    node->token = tok;    
}

void free_token_node(TokenNode* node) {
    free_token(node->token);
    free(node->children);
    free(node);
}

int idck(Vec* id_list, char* word) {
    for (int i = 0; i < id_list->len; i++) {
        if (strcmp(word, (char*)get_vec(id_list, i)) == 0)
            return 1;
    }
    return 0;
}