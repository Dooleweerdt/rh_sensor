use zephyr_sys::{sensor_sample_fetch, sensor_channel_get, sensor_value,
                 sensor_channel_SENSOR_CHAN_AMBIENT_TEMP, sensor_channel_SENSOR_CHAN_HUMIDITY};

use log::info;
use crate::rht_sensor_init::check_sensor_ready;

// Defines the physical quantities the sensors can measure
#[repr(u32)] // Ensures this enum is stored as a 32-bit unsigned integer
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SensorChannel {
    Temperature = 0,
    Humidity = 1,
    // Add more later: Pressure, CO2, etc.
}

/// The common Sensor struct
pub (crate) struct RhtSensor {
    pub dev: *const zephyr_sys::device,
    pub name: &'static str,
    pub capabilities: &'static [SensorChannel],
}

impl RhtSensor {
    pub fn init(&self) -> bool {
        unsafe {
            self.dev = check_sensor_ready();
        }

        if self.dev.is_null() {
            info!("Error: Device not found!");
            return false;
        }
        return true;
    }

    pub fn read_data(&mut self, channel: SensorChannel) -> Result<f32, i32> {
        let mut val = core::mem::zeroed::<sensor_value>();

        if !self.capabilities.contains(&channel) {
            return Err(-1); // Or a specific "Unsupported" error
        }

        let zephyr_chan = match channel {
            SensorChannel::Temperature => zephyr_sys::sensor_channel_SENSOR_CHAN_AMBIENT_TEMP,
            SensorChannel::Humidity => zephyr_sys::sensor_channel_SENSOR_CHAN_HUMIDITY,
        };

        unsafe {
            let ret = sensor_sample_fetch(self.dev);
            if ret != 0 {
                info!("Error fetching sensor sample: {}", ret);
                return Err(ret);
            }

            let ret = sensor_channel_get(self.dev, zephyr_chan, &mut val);
            if ret != 0 {
                info!("Error getting sensor data: {}", ret);
                return Err(ret);
            }

        }
        info!(" - - - - ");
        info!("Sensor: {}", self.name);
        info!("Sensor reading: {}.{} %%", val.val1, val.val2);
        Ok(val.val1 as f32 + val.val2 as f32 / 100.0)
    }
}