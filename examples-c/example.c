#include <stdio.h>
#include "rcl.h"

float square(float x) {
    return x * x;
}

float dy_dx(float x, float y) {
    return x + y;
}

int main() {
    float d = derive(2.0f, square);
    printf("f'(x) at x=2: %f\n", d);

    float i = integrate(0.0f, 1.0f, square);
    printf("Integral of x^2 from 0 to 1: %f\n", i);

    float y_final = euler(0.0f, 1.0f, 1.0f, 0.01f, dy_dx);
    printf("Euler approximation y(1): %f\n", y_final);

    float l = limit(0.0f, square);
    printf("Limit of x^2 as x -> 0: %f\n", l);

    bool is_l = verify_limit(0.0f, 0.0f, square);
    printf("Is limit verified: %s\n", is_l ? "true" : "false");

    return 0;
}
