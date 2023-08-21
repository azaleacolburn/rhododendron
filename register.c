#include"register.h"

RegisterTracker* new_reg_tracker() {
    RegisterTracker* tracker = malloc(sizeof(RegisterTracker));
    for (int i = 0; i < 31; i++) {
        push_vec(tracker->x, NULL);
        push_vec(tracker->w, NULL);
    }
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
    switch (type) {
        case REG_W:  {
            push_vec(tracker->w, 1);
            return tracker->w->len;
        }
        case REG_ARG_RET:  {
            for (int i = 0; i < 31; i++) {
                if (get_vec(tracker->x, i) == NULL) {
                    set_vec(tracker->x, 1, i);
                   return &i;
                }
            }
        }
        case REG_TEMP:  {
            for (int i = 8; i < 31; i++) {
                if (get_vec(tracker->x, i) == NULL) {
                    set_vec(tracker->x, 1, i);
                   return &i;
                }
            }
        }
        case REG_CALLEE:  {
            for (int i = 19; i < 31; i++) {
                if (get_vec(tracker->x, i) == NULL) {
                    set_vec(tracker->x, 1, i);
                   return &i;
                }
            }
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
    return "";
}