// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
// Sigh. The check config system requires that the compiler be told what possible config values
// there might be.  This is completely impossible with both Kconfig and the DT configs, since the
// whole point is that we likely need to check for configs that aren't otherwise present in the
// build.  So, this is just always necessary.
#![allow(unexpected_cfgs)]

mod driver;
mod rht_sensor_init;

use log::info;
use zephyr::raw::ZR_GPIO_OUTPUT_ACTIVE;
use zephyr::time::{sleep, Duration};

use driver::rht_sensor::RhtSensor;
use driver::rht_sensor::SensorChannel;

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
    let duration = Duration::millis_at_least(1000);

    let sht3x = RhtSensor { dev: core::ptr::null(), name: "SHT3X", capabilities: &[SensorChannel::Temperature, SensorChannel::Humidity] };
    let ds18b20 = RhtSensor { dev: core::ptr::null(), name: "DS18B20", capabilities: &[SensorChannel::Temperature] };

    // A "List" of different sensors (Trait Objects)
    // This is the equivalent of a List<IRhtSensor> in C#
    let mut sensors = [sht3x, ds18b20];

    for s in &mut sensors {
        s.init();
    }

    loop {
        led0.toggle_pin();

        for s in &mut sensors {
            if let Ok(t) = s.read_data(SensorChannel::Temperature) {
                info!("{} Temperature: {} C", s.name, t);
            }
            if let Ok(h) = s.read_data(SensorChannel::Humidity) {
                info!("{} Humidity: {} %%", s.name, h);
            }
        }

        sleep(duration);
    }
}

#[cfg(not(dt = "aliases::led0"))]
fn do_blink() {
    let duration = Duration::millis_at_least(5000);
    loop {
        sleep(duration);
        info!("No leds configured");
    }
}
