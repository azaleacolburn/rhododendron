#include<stdio.h>
#include<stdlib.h>

typedef struct Vector Vector;

Vector* new_vec(size_t initial_capacity);

void* get_vec(Vector *vec, size_t n);

void set_vec(Vector* vec, void* item, size_t index);

void push_vec(Vector* vec, void* item);

void realloc_vec(Vector* vec, size_t new_size);

void free_vec(Vector* vec);