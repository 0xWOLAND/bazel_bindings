#include "math.h"

extern "C" {
    double math_add(double a, double b) {
        return a + b;
    }

    double math_sub(double a, double b) {
        return a - b;
    }

    double math_mul(double a, double b) {
        return a * b;
    }

    double math_div(double a, double b) {
        return a / b;
    }
} 