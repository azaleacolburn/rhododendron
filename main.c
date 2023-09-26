#include<stdio.h>
#include<stdlib.h>
#include"cgen.h" // this contains all other header files

Vec* readFile(char* path)
{
    FILE* fp;
    fp = fopen(path, "r");
    if (fp == NULL) {
        printf("Error, path: %s doesn't exist\n", path);
        exit(1);
    }
    fseek(fp, 0, SEEK_END);
    long fsize = ftell(fp);
    fseek(fp, 0, SEEK_SET);  /* same as rewind(f); */
    char* buff = malloc(fsize + 1);
    fread(buff, fsize, 1, fp);
    fclose(fp);
    buff[fsize] = 0;
    Vec* ret = new_vec(2);
    push_vec(ret, buff);
    push_vec(ret, &fsize);
    return ret;
}

void test_tokenizer(void) {
    Tokenizer* t = new_tokenizer("first second third fourth");
    print_token(get_next_token(t));
    print_token(get_next_token(t));
    print_token(get_next_token(t));
    print_token(get_next_token(t));
    
    free_tokenizer(t);
}

int main(int argc, char* argv[]) {
    printf("%s", argv[0]);
    // test_tokenizer();
    Vec* contents = readFile("/Users/elocolburn/CompSci3/floralcc/test.txt");
    Vec* ret = program(get_vec(contents, 0), *(long*)get_vec(contents, 1));
    Error* success_code = get_vec(ret, 0);
    TokenNode* node = get_vec(ret, 1);
    printf("Success Code: %s\n", error_message(*success_code));
    printf("\n");
    printf("%s", code_gen(node));
}
