typedef enum TokType {
    TOK_PROGRAM,
    TOK_IF,
    TOK_FOR,
    TOK_WHILE,
    TOK_DECLARE,
    TOK_NUM,
    TOK_ADD,
    TOK_MUL,
    TOK_SUB,
    TOK_DIV,
    TOK_ASSIGN,
    TOK_ID,
    TOK_STATEMENT,
    TOK_CONDITION,
    TOK_EQ_CMP,
    TOK_NEQ_CMP,
    TOK_B_OR,
    TOK_B_AND,
    TOK_B_XOR,
    TOK_B_OR_EQ,
    TOK_B_AND_EQ,
    TOK_B_XOR_EQ,
    TOK_SUB_EQ,
    TOK_ADD_EQ,
    TOK_DIV_EQ,
    TOK_MUL_EQ,
    TOK_O_PAREN,
    TOK_C_PAREN,
    TOK_O_CURL,
    TOK_C_CURL,
    TOK_NOT, // Fix maybe
    TOK_NONE // Not a token
} TokType;

typedef struct Token {
    TokType type;
    void* value;
} Token;

Token* new_token(TokType type) {
    Token* tok = malloc(sizeof(Token));
    tok->type = type;
    return tok;
};

void print_token(Token* tok);

// Figure out how to check the contents of the vector. Maybe length. goto may lead to something
void free_token(Token* root) {
    free(root->type);
    free(root->value);
    free(root);
}