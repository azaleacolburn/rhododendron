// Ask Andrew about this, I think I'm doing this wrong
#include"cgen.h"

#define add_comma(ret) ret = strcat(ret, ", ")
// Inits new "ret"
#define mov(reg, val) \
    char* ret = strcat("mov ", reg); \
    add_comma(ret); \
    ret = strcat(ret, val); \

// generates a single mov statement
// recurses or calls other functions if it needs to evaluate an expression
// recursion is key
char* declr_code_gen(RegisterTracker* reg_t, TokenNode* node) {
    char* reg;
    int* reg_int = assign_register(reg_t, REG_W);
    if (reg_int == NULL) {
        printf("reg int null\n");
    }
    printf("reg: %d\n", *reg_int);
    itoa(*reg_int, reg, 10);
    printf("past itoa\n");
    mov(reg, (char*)node->token->value);
    printf("ret: %s", ret);
    free(reg);
    // free(ret);
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
