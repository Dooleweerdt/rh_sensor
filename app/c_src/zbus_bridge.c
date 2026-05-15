#include <zephyr/zbus/zbus.h>
#include "app/sensor_data.h"

// 1. Stub the callback functions (even if they do nothing for now)
static void wifi_cb(const struct zbus_channel *chan) { /* Wakes up Rust thread */ }
static void display_cb(const struct zbus_channel *chan) { /* Updates display */ }

// 2. Define the listeners
ZBUS_LISTENER_DEFINE(wifi_sub, wifi_cb);
ZBUS_LISTENER_DEFINE(display_sub, display_cb);

// C handles the complex Zbus channel definition
ZBUS_CHAN_DEFINE(sensor_data_channel, struct sensor_msg, NULL, NULL, ZBUS_OBSERVERS(wifi_sub, display_sub), ZBUS_MSG_INIT(0));

// Simple function for Rust to call
int zbus_bridge_publish_rht(float t, float h) {
    struct sensor_msg msg = { .temp = t, .hum = h };
    return zbus_chan_pub(&sensor_data_channel, &msg, K_MSEC(100));
}