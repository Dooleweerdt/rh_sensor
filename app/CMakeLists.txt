#-------------------------------------------------------------------------------
# Zephyr Example Application
#
# Copyright (c) 2021 Nordic Semiconductor ASA
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.13.1)
find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(app)
rust_cargo_application()

# Add c-project sources
project(app LANGUAGES C)
#FILE(GLOB app_sources c_src/*.c)
target_sources(app PRIVATE c_src/main.c)
target_sources(app PRIVATE c_src/board_init.c)
target_sources(app PRIVATE c_src/rht_sensor_init.c)
target_sources(app PRIVATE c_src/zbus_bridge.c)

target_sources_ifdef(CONFIG_WIFI app PRIVATE c_src/wifi_bridge.c)
target_sources_ifdef(CONFIG_WIFI app PRIVATE c_src/mqtt_bridge.c)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

