#include <zephyr/device.h>
#include <zephyr/drivers/sensor.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(sensor_init, CONFIG_APP_LOG_LEVEL);

const struct device * is_sensor_ready(void)
{
    static const struct device *sensor;

    // Auto-select the SHT3x sensor, or use the example_sensor for debugging/development
    sensor = DEVICE_DT_GET(DT_NODELABEL(sht3xd));
	if (!device_is_ready(sensor)) {
		LOG_ERR("SHT3x sensor is not ready");
        sensor = NULL;
	}

    if (sensor == NULL) {
        sensor = DEVICE_DT_GET(DT_NODELABEL(example_sensor));
        if (!device_is_ready(sensor)) {
            LOG_ERR("Example sensor is not ready");
            sensor = NULL;
        }
    }

    LOG_INF("Sensor device selected: %s", sensor->name);

    if (sensor == NULL) {
        return NULL;
    }
	return sensor;
}