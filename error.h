typedef enum Error {
    ERR_MISSING_DECLARATION,
    ERR_TYPE,
    ERR_ARITHMETIC_OPERATOR,
    ERR_KEYWORD_PLACEMENT,
    ERR_NONE, // Not real Error
    ERR_NOT, // Nor real Error
    ERR_EXPECTED_CONDITION,
    ERR_EXPECTED_COMPARITOR
} Error;

char* error_message(Error err);