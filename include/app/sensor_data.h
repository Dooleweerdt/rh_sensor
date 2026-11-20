#pragma once
#include <stdint.h>
#include <float.h>

// sensor_data.h
struct sensor_msg {
    const uint8_t* name;
    uint8_t name_len;
    float temp;
    float hum;
};
