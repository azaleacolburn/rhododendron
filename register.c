#include"register.h"

RegisterTracker* new_reg_tracker() {
    RegisterTracker* tracker = malloc(sizeof(RegisterTracker));
    tracker->w = new_vec(31);
    tracker->x = new_vec(31);
    return tracker;
}

void free_register_tracker(RegisterTracker* tracker) {
    free_vec(tracker->w);
    free_vec(tracker->x);
    free(tracker);
}

// Returns which slot it's in if it's a x or w register, or NULL if otherwise
//i will be leaked, right
int* assign_register(RegisterTracker* tracker, regType type) {
    int* ret = malloc(sizeof(int));
    int* one = malloc(sizeof(int));
    *one = 1;
    switch (type) {
        case REG_W: {
            push_vec(tracker->w, one);
            *ret = tracker->w->len - 1;
            return ret;
        }
        case REG_ARG_RET: {
            if (get_vec(tracker->x, 8) == NULL) {
                push_vec(tracker->x, one);
                *ret = tracker->x->len - 1;
                return ret;
            } else {
                // send signal to push not used registers to memory
            }
        }
        case REG_TEMP:  {
            for (int i = 8; i < 31; i++) {
                if (get_vec(tracker->x, i) == NULL) {
                    set_vec(tracker->x, one, i);
                    *ret = i;
                    return ret;
                }
            }
        }
        case REG_CALLEE:  {
            for (int i = 19; i < 31; i++) {
                if (get_vec(tracker->x, i) == NULL) {
                    set_vec(tracker->x, one, i);
                    *ret = i;
                    return ret;
                }
            }
        }
        case REG_SP:  {
            tracker->sp = 1;
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
        case REG_FP: {
            
        }
        case REG_LINK: {

        }
    }
    return NULL;
}