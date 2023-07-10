#include<stdio.h>
#include<stdlib.h>
#include"vector.h"

typedef struct Vector 
{
    size_t len;
    size_t capacity;
    void** data;
} Vector;

Vector* new_vec(size_t initial_capacity)
{
    Vector* vec = malloc(sizeof(Vector));
    if (vec != NULL) {
        vec->data = malloc(initial_capacity * sizeof(void*));
        vec->capacity = initial_capacity;
        vec->len = 0;
    }
    return vec;
}

void* get_vec(Vector *vec, size_t n) 
{
    if(vec && n < vec->capacity) {
        return vec->data[n];
    }
    /* return some error value, i'm doing -1 here, 
     * std::vector would throw an exception if using at() 
     * or have UB if using [] */
    return -1;
}

void set_vec(Vector* vec, void* item, size_t index) 
{
    if (vec && index > vec->capacity){
        vec->data[index] = item;
    } 
    else 
    {
        realloc_vec(vec, index * 2);
    }
}

void push_vec(Vector* vec, void* item)
{
    if (vec->len + 1 >= vec->capacity) {
        realloc_vec(vec, vec->capacity * 2);
    }
    vec->data[vec->len] = item;
    vec->len++;
}

void realloc_vec(Vector* vec, size_t new_size)
{
    if (vec) {
        void* new_data = realloc(vec->data, new_size * sizeof(void*));
        vec->capacity = new_size;
        vec->data = new_data;
    }
}

void free_vec(Vector* vec)
{
    if (vec) {
        free(vec->data);
        free(vec);
    }
}