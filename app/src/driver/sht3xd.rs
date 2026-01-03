use core::ffi::CStr;
use zephyr_sys::{device, device_get_binding, device_is_ready, 
                sensor_sample_fetch, sensor_channel_get, sensor_value,
                sensor_channel_SENSOR_CHAN_AMBIENT_TEMP, sensor_channel_SENSOR_CHAN_HUMIDITY};

use log::info;

pub fn read_sensor_example() {
    unsafe {
        let sensor_label = CStr::from_bytes_with_nul(b"sht3xd@45\0").unwrap();
        let dev: *const device = device_get_binding(sensor_label.as_ptr());

        if dev.is_null() {
            info!("Error: Device not found!");
            return;
        }

        if device_is_ready(dev) {
            info!("SHT3Xd ready!");
        }
        else {
            info!("SHT3Xd is NOT ready");
        }

        loop {
            let ret = sensor_sample_fetch(dev);
            if ret != 0 {
                info!("Error fetching sensor sample: {}", ret);
                return;
            }

            let mut temperature = sensor_value { val1: 0, val2: 0 };
            let mut humidity = sensor_value { val1: 0, val2: 0 };
            let ret = sensor_channel_get(dev, sensor_channel_SENSOR_CHAN_AMBIENT_TEMP, &mut temperature);
            if ret != 0 {
                info!("Error getting temperature: {}", ret);
                return;
            }

            let ret = sensor_channel_get(dev, sensor_channel_SENSOR_CHAN_HUMIDITY as u32, &mut humidity);
            if ret != 0 {
                info!("Error getting humidity: {}", ret);
                return;
            }
            info!(" - - - - ");
            info!("Temperature: {}.{} C", temperature.val1, temperature.val2);
            info!("Humidity: {}.{} %%", humidity.val1, humidity.val2);

            zephyr::time::sleep(zephyr::time::Duration::secs_at_least(5));
        }
    }
}