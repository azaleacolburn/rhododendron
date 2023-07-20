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
    TOK_SEMI,
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

void print_tok_type(TokType type) {
    char* p;
    switch (type) {
        case TOK_ADD:
            p = "+";
            break;
        case TOK_ADD_EQ:
            p = "+=";
            break;
        case TOK_ASSIGN:
            p = "=";
            break;
        case TOK_B_AND:
            p = "&";
            break;
        case TOK_B_AND_EQ:
            p = "&=";
            break;
        case TOK_B_OR: 
            p = "|";
            break;
        case TOK_B_OR_EQ:
            p = "|=";
            break;
        case TOK_B_XOR:
            p = "^";
            break;
        case TOK_B_XOR_EQ:
            p = "^=";
            break;
        case TOK_C_CURL:
            p = "}";
            break;
        case TOK_C_PAREN:
            p = ")";
            break;
        case TOK_CONDITION:
            p = "condition: ";
            break;
        case TOK_DECLARE:
            p = "declaration";
            break;
        case TOK_DIV:
            p = "/";
            break;
        case TOK_DIV_EQ:
            p = "/=";
            break;
        case TOK_EQ_CMP:
            p = "==";
            break;
        case TOK_FOR:
            p = "for";
            break;
        case TOK_ID:
            p = "id: ";
            break;
        case TOK_IF:
            p = "if";
            break;
        case TOK_MUL:
            p = "*";
            break;
        case TOK_MUL_EQ:
            p = "*=";
            break;
        case TOK_NEQ_CMP:
            p = "!=";
            break;
        case TOK_NONE:
            p = "";
            break;
        case TOK_NOT:
            p = "!";
            break;
        case TOK_NUM:
            p = "num: ";
            break;
        case TOK_O_CURL:
            p = "{";
            break;
        case TOK_O_PAREN:
            p = "(";
            break;
        case TOK_PROGRAM:
            p = "program start: ";
            break;
        case TOK_STATEMENT:
            p = "statement: ";
            break;
        case TOK_SUB:
            p = "-";
            break;
        case TOK_SUB_EQ:
            p = "-=";
            break;
        case TOK_WHILE:
            p = "while";
            break;
        case TOK_SEMI:
            p = ";";
            break;
    }
    printf("Token Type: %s\n", p);
}

void print_token(Token* tok) {
    print_tok_type(tok->type);
    printf("%s\n", (char*)tok->value);
};

// Figure out how to check the contents of the vector. Maybe length. goto may lead to something
void free_token(Token* root) {
    free(root->type);
    free(root->value);
    free(root);
}