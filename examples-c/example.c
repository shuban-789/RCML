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
extern void cross2(const float* ptr1, const float* ptr2, size_t len, float* out);
extern void cross3(const float* ptr1, const float* ptr2, size_t len, float* out);

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

    float v1[] = {1.0, 2.0};
    float v2[] = {3.0, 4.0};
    float cross_prod_2;
    cross2(v1, v2, 2, &cross_prod_2);
    printf("2D vector cross product is %f\n", cross_prod_2);

    float v3[] = {1.0, 2.0, 3.0};
    float v4[] = {4.0, 5.0, 6.0};
    float cross_prod_3[3];
    cross3(v3, v4, 3, cross_prod_3);
    printf("3D vector cross product is <%f, %f, %f>\n", cross_prod_3[0], cross_prod_3[1], cross_prod_3[2]);

    return 0;
}
