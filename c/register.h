// Here we store the registers being used and by what
// Ask Andrew for help
#include<stdlib.h>
#include<stdio.h>
#include"parser.h"
typedef struct RegisterTracker {
    Vec* x;
    Vec* w;
    int sp;
} RegisterTracker;

typedef enum regType {
    REG_ARG_RET,
    REG_TEMP,
    REG_CALLEE,
    REG_FP,
    REG_LINK,
    REG_ZERO,
    REG_SP,
    // w regs
    REG_GEN_W,
    // x regs
    REG_GEN_X,
} regType;



RegisterTracker* new_reg_tracker();

void free_register_tracker(RegisterTracker* tracker);

void free_reg(RegisterTracker* tracker, regType type, int num);

int* assign_register(RegisterTracker* tracker, regType type);