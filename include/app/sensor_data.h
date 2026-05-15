#pragma once
#include <float.h>

// ID's here must match the Rust enum SensorId
enum sensor_id { SENSOR_SHT3X=0, SENSOR_ONEWIRE=1, SENSOR_MOCK=2 };

// sensor_data.h
struct sensor_msg {
    enum sensor_id id;
    float temp;
    float hum;
};
