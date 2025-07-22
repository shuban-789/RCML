#ifndef RCL_H
#define RCL_H

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

float derive(float c, float (*f)(float));
float limit(float c, float (*f)(float));
bool verify_limit(float c, float l, float (*f)(float));
float integrate(float a, float b, float (*f)(float));
float euler(float x_init, float y_init, float x_final, float step, float (*d)(float, float));
void path_x(float t, float* out);
void path_diag(float t, float* out);
float limit2(
    float (*f)(float, float),
    float px,
    float py,
    void (**paths)(float, float*),
    unsigned long num_paths
);
void cross2(const float* ptr1, const float* ptr2, size_t len, float* out);
void cross3(const float* ptr1, const float* ptr2, size_t len, float* out);

#ifdef __cplusplus
}
#endif

#endif
