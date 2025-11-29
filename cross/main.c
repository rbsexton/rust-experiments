#include <stdio.h>

#include "library_base.h"

#include "rustlib_header.h"


int main() {
    printf("Hello, World!\n");

    printf("One plus = %d\n", add_one(1));

    printf("Rust plus = %d\n", plus_two(1));

    printf("Rust plus = %d\n", plus_three(1));

    return 0;
}
