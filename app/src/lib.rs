// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
// Sigh. The check config system requires that the compiler be told what possible config values
// there might be.  This is completely impossible with both Kconfig and the DT configs, since the
// whole point is that we likely need to check for configs that aren't otherwise present in the
// build.  So, this is just always necessary.
#![allow(unexpected_cfgs)]

mod rht_sensor;
mod rht_sensor_init;

use log::info;
use zephyr::raw::ZR_GPIO_OUTPUT_ACTIVE;
use zephyr::time::{sleep, Duration};

use crate::rht_sensor::{RhtSensor, sht3x::Sht3x, onewire::OneWire};

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
        loop {}
    }

    led0.configure(ZR_GPIO_OUTPUT_ACTIVE);
    let duration = Duration::millis_at_least(1000);

    let mut s1 = Sht3x { dev: core::ptr::null() };
    let mut s2 = OneWire { dev: core::ptr::null() };

    // A "List" of different sensors (Trait Objects)
    // This is the equivalent of a List<IRhtSensor> in C#
    let sensors: &mut [&mut dyn RhtSensor] = &mut [&mut s1, &mut s2];    

    for s in &mut *sensors {
        s.init().unwrap();
    }

    loop {
        led0.toggle_pin();

        for s in &mut *sensors {
            let (temp, hum) = s.read_data().unwrap();
            //info!("s: {}:  Temperature: {} C, Humidity: {} %%", s, temp, hum);
            info!(" - - - - ");
            info!("Temperature: {} C", temp);
            info!("Humidity: {} %%", hum);
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
