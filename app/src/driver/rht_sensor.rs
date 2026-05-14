use zephyr_sys::{sensor_sample_fetch, sensor_channel_get, sensor_value,
                 sensor_channel_SENSOR_CHAN_AMBIENT_TEMP, sensor_channel_SENSOR_CHAN_HUMIDITY};

use log::info;

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
/// Creates a new sensor instance with specific capabilities
    pub fn new(
        dev: *const zephyr_sys::device, 
        name: &'static str, 
        capabilities: &'static [SensorChannel]
    ) -> Option<Self> {
        // Validation: If Zephyr couldn't find the device, don't create the object
        if dev.is_null() {
            return None;
        }

        Some(Self {dev, name, capabilities})
    }

    pub fn read_data(&mut self, channel: SensorChannel) -> Result<f32, i32> {
        let mut val = sensor_value { val1: 0, val2: 0 };

        if !self.capabilities.contains(&channel) {
            return Err(-1); // Or a specific "Unsupported" error
        }

        let zephyr_chan = match channel {
            SensorChannel::Temperature => sensor_channel_SENSOR_CHAN_AMBIENT_TEMP,
            SensorChannel::Humidity => sensor_channel_SENSOR_CHAN_HUMIDITY,
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
        // info!(" - - - - ");
        // info!("Sensor: {}", self.name);
        // info!("Sensor reading: {}.{} %%", val.val1, val.val2);
        Ok(val.val1 as f32 + (val.val2 as f32 / 1_000_000.0))
    }
}