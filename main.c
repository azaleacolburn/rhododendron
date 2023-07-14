#include<stdio.h>
#include<stdlib.h>
#include"rdp.h"

char* readFile(char* path)
{
    FILE* fp;
    static char buff[300];
    fp = fopen(path, "r");
    if (fp == NULL) {
        printf("Error, path: %s doesn't exist\n", path);
        exit(1);
    }
    fgets(buff, 300, fp);
    printf("File contents: %s\n", buff);
    return buff;
}

int main()
{
    char* contents = readFile("/Users/elocolburn/CompSci3/floralcc/text.txt");
    int success_code = program(contents);
    printf("Success Code: %d", success_code);
}