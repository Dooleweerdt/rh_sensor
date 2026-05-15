#pragma once
#include <stdint.h>
#include <float.h>

// sensor_data.h
struct sensor_msg {
    uint32_t id;
    float temp;
    float hum;
};
