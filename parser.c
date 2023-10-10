#include<string.h>
#include<stdlib.h>
#include<stdio.h>
#include<assert.h>
#include"parser.h"

// Everything but arithmetic experssion parsing needs to be refactored to return an ASTReturn* instead of passing around a tree tht gets modified
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
                
#define is_add(t) t->type == TOK_ADD || t->type == TOK_SUB

#define filled(type) (type == TOK_NUM || type == TOK_ID)

#define id_exists(tok) (tok->type == TOK_ID && idck(id_list, tok->value))

#define arithmetic_simplify() \
    left = get_vec(parent->children, 0); \
    right = get_vec(parent->children, 1); \
    if (right != NULL && left != NULL) { \
        if (right->token->type == TOK_NUM && left->token->type == TOK_NUM) { \
            TokType op = parent->token->type; \
            printf("OP: "); \
            print_tok_type(op); \
            int val = 0; \
            switch (op) { \
                case TOK_MUL: \
                    val = *(int*)right->token->value* *(int*)left->token->value; \
                    break; \
                case TOK_ADD: \
                    val = *(int*)right->token->value+ *(int*)left->token->value; \
                    break; \
                case TOK_SUB: \
                    val = *(int*)right->token->value- *(int*)left->token->value; \
                    break; \
                case TOK_DIV: \
                    val = *(int*)right->token->value/ *(int*)left->token->value; \
                    break; \
                case TOK_B_AND: \
                    val = *(int*)right->token->value& *(int*)left->token->value; \
                    break; \
                case TOK_B_OR: \
                    val = *(int*)right->token->value| *(int*)left->token->value; \
                    break; \
                case TOK_B_XOR: \
                    val = *(int*)right->token->value^ *(int*)left->token->value; \
                    break; \
                case TOK_B_AND_EQ: \
                    val = *(int*)right->token->value&= *(int*)left->token->value; \
                    break; \
                case TOK_B_OR_EQ: \
                    val = *(int*)right->token->value|= *(int*)left->token->value; \
                    break; \
                case TOK_B_XOR_EQ: \
                    val = *(int*)right->token->value^= *(int*)left->token->value; \
                    break; \
                default: \
                    break; \
            } \
            printf("val: %d\n", val); \
            printf("made it to simplifying arithmetic expressions\n"); \
            Token* tok = new_token(TOK_NUM); \
            tok->value = &val; \
            TokenNode* new_node = new_token_node(tok); \
            print_token(parent->token); \
            parent = new_node; \
            return result; \
        } \
    }

Vec* program(char* string, long file_size) {
    printf("here with content:\n%s\n", string);
    TokenNode* program_node = new_token_node(new_token(TOK_PROGRAM));
    Tokenizer* t = new_tokenizer(string);
    Vec* id_list = new_vec(10);
    // Vec* error_list = new_vec(2); // Might use this later
    // Every starting token should have a function here
    // while (strlen(t->string) > 0) { 
    Error result = program_check(t, program_node, id_list);
    printf("\n\nAST:\n");
    print_token_node(program_node);
    free_tokenizer(t);
    free_vec(id_list);
    Vec* ret = new_vec(2);
    Error* e = malloc(sizeof(Error));
    *e = result;
    set_vec(ret, program_node, 1);
    set_vec(ret, e, 0);
    return ret;
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
    printf("parent children length: %zu\n1 printing\n", assign_node->children->len);
    print_token_node(assign_node);
    Error var_id_result = var_id(t, assign_node, id_list); // the problem node is being appended to assign_node in here
    printf("assign\n");
    if (var_id_result == ERR_NONE) {
        Token* change_tok = get_next_token(t);
        // printf("change token:\n");
        // print_token(change_tok);
        // print_tok_type(change_tok->type);
        if (is_change_tok(change_tok->type)) {
            printf("change\n");
            // TokenNode* change_node = new_token_node(change_tok);
            // push_vec(assign_node->children, change_node); // change_node is the problem
            // printf("parent children length: %zu\n2 printing\n", assign_node->children->len);
            printf("ASSIGNMENT NODE\n\n");
            print_token(assign_node->token);
            printf("\n");
            Error expr_result = handle_expr(t, assign_node, id_list); // This is the expr is coming from
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
    TokenNode* id_node = new_token_node(get_next_token(t));
    // print_token(id_node->token);
    // printf("var id tok type\n");
    if (id_node->token->type == TOK_ID) {
        printf("kwck\n");
        if (kwck(id_node->token->value) == TOK_NONE) {
            if (!idck(id_list, id_node->token->value)) {
                printf("new id: %s\n", (char*)id_node->token->value);
                push_vec(id_list, id_node->token->value); // We lose access to this value when we do this. Do we? OML WTF
            }
            push_vec(parent->children, id_node);
            return ERR_NONE;
        } else return ERR_KEYWORD_PLACEMENT;
    } else return ERR_EXEPCTED_ID;
}

// Beautiful, recursive expression analysis
// Pass in the node before the comparitor node actually
Error handle_expr(args()) {
    ASTReturn* result = parse_expr(t);
    if (result->tag == TAG_ERR) return result->value.err;
    
    push_vec(parent->children, result->value.ast);
    return ERR_NONE;
}

ASTReturn* parse_expr(Tokenizer* t) {
    printf("PARSE EXPR CALLED\n");
    ASTReturn* left_ret = parse_term(t); // this is 1 good
    if (left_ret->tag == TAG_ERR) return left_ret;
    TokenNode* left = left_ret->value.ast;
    printf("LEFT TERM\n\n");
    print_token(left->token);
    printf("\n");
    Token* curr = get_next_token(t);
    printf("CURR\n\n");
    print_token(curr); // this 1 value is being consumed instead of the plus
    printf("\n");
    while (is_add(curr)) {
        Token* op = new_token(TOK_NONE);
        *op = *curr;
        printf("IS ARITHMETIC AND PRINTING OP: \n\n");
        print_token(op);
        printf("\n");
        curr = get_next_token(t);

        ASTReturn* right_ret = parse_term(t);
        if (right_ret->tag == TAG_ERR) return right_ret;
        TokenNode* right = right_ret->value.ast;
        TokenNode* op_tok = new_token_node(op);
        
        push_vec(op_tok->children, left);
        push_vec(op_tok->children, right);

        left = op_tok;
    }
    return new_ast_return(left);
}

ASTReturn* parse_term(Tokenizer* t) {
    printf("PARSE TERM CALLED\n");
    // printf("t: %s\n", t->string); // string is faulting here
    ASTReturn* factor_res = parse_factor(t);
    if (factor_res->tag == TAG_ERR) return factor_res;
    TokenNode* left = factor_res->value.ast;
    Token* curr = get_curr_token(t);
    printf("CURR IN TERM: \n\n");
    print_token(curr);
    printf("\n");
    while (curr->type == TOK_MUL || curr->type == TOK_DIV) {
        Token* op;
        *op = *curr;

        curr = get_next_token(t);
        printf("CURR IN TERM\n\n");
        print_token(curr);
        printf("\n");
        
        ASTReturn* right_result = parse_factor(t);
        if (right_result->tag == TAG_ERR) return right_result;
        TokenNode* right = right_result->value.ast;

        TokenNode* op_tok = new_token_node(op);
        push_vec(op_tok->children, left);
        push_vec(op_tok->children, right);

        left = op_tok;
    }
    return new_ast_return(left);
}

ASTReturn* parse_factor(Tokenizer* t) { // all of these should return ASTReturn
    printf("PARSE FACTOR CALLED\n");
    printf("FACTOR TOKEN\n\n");
    // printf("t.string: %s\n", t->string);
    Token* factor_token = get_next_token(t);
    printf("are we faulting here\n");
    print_token(factor_token);
    printf("\n");
    if (factor_token->type == TOK_NUM) {
        // Create a number node
        TokenNode* num_node = new_token_node(factor_token);
        set_vec(num_node->children, NULL, (size_t)0);
        set_vec(num_node->children, NULL, (size_t)1);
        return new_ast_return(num_node);
    } else if (factor_token->type == TOK_VAR) {
        return new_ast_return(new_token_node(factor_token));
    } else if (factor_token->type == TOK_FUNC_CALL) {
        // omg imagine if functions were a thing
    } else if (factor_token->type == TOK_O_PAREN) {
        factor_token = get_next_token(t);
        ASTReturn* expr_node_ret = parse_expr(t);
        if (expr_node_ret->tag == TAG_ERR)
            return expr_node_ret;
        TokenNode* expr_node = expr_node_ret->value.ast;
        if (get_next_token(t)->type != TOK_C_PAREN) {
            return new_err_return(ERR_MISSING_C_PARAEN);
        }
        return new_ast_return(expr_node);
    }
    return new_err_return(ERR_EXPECTED_EXPR);
}

ASTReturn* new_ast_return(TokenNode* node) {
    ASTReturn* ret = malloc(sizeof(ASTReturn));
    ret->tag = TAG_AST;
    ret->value.ast = node;
    return ret;
}

ASTReturn* new_err_return (Error err) {
    ASTReturn* ret = malloc(sizeof(ASTReturn));
    ret->tag = TAG_ERR;
    ret->value.err = err;
    return ret;
}

// This is a mess
// Come back to it, this might be helpful for algebraic expression analysis
// I thought I was being clever, I wasn't. Maybe there's something useful in here though


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
// Builds a tree of ops_tokens
// Error op_expr(TokenNode* parent, Vec* ops_tokens, int i) {
//     if (ops_tokens->len == i)
//         return ERR_NONE;
//     TokenNode* op_node = new_token_node(get_vec(ops_tokens, i));
//     printf("op expr\n");
//     print_tok_type(op_node->token->type);
//     push_vec(parent->children, op_node);
//     // printf("parent length:%zu\n", parent->children->len);
//     // print_token_node(parent);
//     assert(parent->children->len != 0);
//     printf("%d\n\n", i);
//     if (i != 0) { // Checks if parent is the assignment node
//         if (parent->children->len == 1)
//             return op_expr(parent, ops_tokens, i + 1); // This is causing parent node buildup
//         else if (parent->children->len == 2)
//             return op_expr(op_node, ops_tokens, i + 1);
//         else return ERR_FORMATTED_AST_WRONG; // This is true
//     } else return op_expr(op_node, ops_tokens, 1);
    
// }

// Adds value leaves to a tree of ops_tokens
// Right to left, two per leaf
// Error val_expr(TokenNode* parent, Vec* vals, int i, Vec* id_list) {
//     printf("val expr called\n\n");
//     printf("\nPARENT: \n");
//     print_tok_type(parent->token->type);
//     printf("here\n");
//     if (vals->len == i) return ERR_NONE;
//     Error result = ERR_NONE;

//     TokenNode* left = get_vec(parent->children, 0);
//     TokenNode* right = get_vec(parent->children, 1);

//     // There's an issue with parent(first right)'s token type
//     // if (parent->token- >type == NULL) printf("null");
//     // if (right == NULL && left == NULL) printf("here\n");
//     if (!filled(parent->token->type) && parent->children->len < 2) {
//     // if (right == NULL && left == NULL && !filled(parent->token->type)) { // parent is an op leaf 
//         printf("op node\n");
//         push_vec(parent->children, left);
//         i++;
//         result = val_expr(parent, vals, i + 1, id_list);
//         arithmetic_simplify()
//         if (result != ERR_NONE || result != ERR_NOT)
//             return result;
//     }
//     if (right != NULL) {
//         printf("right\n");
//         if (right->token->type == TOK_ID && !idck(id_list, right->token->value))
//             return ERR_ID_NOT_VALID;
//         result = val_expr(right, vals, i, id_list);
//         if (result != ERR_NONE) {
//             return result;
//         }
//     } else printf("RIGHT NOT NULL\n");
//     if (left != NULL) {
//         printf("left\n");
//         if (left->token->type == TOK_ID && !idck(id_list, left->token->value))
//             return ERR_ID_NOT_VALID;
//         result = val_expr(left, vals, i, id_list);
//         if (result != ERR_NONE) {
//             return result;
//         }
//     }
//     return ERR_NONE;
// }

// Modifies ret_buff like a return value
// Error format_expression(Tokenizer* t, Vec* id_list, Vec* ret_buff) {
//     printf("STARTED EXPR FORMATTING\n");
//     Token* tok = get_next_token(t);
//     // Both of these are Tokens
//     Vec* ops_tokens = new_vec(2);
//     Vec* vals_tokens = new_vec(2);
//     while (tok->type != TOK_SEMI && tok->type != TOK_NONE) {
//         print_token(tok);
//         if (is_op(tok->type)) {
//             printf("formatted op\n");
//             push_vec(ops_tokens, tok);
//             tok = get_next_token(t);
//         }
//         else if (id_exists(tok) || (tok->type == TOK_NUM)) {
//             printf("formatted id\n");
//             push_vec(vals_tokens, tok);
//             tok = get_next_token(t);
//         }
//         else {
//             if (vals_tokens->len == ops_tokens->len + 1) {
//                 printf("this\n");
//                 goto DONE;
//           }
//             printf("not\n");
//             return ERR_NOT;
//         }
//     } 
//     printf("found semi-colon\n");
//     DONE:
//     // printf("done\n");
//     push_vec(ret_buff, ops_tokens);
//     // printf("here\n");
//     push_vec(ret_buff, vals_tokens);
//     // printf("hereish\n");
//     return ERR_NONE;
// }

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
        Error expr_result = handle_expr(t, comparitor_node, id_list);
        if (expr_result == ERR_NONE) {
            Token* comparitor = get_next_token(t);
            print_tok_type(comparitor->type);
            comparitor_node->token = comparitor;
            // expr_result = handle_expr(t, comparitor_node, id_list);
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
