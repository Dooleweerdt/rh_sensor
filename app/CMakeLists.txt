#-------------------------------------------------------------------------------
# Zephyr Example Application
#
# Copyright (c) 2021 Nordic Semiconductor ASA
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.13.1)
find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

list(APPEND ZEPHYR_DTS_PATH ${CMAKE_CURRENT_LIST_DIR/dts})

project(app)
rust_cargo_application()

# Add c-project sources
project(app LANGUAGES C)
#FILE(GLOB app_sources c_src/*.c)
target_sources(app PRIVATE c_src/main.c)
target_sources(app PRIVATE c_src/board_init.c)
target_sources(app PRIVATE c_src/rht_sensor_init.c)
target_sources(app PRIVATE c_src/zbus_bridge.c)
target_sources(app PRIVATE c_src/smp_ble.c)

target_sources_ifdef(CONFIG_WIFI app PRIVATE c_src/wifi_bridge.c)
target_sources_ifdef(CONFIG_WIFI app PRIVATE c_src/mqtt_bridge.c)

# Checks if 'MQTT_USER' exists in 'wifi_settings.h'
include(CheckSymbolExists)
set(CMAKE_REQUIRED_INCLUDES "${CMAKE_CURRENT_SOURCE_DIR}/../include/app")

check_symbol_exists(MQTT_USER "wifi_settings.h" HAS_MQTT_CREDS)

if(NOT HAS_MQTT_CREDS)
    target_compile_definitions(app PRIVATE MQTT_USER=\"USER\")
    target_compile_definitions(app PRIVATE MQTT_PASSWORD=\"DUMMY\")
endif()
# End MQTT_USER check

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

