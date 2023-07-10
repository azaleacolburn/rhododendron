#include<stdio.h>
#include<stdlib.h>
#include"parser.h"

int main()
{
    char* contents = readFile("/Users/elocolburn/CompSci3/c-testing/compiler/text.txt");
    parse(contents);
}