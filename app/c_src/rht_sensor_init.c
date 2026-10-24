#include <zephyr/device.h>
#include <zephyr/logging/log.h>
#include <zephyr/drivers/i2c.h>

struct sht3xd_config {
	struct i2c_dt_spec bus;
};

enum chip_type {type_ds18b20, type_ds18s20};
struct ds18b20_config {
	const struct device *bus;
	uint8_t family;
	uint8_t resolution;
	enum chip_type chip;
};

LOG_MODULE_REGISTER(sensor_init, CONFIG_APP_LOG_LEVEL);

const struct device *get_sht3x_device(void) {
    const struct device *dev = DEVICE_DT_GET_ONE(sensirion_sht3xd);
    if (dev) {
        const struct sht3xd_config *cfg = dev->config;
        /* Print the actual address stored in the driver's config structure */
        printk("SHT3x configured I2C addr: 0x%02X\n", cfg->bus.addr);
    }
    if (dev && device_is_ready(dev)) return dev;
    LOG_ERR("SHT3x sensor is not ready");
    return NULL;
}

const struct device *get_ds18b20_device(void) {
    const struct device *dev = DEVICE_DT_GET_ONE(maxim_ds18b20);
    if (dev) {
        const struct ds18b20_config *cfg = dev->config;
        /* Print the actual address stored in the driver's config structure */
        printk("DS18B20 config type: 0x%02X\n", cfg->chip);
    }
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
