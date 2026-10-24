pub mod blink;
pub mod button;

use log::info;
use zephyr::time::{sleep, Duration};
use alloc::vec::Vec;

use crate::driver::rht_sensor::RhtSensor;
use crate::driver::rht_sensor::SensorChannel;
use crate::driver::button_usr::UserButton;
use crate::application::button::Button;

// Use Rust pointer syntax - not C style!
unsafe extern "C" {
    fn zbus_bridge_publish_rht(msg: *const SensorMsg) -> i32;
    fn init_smp_ble() -> i32;
}

pub const ID_SHT3X: u32 = 0;
pub const ID_ONEWIRE: u32 = 1;
pub const ID_MOCK: u32 = 2;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct SensorMsg {
    pub source: u32,
    pub temp: f32,
    pub hum: f32,
}

pub fn broadcast_data(msg: *const SensorMsg) {
    // SAFETY:
    // Calls unsafe c-functions in zbus_bridge.c
    unsafe {
        zbus_bridge_publish_rht(msg);
    }
}

pub fn start (mut sensors: Vec<RhtSensor>, mut button: UserButton) {
    let duration = Duration::millis_at_least(30000);

    loop {
        if button.is_pressed() {
            info!("Button Pressed!");
            #[cfg(CONFIG_BT)]
	        // SAFETY:
            // Calls unsafe c-functions in smp_ble.c
            unsafe {
                init_smp_ble();
            }
            loop {
                sleep(duration);
            }
            // Now in FWU state - BLE IF activated!
        }

        for s in &mut sensors {
            let id = match s.name {"SHT3x" => ID_SHT3X, "DS18B20" => ID_ONEWIRE, _ => ID_MOCK};
            let mut sensor_data = SensorMsg { source: id, temp: 0.0, hum: 0.0 };

            if let Ok(t) = s.read_data(SensorChannel::Temperature) {
                info!("{} Temperature: {} C", s.name, t);
                sensor_data.temp = t;
            }
            if let Ok(h) = s.read_data(SensorChannel::Humidity) {
                info!("{} Humidity: {} %%", s.name, h);
                sensor_data.hum = h;
            }

            // Send data on zbus to subscribers (wifi, display, etc.)
            broadcast_data(&sensor_data);
        }

        sleep(duration);
    }
}
