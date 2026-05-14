#include <zephyr/device.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(sensor_init, CONFIG_APP_LOG_LEVEL);

const struct device *get_sht3x_device(void) {
    const struct device *dev = DEVICE_DT_GET_ANY(sht3xd);
	if (dev && device_is_ready(dev)) return dev;
    LOG_ERR("SHT3x sensor is not ready");
    return NULL;
}

const struct device *get_ds18b20_device(void) {
    const struct device *dev = DEVICE_DT_GET_ANY(ds18b20);
    if (dev && device_is_ready(dev)) return dev;
    LOG_ERR("DS18B20 sensor is not ready");
    return NULL;
}

const struct device *get_demo_sensor_device(void) {
    const struct device *dev = DEVICE_DT_GET(DT_NODELABEL(example_sensor));
    if (dev && device_is_ready(dev)) return dev;
    LOG_ERR("Example (mock) sensor is not ready");
    return NULL;
}
