#include"typeck.h"

VarType num_type_ck(char* num) {
    size_t s = sizeof(atoi(num));
    if (s == 1) return VAR_CHAR;
    else if (s == 4) return VAR_INT;
    else return VAR_NONE;
}