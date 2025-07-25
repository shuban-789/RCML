#include <stdio.h>
#include "rcl.h"

float f(float x) {
    return 1/x;
}

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

    float vd1[] = {1.0, 2.0};
    float vd2[] = {3.0, 4.0};
    float dot_prod_2;
    dot(vd1, vd2, 2, &dot_prod_2);
    printf("2D vector dot product is %f\n", dot_prod_2);

    float vd3[] = {1.0, 2.0, 3.0};
    float vd4[] = {4.0, 5.0, 6.0};
    float dot_prod_3;
    dot(vd3, vd4, 3, &dot_prod_3);
    printf("3D vector dot product is %f\n", dot_prod_3);

    float vc1[] = {1.0, 2.0};
    float vc2[] = {3.0, 4.0};
    float cross_prod_2;
    cross2(vc1, vc2, &cross_prod_2);
    printf("2D vector cross product is %f\n", cross_prod_2);

    float vc3[] = {1.0, 2.0, 3.0};
    float vc4[] = {4.0, 5.0, 6.0};
    float cross_prod_3[3];
    cross3(vc3, vc4, cross_prod_3);
    printf("3D vector cross product is <%f, %f, %f>\n", cross_prod_3[0], cross_prod_3[1], cross_prod_3[2]);

    float approx = taylor(2.0f, 2.0f, 2, f);
    printf("2nd Degree Taylor Series Approximation for 1/x with x = 2, centered at x = 1: %f\n", approx);

    float projected_vec[] = {7.0, 14.0, 21.0};
    float target_vec[] = {4.0, 5.0, 6.0};
    float projection[3];
    project(projected_vec, target_vec, 3, projection);
    printf("Vector <1.0, 2.0, 3.0> projected onto <4.0, 5.0, 6.0> forms the projection vector <%f, %f, %f>\n", projection[0], projection[1], projection[2]);

    return 0;
}