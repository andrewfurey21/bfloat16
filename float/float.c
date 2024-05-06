#include "stdio.h"

int main() {
    float a = 10.0f;
    printf("Float: %x\n",*(int*)&a);
    return 0;
}
