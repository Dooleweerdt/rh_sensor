#include <zephyr/zbus/zbus.h>
#include "app/sensor_data.h"

// Define the zbus subscribers here:
ZBUS_SUBSCRIBER_DEFINE(wifi_sub, 1);
ZBUS_SUBSCRIBER_DEFINE(display_sub, 1);

// C handles the complex Zbus channel definition
ZBUS_CHAN_DEFINE(sensor_data_channel, struct sensor_msg, NULL, NULL, ZBUS_OBSERVERS(wifi_sub, display_sub), ZBUS_MSG_INIT(0));

// Publish function from Rust
int zbus_bridge_publish_rht(struct sensor_msg *msg) {
    return zbus_chan_pub(&sensor_data_channel, msg, K_MSEC(100));
}

// Subscribe: Wait and read function from Rust
int zbus_bridge_wait_read(const struct zbus_observer *sub, 
                         const struct zbus_channel **chan,
                         struct sensor_msg *msg) {
    return zbus_sub_wait_msg(sub, chan, msg, K_FOREVER);
}
