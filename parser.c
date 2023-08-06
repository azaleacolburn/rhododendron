#include<string.h>
#include<stdlib.h>
#include<stdio.h>
#include<stdbool.h>
#include"parser.h"

#define is_change_tok(t) ( \
                t == TOK_B_AND_EQ || \
                t == TOK_B_OR_EQ || \
                t == TOK_B_XOR_EQ || \
                t == TOK_ADD_EQ || \
                t == TOK_MUL_EQ || \
                t == TOK_DIV_EQ || \
                t == TOK_SUB_EQ) || \
                t == TOK_EQ
#define is_keyword(t) ( \
                t == TOK_FOR || \
                t == TOK_WHILE || \
                t == TOK_IF)
#define is_op(t) ( \
                t == TOK_B_AND || \
                t == TOK_B_OR || \
                t == TOK_B_XOR || \
                t == TOK_ADD || \
                t == TOK_MUL || \
                t == TOK_DIV || \
                t == TOK_SUB)
#define filled(type) (type == TOK_NUM || type == TOK_ID)

#define id_exists(tok) (tok->type == TOK_ID && idck(id_list, tok->value))

Error program(char* string, long file_size) {
    printf("here with content:\n%s\n", string);
    TokenNode* program_node = new_token_node(new_token(TOK_PROGRAM));
    Tokenizer* t = new_tokenizer(string);
    Vec* id_list = new_vec(10);
    // Vec* error_list = new_vec(2); // Might use this later
    // Every starting token should have a function here
    while (strlen(t->string) > 0) {
        Error result = program_check(t, program_node, id_list);
        return result;
    }
    return ERR_NOT;
}

Error declare(args()) {
    printf("declare\n");
    Token* tok = get_next_token(t);
    printf("after getting token\n");
    if (tok->type == TOK_DECLARE) {
        TokenNode* dec_tok = new_token_node(tok);
        printf("test\n");
        push_vec(parent->children, dec_tok); // maybe this
        // if (dec_tok->token->type);
        print_token_node(parent);
        
        printf("Declaration found and token pushed to tree\n");
        Error assignment_result = assign(t, dec_tok, id_list);
        return assignment_result;
    } else {
        printf("No declaration found");
        return ERR_NOT;
    }
}

Error assign(args()) {
    // print_token_node(parent);
    // printf("parent children length: %zu\n", assign->children->len);
    TokenNode* assign_node = new_token_node(new_token(TOK_ASSIGN));
    Error var_id_result = var_id(t, assign_node, id_list);
    printf("assign\n");
    if (var_id_result == ERR_NONE) {
        Token* change_tok = get_next_token(t);
        printf("change token:\n");
        print_token(change_tok);
        // print_tok_type(change_tok->type);
        if (is_change_tok(change_tok->type)) {
            printf("change\n");
            TokenNode* change_node = new_token_node(change_tok);
            push_vec(assign_node->children, change_node); // change_node is the problem
            printf("parent children length: %zu\n", assign_node->children->len); 
            Error expr_result = expr(t, assign_node, id_list); // This is the expr is coming from
            if (expr_result == ERR_NOT) return ERR_EXPECTED_EXPR;
            else if (expr_result == ERR_NONE)
                push_vec(parent->children, assign_node);
            return expr_result;
        } else {
            printf("NOT AN ASSIGNMENT, NOT AN ASSIGNMENT!!!\n");
            return ERR_NOT;
        }
    } else return var_id_result;
}

// Can be used for processing any id
Error var_id(args()) {
    print_token_node(parent);
    printf("started variable iding\n");
    Token* id_tok = get_next_token(t);
    // print_token(id_tok);
    // printf("var id tok type\n");
    if (id_tok->type == TOK_ID) {
        printf("kwck\n");
        if (kwck(id_tok->value) == TOK_NONE) {
            if (!idck(id_list, id_tok->value)) {
                printf("new id: %s\n", (char*)id_tok->value);
                push_vec(id_list, id_tok->value); // We lose access to this value when we do this. Do we? OML WTF
            }
            TokenNode* id_node = new_token_node(id_tok);
            print_token_node(id_node);
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

// Parses expressions cleverly, by splitting vals_tokens and operations up
Error expr(args()) {
    printf("think\n");
    // This is the last print, by here you can't print it
    print_token_node(parent); // so here this problem is being haved
    Vec* expr = new_vec(10);
    Error expr_result = format_expression(t, id_list, expr);
    if (expr_result == ERR_NOT)
        return ERR_EXPECTED_EXPR;
    Vec* ops_tokens = get_vec(expr, 0);
    Vec* vals = get_vec(expr, 1); 
    Error op_tree_result = op_expr(parent, ops_tokens, 0);
    printf("expr\n");
    // printf("parent type: %d\n", (((TokenNode*)get_vec(parent->children, 0))->token));
    if (((TokenNode*)get_vec(parent->children, 0))->token->type) printf("this\n"); // This is toxic
    Error val_expr_result = val_expr(parent, vals, 0, id_list);
    
    if (op_tree_result == ERR_NONE)
        return val_expr_result;
    else return op_tree_result;
}

// Builds a tree of ops_tokens
Error op_expr(TokenNode* parent, Vec* ops_tokens, int i) {
    if (ops_tokens->len == i)
        return ERR_NONE;
    Token* op_tok = get_vec(ops_tokens, i);
    TokenNode* op_node = new_token_node(op_tok);
    push_vec(parent->children, op_node);
    if (parent->children->len - 1 == 0)
        return op_expr(parent, ops_tokens, i + 1);
    else if (parent->children->len - 1 == 1)
        return op_expr(op_node, ops_tokens, i + 1);
    else return ERR_FORMATTED_AST_WRONG;
}

// Adds value leaves to a tree of ops_tokens
// Right to left, two per leaf
// Should work
Error val_expr(TokenNode* parent, Vec* vals, int i, Vec* id_list) {
    printf("val expr called\n");
    if (vals->len == i) return ERR_NONE;
    TokenNode* val_node = new_token_node(get_vec(vals, i));
    // print_token_node(val_node);
    Error result = ERR_NONE;
    TokenNode* right = get_vec(parent->children, 0);
    // print_token_node(right); // for some reason printing the tok->type is seg faulting
    TokenNode* left = get_vec(parent->children, 1);
    // printf("val\n");
    // print_token(parent->token); // this faults too
    // printf("past\n");
    // print_token_node(right);
    // This is recursive because it needs to be called twice
    // if (right == NULL && left == NULL) {
    //     printf("null\n");
    // }

    // There's an issue with parent(first right)'s token type
    // if (parent->token- >type == NULL) printf("null");
    printf("here\n"); // last
    // if (right == NULL && left == NULL) printf("here\n");
    if (!filled(parent->token->type)) printf("y\n"); // This type is poisoned on the second way through
    if (right == NULL && left == NULL && !filled(parent->token->type)) { // parent is a leaf 
        printf("op node\n");
        if (val_node->token->type == TOK_ID && !idck(id_list, val_node->token->value))
            return ERR_ID_NOT_VALID;
        push_vec(parent->children, val_node);
        i++;
        result = val_expr(parent, vals, i + 1, id_list);
        if (result != ERR_NONE || result != ERR_NOT)
            return result;
    }
    if (right != NULL) {
        printf("right\n");
        result = val_expr(right, vals, i, id_list);
        printf("recursed\n");
        if (result != ERR_NONE)
            return result; 
    } else printf("RIGHT NOT NULL\n");
    if (left != NULL) {
        printf("left\n");
        result = val_expr(left, vals, i, id_list);
        if (result != ERR_NONE)
            return result; 
    }
    return ERR_NONE;
}

// Modifies ret_buff like a return value
Error format_expression(Tokenizer* t, Vec* id_list, Vec* ret_buff) {
    printf("STARTED EXPR FORMATTING\n");
    Token* tok = get_next_token(t);
    // Both of these are Tokens
    Vec* ops_tokens = new_vec(2);
    Vec* vals_tokens = new_vec(2);
    while (tok->type != TOK_SEMI && tok->type != TOK_NONE) {
        print_token(tok);
        if (is_op(tok->type)) {
            printf("formatted op\n");
            push_vec(ops_tokens, tok);
            tok = get_next_token(t);
        }
        else if (id_exists(tok) || (tok->type == TOK_NUM)) {
            printf("formatted id\n");
            push_vec(vals_tokens, tok);
            tok = get_next_token(t);
        }
        else {
            if (vals_tokens->len == ops_tokens->len + 1) {
                printf("this\n");
                goto DONE;
            }
            printf("not\n");
            return ERR_NOT;
        }
    } 
    printf("found semi-colon\n");
    DONE:
    // printf("done\n");
    push_vec(ret_buff, ops_tokens);
    // printf("here\n");
    push_vec(ret_buff, vals_tokens);
    // printf("hereish\n");
    return ERR_NONE;
}

Error statement(args()) {
    Error result;
    Token* statement_tok = new_token(TOK_STATEMENT);
    TokenNode* statement_node = new_token_node(statement_tok);
    Token* tok = get_next_token(t);
    if (tok->type == TOK_IF) {
        result = conditional(t, statement_node, id_list);
        if (result != ERR_NONE)
            return result;
    } else if (tok->type == TOK_WHILE) {
        result = while_loop(t, statement_node, id_list);
        if (result != ERR_NONE)
            return result;
    } else if (tok->type == TOK_FOR) {
        result = for_loop(t, statement_node, id_list);
        if (result != ERR_NONE)
            return result;
    }
    return ERR_NOT;
}

Error conditional(args()) {
    Token* tok = get_next_token(t);
    if (tok->type == TOK_O_PAREN) {
        Token* dummy;
        TokenNode* comparitor_node = new_token_node(dummy);
        Error expr_result = expr(t, comparitor_node, id_list);
        if (expr_result == ERR_NONE) {
            Token* comparitor = get_next_token(t);
            print_tok_type(comparitor->type);
            comparitor_node->token = comparitor;
            push_vec(parent->children, comparitor_node);
            expr_result = expr(t, comparitor_node, id_list);
            if (expr_result == ERR_NONE) {
                if (get_next_token(t)->type == TOK_C_PAREN && get_next_token(t)->type == TOK_O_CURL) {
                    // Check all possibilities
                    // Back to base witout root node
                    // Maybe some subset of program()
                }
                if (get_next_token(t)->type == TOK_C_CURL)
                    return ERR_NONE;
            } return expr_result;
        } return expr_result;
    } else return ERR_MISSING_O_PARAEN;
}


Error while_loop(args()) {
    Token* tok = new_token(TOK_WHILE);
    TokenNode* node = new_token_node(tok);
    if (get_next_token(t)->type == TOK_O_PAREN ) {
        Error condition_result = conditional(t, node, id_list);
        if (condition_result != ERR_NONE)
            return condition_result;
    } else return ERR_MISSING_O_PARAEN;
    if (get_next_token(t)->type == TOK_C_PAREN)
        return ERR_NONE;
    else return ERR_MISSING_C_PARAEN;
    // check for every possible thing here and append it to the while token
}


// This is a mess, please refactor at all costs
Error for_loop(args()) {
    Token* tok = new_token(TOK_FOR);
    TokenNode* node = new_token_node(tok);
    if (get_next_token(t)->type == TOK_O_PAREN) {
        Error declare_result = declare(t, node, id_list);
        if (declare_result == ERR_NONE) {
            if (get_next_token(t)->type == TOK_SEMI) { 
                // This might be a problem with paratheses becoming madatory around for loop conditions, refactor later to assure this doesn't happen
                Error condition_result = conditional(t, node, id_list);
                if (condition_result == ERR_NONE) {
                    if (get_next_token(t)->type == TOK_SEMI) {
                        Token* incr_token = get_next_token(t);
                        if (incr_token->type == TOK_ID  && strcmp(incr_token->value, get_vec(id_list, id_list->len))) {
                            TokenNode* incr_node = new_token_node(incr_token);
                            push_vec(node->children, incr_node);
                            return program_check(t, parent, id_list);
                        } else return ERR_EXPECTED_EXPR;
                    } else return ERR_MISSING_SEMICOLON;
                } else return condition_result;
            } else return ERR_MISSING_SEMICOLON;
        } else return declare_result;
    } else return ERR_EXPECTED_CONDITION;
    if (get_next_token(t)->type == TOK_C_PAREN)
        return ERR_NONE;
}

// Make sure you're passing in the right parent
// This will check for expr and statements
Error program_check(args()) {
    // Don't get token, peek forward
    Token* tok = peek_tok(t);
    printf("testish\n");
    if (is_keyword(tok->type)) {
        Error statement_result = statement(t, parent, id_list);
        if (statement_result == ERR_NOT)
            return ERR_EXPECTED_STATEMENT;
        else return statement_result;
    } else if (tok->type == TOK_DECLARE) {
        Error declare_result = declare(t, parent, id_list);
        if (declare_result == ERR_NOT)
            return ERR_EXPECTED_DECLARATION;
        else return declare_result;
    } else if (tok->type == TOK_ID) {
        Error assign_result = assign(t, parent, id_list);
        if (assign_result == ERR_NOT)
            return ERR_EXPECTED_ASSIGNMENT;
        else return assign_result;
    }
    else return ERR_NOT;
}

TokenNode* new_token_node(Token* tok) {
    TokenNode* node = malloc(sizeof(TokenNode));
    node->token = tok;
    node->children = new_vec(10);
    return node;
}

void free_token_node(TokenNode* node) {
    free_token(node->token);
    free(node->children);
    free(node);
}

void print_token_node(TokenNode* tok) {
    // printf("here\n");
    // printf("address %p\n", tok);
    // printf("tok address %p\n", tok->token);
    print_token(tok->token); // this imght be a null arg
    printf("Token children:\n");
    if (tok->children == NULL) return;
    for (int i = 0; i < tok->children->len; i++) {
        print_token_node(get_vec(tok->children, i));
    }
    printf("Token printing complete\n");
}