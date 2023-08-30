#include<stdio.h>
#include<stdlib.h>
#include"vector.h"

Vec* new_vec(size_t initial_capacity) {
    Vec* vec = malloc(sizeof(Vec));
    if (vec != NULL) {
        vec->data = calloc(initial_capacity, sizeof(void*));
        vec->capacity = initial_capacity;
        vec->len = 0;
    } else return NULL;
    return vec;
}

void* get_vec(Vec *vec, size_t n) {
    if(vec && n < vec->capacity)
        return vec->data[n];
    return NULL;
}

void set_vec(Vec* vec, void* item, size_t index) {
    if (vec && index < vec->capacity)
        vec->data[index] = item;
    else realloc_vec(vec, index * 2);
}

void push_vec(Vec* vec, void* item) {
    // or maybe deref vec
    if (vec == NULL) {
        printf("null vec\n");
        return;
    }
    if (vec->len + 1 >= vec->capacity) {
        realloc_vec(vec, vec->capacity * 2);
    }
    vec->data[vec->len] = item;
    vec->len++;
}

void realloc_vec(Vec* vec, size_t new_size) {
    if (vec) {
        void* new_data = realloc(vec->data, new_size * sizeof(void*));
        vec->capacity = new_size;
        vec->data = new_data;
    }
}

void free_vec(Vec* vec) {
    if (vec) {
        free(vec->data);
        free(vec);
    }
}