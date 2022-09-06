#include "esp_log.h"

struct Pair {
    int a, b;
};

void abi_test_5(int p1, int p2, int p3, int p4, struct Pair p5) {
    ESP_LOGI("abi_test_5", "p5.a=%d, p5.b=%d", p5.a, p5.b);
}

void abi_test_6(int p1, int p2, int p3, int p4, int p5, struct Pair p6) {
    ESP_LOGI("abi_test_6", "p6.a=%d, p6.b=%d", p6.a, p6.b);
}

void abi_test_7(int p1, int p2, int p3, int p4, int p5, int p6, struct Pair p7) {
    ESP_LOGI("abi_test_7", "p7.a=%d, p7.b=%d", p7.a, p7.b);
}

void abi_test_8(int p1, int p2, int p3, int p4, int p5, int p6, int p7, struct Pair p8) {
    ESP_LOGI("abi_test_8", "p8.a=%d, p8.b=%d", p8.a, p8.b);
}

void abi_test_18(int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8,int p9,int p10,int p11,int p12,int p13,int p14,int p15,int p16,int p17,struct Pair p18) {
    ESP_LOGI("abi_test_18", "p18.a=%d, p18.b=%d", p18.a, p18.b);
}

double abi_test_double(double x) {
    ESP_LOGI("abi_test_double", "%f", x);
    return x;
}

float abi_test_float(float x) {
    ESP_LOGI("abi_test_float", "%f", x);
    return x;
}

struct Return4 {
    int a, b, c, d;
};

struct Return4 ret_abi_test_4() {
    struct Return4 r;
    r.a = 1;
    r.b = 2;
    r.c = 3;
    r.d = 4;
    return r;
}


void ret_abi_test_internal_4() {
    struct Return4 r = ret_abi_test_4();
    ESP_LOGI("ret_abi_test_internal_4", "a=%d, b=%d, c=%d, d=%d", r.a, r.b, r.c, r.d);
}

struct Return5 {
    int a, b, c, d, e;
};

struct Return5 ret_abi_test_5() {
    struct Return5 r;
    r.a = 1;
    r.b = 2;
    r.c = 3;
    r.d = 4;
    r.e = 5;
    return r;
}

void ret_abi_test_internal_5() {
    struct Return5 r = ret_abi_test_5();
    ESP_LOGI("ret_abi_test_internal_5", "a=%d, b=%d, c=%d, d=%d, e=%d", r.a, r.b, r.c, r.d, r.e);
}

void double_demo() {
    double x = 3.14;
    double y = x * 2.0;
    ESP_LOGI("double_demo_internal", "%f * 2.0 == %f", x, y);
}

void internal_clang_test() {
    // unsigned __int128 t = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    uint64_t t = 0xFFFFFFFFFFFFFFFF;

    ESP_LOGI("internal_clang_test", "%lld", t);
}