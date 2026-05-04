/*
 * Copyright (c) 2021 Nordic Semiconductor ASA
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/init.h>
#include <zephyr/logging/log.h>
#include <zephyr/drivers/display.h>
#include <zephyr/display/cfb.h>

#include <app_version.h>

// Custom libraries
#include <app/drivers/wifi.h>
#include <app/wifi_settings.h>

LOG_MODULE_REGISTER(board_init, CONFIG_APP_LOG_LEVEL);

#ifndef WIFI_CREDENTIALS
#define WIFI_SSID "WirelessNetwork"
#define WIFI_PSK "Password"
#endif

#if CONFIG_DISPLAY
#define DISPLAY_DRIVER      DT_CHOSEN(zephyr_display)
const struct device *display_dev = DEVICE_DT_GET(DISPLAY_DRIVER);

#if CONFIG_LVGL
#include <lvgl.h>
//LV_IMG_DECLARE(gemini-icon);
//LV_IMG_DECLARE(zephyr-icon);
#endif // CONFIG_LVGL

#endif // CONFIG_DISPLAY

int board_init(void)
{
    printk("Zephyr - RH Sensor in Rust %s\n", STRINGIFY(APP_BUILD_VERSION));

    #if CONFIG_DISPLAY
    if (!device_is_ready(display_dev)) {
        // Handle error
        return ENODEV;
    }

    // Now you can use display_blanking_off(display_dev) 
    // and start writing pixels or text.
    int ret = display_blanking_off(display_dev);
	if (ret < 0 && ret != -ENOSYS) {
		LOG_ERR("Failed to turn blanking off (error %d)", ret);
    }

    #if CONFIG_LVGL
    lv_obj_t * label1 = lv_label_create(lv_scr_act());
    lv_label_set_text(label1, "Zephyr LVGL!");
    lv_obj_align(label1, LV_ALIGN_TOP_LEFT, 0, 0);
    lv_timer_handler();
    #else
    // Initialize the CFB (Character Frame Buffer)
    if (cfb_framebuffer_init(display_dev)) {
        return EIO; // Error initializing framebuffer
    }

    // Clear the screen and set font
    cfb_framebuffer_clear(display_dev, true);

    // Print text at coordinates (x, y)
    cfb_print(display_dev, "Zephyr RTOS!", 0, 0);
    cfb_print(display_dev, "Feather nRF52840", 0, 16);

    // Finalize and push the buffer to the hardware
    cfb_framebuffer_finalize(display_dev);
    #endif // CONFIG_LVGL
    #endif // CONFIG_DISPLAY

    #if CONFIG_WIFI
    #if WIFI_SSID
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

SYS_INIT_NAMED(board_init, &board_init, APPLICATION, 256);