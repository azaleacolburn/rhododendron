#include<stdio.h>
#include<stdlib.h>
#include"error.h"

char* error_message(Error err) {
    switch (err) {
        case ERR_MISSING_DECLARATION:
            return "Expected '='";
        case ERR_TYPE:
            return "Unsupported type, supported type: char, int";
        case ERR_ARITHMETIC_OPERATOR:
            return "Arithmetic operator error, supported operators: +, -, *, /";
        case ERR_KEYWORD_PLACEMENT:
            return "Expected expression, found keyword";
        case ERR_NONE:
            return "No Error";
    }
}