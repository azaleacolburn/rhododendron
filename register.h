// Here we store the registers being used and by what
// Ask Andrew for help
#include"vector.h"
typedef struct RegisterTracker {
    Vec* x;
    Vec* w;
    int wxr;
    int xzr;
    int sp;
} RegisterTracker;

typedef enum regType {
    REG_X,
    REG_W,
    REG_WXR,
    REG_XZR,
    REG_SP
} regType;