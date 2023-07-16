typedef struct Tokenizer {
    char* string;
} Tokenizer;

char* get_next_token(Tokenizer* t);

// 1 if there are more tokens
// 0 if there aren't
int has_more_tokens(Tokenizer* t);

Tokenizer* new_tokenizer(char* string);

void free_tokenizer(Tokenizer* t);

// Params: string, buffer to be copied into, start and end indexed
void slice(const char* str, char* result, size_t start, size_t end);

int check_delimeter(char c);

char* str_remove(char* str, int start_index, int end_index);