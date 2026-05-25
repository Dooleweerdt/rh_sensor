// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
// Sigh. The check config system requires that the compiler be told what possible config values
// there might be.  This is completely impossible with both Kconfig and the DT configs, since the
// whole point is that we likely need to check for configs that aren't otherwise present in the
// build.  So, this is just always necessary.
#![allow(unexpected_cfgs)]

mod driver;
mod application;
mod comm;
extern crate alloc;

use log::info;
use zephyr::raw::ZR_GPIO_OUTPUT_ACTIVE;
use zephyr_sys::device;
use alloc::vec::Vec;

use driver::rht_sensor::RhtSensor;
use driver::rht_sensor::SensorChannel;
use comm::comm_thread;

#[cfg(not(dt = "aliases::led0"))]
use zephyr::time::{sleep, Duration};

// Use Rust pointer syntax - not C style!
unsafe extern "C" {
    fn get_sht3x_device() -> *const device;
    fn get_ds18b20_device() -> *const device;
    fn get_demo_sensor_device() -> *const device;
}


#[unsafe(no_mangle)]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    info!("Starting blinky");

    do_blink();
}

#[cfg(dt = "aliases::led0")]
fn do_blink() {
    info!("Inside of blinky");

    let mut led0 = zephyr::devicetree::aliases::led0::get_instance().unwrap();

    if !led0.is_ready() {
        info!("LED is not ready");
        loop {};
    }

    led0.configure(ZR_GPIO_OUTPUT_ACTIVE);

    // Create a "List" of different sensors (Trait Objects)
    let mut sensors = Vec::new();

    unsafe {
        if let Some(sht3x) = RhtSensor::new(get_sht3x_device(), "SHT3X", &[SensorChannel::Temperature, SensorChannel::Humidity]) {
            sensors.push(sht3x);
        }
        if let Some(ds18b20) = RhtSensor::new(get_ds18b20_device(), "DS18B20", &[SensorChannel::Temperature]) {
            sensors.push(ds18b20);
        }

        if sensors.is_empty() {
            if let Some(demo_sensor) = RhtSensor::new(get_demo_sensor_device(), "Mock Sensor", &[SensorChannel::Temperature, SensorChannel::Humidity]) {
                sensors.push(demo_sensor);
            }
        }
    }

    let _comm_thread = comm_thread();
    _comm_thread.set_priority(10);
    _comm_thread.start();
    application::start(led0, sensors);
}

#[cfg(not(dt = "aliases::led0"))]
fn do_blink() {
    let duration = Duration::millis_at_least(5000);
    loop {
        sleep(duration);
        info!("No leds configured");
    }
}
