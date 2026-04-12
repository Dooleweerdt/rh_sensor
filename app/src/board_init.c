/*
 * Copyright (c) 2021 Nordic Semiconductor ASA
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/init.h>
#include <zephyr/logging/log.h>

#include <app_version.h>

// Custom libraries
#include <app/drivers/wifi.h>
#include <app/wifi_settings.h>

LOG_MODULE_REGISTER(board_init, CONFIG_APP_LOG_LEVEL);

int board_init(void)
{
    printk("Zephyr - RH Sensor in Rust %s\n", STRINGIFY(APP_BUILD_VERSION));

    #if CONFIG_WIFI
    #if WIFI_CREDENTIALS
	int ret;

    // Wifi initialization - Test code
    wifi_init();

    do {
        // Connect to the WiFi network (blocking)
        ret = wifi_connect(WIFI_SSID, WIFI_PSK);
        if (ret < 0) {
            printk("Error (%d): WiFi connection failed. Retrying in 5s...\r\n", ret);
            k_sleep(K_MSEC(5000)); // Wait before retrying
        }
    } while (ret < 0);

    // Wait to receive an IP address (blocking)
    wifi_wait_for_ip_addr();

    // Wifi initialization - Test code End
    #else
    #error "WIFI_SSID is not defined. Please define WIFI_SSID and WIFI"
    #endif // WIFI_SSID
    #endif

    return 0;
}

SYS_INIT_NAMED(board_init, &board_init, APPLICATION, 0);