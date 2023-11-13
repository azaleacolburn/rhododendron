typedef struct Vec 
{
    size_t len;
    size_t capacity;
    void** data;
} Vec;

Vec* new_vec(size_t initial_capacity);

void* get_vec(Vec* vec, size_t n);

void set_vec(Vec* vec, void* item, size_t index);

void push_vec(Vec* vec, void* item);

void realloc_vec(Vec* vec, size_t new_size);

void free_vec(Vec* vec);