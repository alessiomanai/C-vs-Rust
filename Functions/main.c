#include <stdio.h>
#include <stdlib.h>

int addAB(int a, int b){

    return a + b;

}

int mulAB(int a, int b){

    return a * b;

}

int main()
{
    int a = 2;
    int b = 2;

    printf("%d\n", addAB(a,b));
    printf("%d\n", mulAB(a,b));

    return 0;
}
