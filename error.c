#include<stdio.h>
#include<stdlib.h>
#include"error.h"

char* error_message(Error err) {
    // printf("here\n");
    switch (err) {
        case ERR_MISSING_DECLARATION:
            return "Expected '='";
        case ERR_TYPE:
            return "Unsupported type, supported type: char, int";
        case ERR_ARITHMETIC_OPERATOR:
            return "Arithmetic operator error, supported operators: +, -, *, /";
        case ERR_KEYWORD_PLACEMENT:
            return "Unexpected keyword";
        case ERR_NONE:
            return "No Error";
        case ERR_NOT:
            return "No Error, just checked out the wrong path";
        case ERR_EXPECTED_CONDITION:
            return "Expected condition, maybe you're missing a '(' or ')', maybe an expresssion is missing";
        case ERR_EXPECTED_COMPARITOR:
            return "Expected a comparitor, maybe you're missing a '==' or a '!='";
        case ERR_EXPECTED_EXPR:
            return "Expected an expression";
        case ERR_ID_NOT_VALID:
            return "Found undeclared id";
        case ERR_EXPECTED_ASSIGNMENT:
            return "Expected a assignment, maybe you forgot a '='";
        case ERR_FORMATTED_AST_WRONG:
            return "Internal expression parsing problem";
        case ERR_EXEPCTED_ID:
            return "Expected identifier";
        case ERR_MISSING_O_PARAEN:
            return "Missing '('";
        case ERR_MISSING_C_PARAEN:
            return "Missing ')'";
        case ERR_MISSING_SEMICOLON:
            return "Missing ';'";
        case ERR_EXPECTED_STATEMENT:
            return "Expected statement";
        case ERR_EXPECTED_DECLARATION:
            return "Expected variable declaration";
    }
}