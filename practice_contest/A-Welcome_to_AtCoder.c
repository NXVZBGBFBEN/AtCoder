//
// Created by NXVZBGBFBEN on 2022/10/30.
//

#include <stdio.h>

int main(void) {
    int a, b, c;
    int n;
    char s[99];
    scanf("%d", &a);
    scanf("%d %d", &b, &c);
    scanf("%s", s);
    n = a + b + c;
    printf("%d %s", n, s);
    return 0;
}