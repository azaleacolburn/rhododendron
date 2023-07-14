#include<stdio.h>
#include<stdlib.h>
#include"error.h"

char* error_message(Error err) {
    switch (err) {
        case MISSING_DECLARATION_ERR:
            return "Expected '='";
        case TYPE_ERR:
            return "Unsupported type, supported type: char, int";
        case ARITHMETIC_OPERATOR_ERR:
            return "Arithmetic operator error, supported operators: +, -, *, /";
        case KEYWORD_PLACEMENT_ERR:
            return "Expected expression, found keyword";
        case NONE_ERR:
            return "";
    }
}