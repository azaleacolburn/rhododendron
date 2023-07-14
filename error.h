typedef enum Error {
    ERR_MISSING_DECLARATION,
    ERR_TYPE,
    ERR_ARITHMETIC_OPERATOR,
    ERR_KEYWORD_PLACEMENT,
    ERR_NONE
} Error;

char* error_message(Error err);