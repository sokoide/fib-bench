#include <stdint.h>
#include <stdio.h>
#include <time.h>

uint64_t fib(uint64_t i) {
    if (i <= 1)
        return i;
    return fib(i - 1) + fib(i - 2);
}

int main() {
    uint64_t i;
    for (i = 40; i < 44; i++) {
        struct timespec tw1, tw2;
        clock_gettime(CLOCK_MONOTONIC, &tw1);
        uint64_t result = fib(i);
        clock_gettime(CLOCK_MONOTONIC, &tw2);

        double wall = 1000.0 * tw2.tv_sec + 1e-6 * tw2.tv_nsec -
                      (1000.0 * tw1.tv_sec + 1e-6 * tw1.tv_nsec);
        printf("%lf, fib(%llu)=%llu\n", wall, i, result);
    }
    return 0;
}
