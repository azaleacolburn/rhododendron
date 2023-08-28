// Here we store the registers being used and by what
// Ask Andrew for help
#include<stdlib.h>
#include"parser.h"
typedef struct RegisterTracker {
    Vec* x;
    Vec* w;
    int wxr;
    int xzr;
    int sp;
} RegisterTracker;

typedef enum regType {
    REG_ARG_RET,
    REG_TEMP,
    REG_CALLEE,
    REG_FP,
    REG_LINK,
    REG_SP,
    REG_W,
    REG_WXR,
    REG_XZR,
    REG_SP
} regType;

RegisterTracker* new_reg_tracker();

void free_register_tracker(RegisterTracker* tracker);

int* assign_register(RegisterTracker* tracker, regType type);