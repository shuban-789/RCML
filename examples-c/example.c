#include <stdio.h>
#include "rcl.h"

extern void path_x(float t, float* out);
extern void path_diag(float t, float* out);
extern float limit2(
    float (*f)(float, float),
    float px,
    float py,
    void (**paths)(float, float*),
    unsigned long num_paths
);

float square(float x) {
    return x * x;
}

float dy_dx(float x, float y) {
    return x + y;
}

float fxy(float x, float y) {
    return (x * x + y * y) / (x * x + y * y + 1.0f);
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

    void (*paths[2])(float, float*) = { path_x, path_diag };
    float limit = limit2(fxy, 0.0f, 0.0f, paths, 2);
    printf("Limit of fxy as (x,y) -> (0,0) = %f\n", limit);

    return 0;
}
