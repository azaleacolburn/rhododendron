#include<stdio.h>
#include"register.h"

RegisterTracker* new_reg_tracker() {
    RegisterTracker* tracker = malloc(sizeof(RegisterTracker));
    for (int i = 0; i < 31; i++) {
        push_vec(tracker->x, 0);
        push_vec(tracker->w, 0);
    }
    tracker->wxr = 1;
    tracker->xzr = 1;
    return tracker;
}

// Returns which slot it's in if it's a x or w register, or NULL if otherwise
int* assign_register(RegisterTracker* tracker, regType type) {
    switch (type) {
        case REG_W:  {
            push_vec(tracker->w, 1);
            return tracker->w->len;
        }
        case REG_X:  {
            push_vec(tracker->x, 1);
            return tracker->x->len;
        }
        case REG_SP:  {
            push_vec(tracker->sp, 1);
            return NULL;
        }
        case REG_WXR:  {
            tracker->wxr = 1;
            return NULL;
        }
        case REG_XZR:  {
            tracker->xzr = 1;
            return NULL;
        }
    }
}