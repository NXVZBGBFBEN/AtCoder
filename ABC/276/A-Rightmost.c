//
// Created by NXVZBGBFBEN on 2022/11/05.
//

#include <stdio.h>

int main(void){
    char S[100];
    char *a = "a";
    int last=0;
    scanf( "%s",S);
    for (int i=0; i < 100; ++i) {
        if (S[i]==*a){
            last=i+1;
        }
    }
    if(last!=0){
        printf("%d\n", last);
    }
    if(last==0){
        printf("-1\n");
    }
}