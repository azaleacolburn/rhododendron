typedef enum Error {
    MISSING_DECLARATION_ERR,
    TYPE_ERR,
    ARITHMETIC_OPERATOR_ERR,
    KEYWORD_PLACEMENT_ERR,
    NONE_ERR
} Error;

char* error_message(Error err);