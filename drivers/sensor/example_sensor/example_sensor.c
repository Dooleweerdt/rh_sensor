/*
 * Copyright (c) 2021 Nordic Semiconductor ASA
 * SPDX-License-Identifier: Apache-2.0
 */

#define DT_DRV_COMPAT zephyr_example_sensor

#include <zephyr/device.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/sensor.h>

#include <zephyr/logging/log.h>
LOG_MODULE_REGISTER(example_sensor, CONFIG_SENSOR_LOG_LEVEL);

struct example_sensor_data {
	int state;
};

struct example_sensor_config {
	struct gpio_dt_spec input;
};

static int16_t simulated_temp = 20;
static int32_t simulated_temp_decimal = 500000;
static int16_t simulated_humidity = 40;

static int example_sensor_sample_fetch(const struct device *dev,
				      enum sensor_channel chan)
{
	// Simulate changing temperature and humidity values
	if (simulated_temp_decimal >= 900000) {
		simulated_temp++;
		simulated_temp_decimal = 0;
	} else {
		simulated_temp_decimal += 100000; // Increment decimal part by .1
	}

	if (simulated_humidity > 80) {
		simulated_humidity = 40;
	} else {
		simulated_humidity += 1; // Increment humidity by 5%
	}
    if (simulated_temp > 30) {
		simulated_temp = 20;
	}
    return 0;
}

static int example_sensor_channel_get(const struct device *dev,
				     enum sensor_channel chan,
				     struct sensor_value *val)
{
	if (chan == SENSOR_CHAN_AMBIENT_TEMP) {
        val->val1 = simulated_temp; // 20-30 degrees
        val->val2 = simulated_temp_decimal; // .5
        return 0;
    } else if (chan == SENSOR_CHAN_HUMIDITY) {
        val->val1 = simulated_humidity; // 40-80% humidity
        val->val2 = 0;
        return 0;
    }
    return -ENOTSUP;
}

static DEVICE_API(sensor, example_sensor_api) = {
	.sample_fetch = &example_sensor_sample_fetch,
	.channel_get = &example_sensor_channel_get,
};

static int example_sensor_init(const struct device *dev)
{
	const struct example_sensor_config *config = dev->config;

	int ret;

	if (!device_is_ready(config->input.port)) {
		LOG_ERR("Input GPIO not ready");
		return -ENODEV;
	}

	ret = gpio_pin_configure_dt(&config->input, GPIO_INPUT);
	if (ret < 0) {
		LOG_ERR("Could not configure input GPIO (%d)", ret);
		return ret;
	}

	return 0;
}

#define EXAMPLE_SENSOR_INIT(i)						       \
	static struct example_sensor_data example_sensor_data_##i;	       \
									       \
	static const struct example_sensor_config example_sensor_config_##i = {\
		.input = GPIO_DT_SPEC_INST_GET(i, input_gpios),		       \
	};								       \
									       \
	DEVICE_DT_INST_DEFINE(i, example_sensor_init, NULL,		       \
			      &example_sensor_data_##i,			       \
			      &example_sensor_config_##i, POST_KERNEL,	       \
			      CONFIG_SENSOR_INIT_PRIORITY, &example_sensor_api);

DT_INST_FOREACH_STATUS_OKAY(EXAMPLE_SENSOR_INIT)
