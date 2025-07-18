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

#ifdef __cplusplus
}
#endif

#endif
