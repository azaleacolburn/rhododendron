#include"cgen.h"

// #define add_comma(ret) ret = strcat(ret, ", ")
// // Inits new "ret"
#define TOK_IS_OP(t) t == TOK_B_AND || t == TOK_B_OR || t == TOK_B_XOR || t == TOK_ADD || t == TOK_SUB || t == TOK_DIV || t == TOK_MUL

#define mov(reg, val) \
    char* ret; \
    sprintf(ret, "mov %s, %s", reg, val);
#define push(reg, size) \
    char* ret; \
    sprintf(ret, "str %s, [sp, #-%s]", reg, size);
#define pop(reg, size) \
    char* ret; \
    sprintf(ret, "str %s, [sp], #%s", reg, size);

// generates a single mov statement
// recurses or calls other functions if it needs to evaluate an expression
// recursion is key
// // this system can only handle one ret at a time, which is fine cause recursion
// char* declr_code_gen(RegisterTracker* reg_t, TokenNode* node) {
//     char* reg;
//     int* reg_int = assign_register(reg_t, REG_W);
//     if (reg_int == NULL) {
//         printf("reg int null\n");
//     }
//     printf("reg: %d\n", *reg_int);
//     itoa(*reg_int, reg, 10);
//     printf("past itoa\n");
//     mov(reg, (char*)node->token->value);
//     printf("ret: %s", ret);
//     free(reg);
//     // free(ret);
//     return ret;
// }

// Right now we check the parent 
char* expr_code_gen(TokenNode* parent, RegisterTracker* tracker) {
    // An operator represents the begining of an expression
    char* ret = "";
    
    if (TOK_IS_OP(parent->token->type)) {
        Vec* children = parent->children->data;
        TokenNode* left = get_vec(children, 0);
        TokenNode* right = get_Vec(children, 1);
        char* left = expr_code_gen(left, tracker);
        sprintf(ret, "%s\n%s", ret, left);
        char* right = expr_code_gen(right, tracker);
        sprintf(ret, "%s\n%s", ret, right);
        // Then perform operation and mov into new register that we keep track of
        // might refactor to deal with parent's roots, and not recurse parent into value
        switch (parent->token->type) {
            case TOK_B_AND:
                int reg_A = *assign_register(tracker, REG_XZR); // assign register needs a refactor for this to work
                int reg_B = *assign_register(tracker, REG_XZR);
                sprintf(ret, "%s\nadd x%d, x%d, x%d", ret, reg_A, reg_B, reg_A);
                free_reg(tracker, REG_XZR, reg_B);
        }
    } else {
        // Now we add things and do the code generation
        char* reg_buff;
        itoa(*assign_register(tracker, REG_XZR), reg_buff, 10);
        sprintf(ret, "mov %s, %s", reg_buff, (char*)parent->token->value);

    }
    return ret;
}

char* itoa(int num, char* buffer, int base) // Issue seems to be with itoa function returning a char* instead of a literal
{
    int current = 0;
    if (num == 0) 
    {
        buffer[0] = '0';
        buffer[1] = '\0';
        return buffer;  
    }
    int num_digits = 0;  
    if (num < 0) 
    {  
        if (base == 10) 
        {  
            num_digits++;
            buffer[0] = '-';
            current ++; 
            num *= -1;
        } else return NULL;
    }  
    num_digits += (int)floor(log(num) / log(base)) + 1;  
    while (current < num_digits)   
    {  
        int base_val = (int) pow(base, num_digits-1-current);  
        int num_val = num / base_val;  
        char value = num_val + '0';  
        buffer[current] = value;  
        current ++;  
        num -= base_val * num_val;  
    }  
    buffer[current] = '\0';  
    return buffer;  
}
