#include<stdlib.h>
#include<stdio.h>

typedef enum VarType {
    VAR_CHAR,
    VAR_INT,
    VAR_NONE
} VarType;

VarType num_type_ck(char* num);