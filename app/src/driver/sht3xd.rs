use core::ffi::CStr;
use zephyr_sys::{device, device_get_binding, device_is_ready}; //, sensor_sample_fetch, sensor_channel_get, sensor_value};

use log::info;

pub fn read_sensor_example() {
    unsafe {
        // 1. Get the device pointer (Using the label from your DTS/Overlay)
        // Ensure your sensor has a "label" property or use the node name if supported
        let sensor_label = CStr::from_bytes_with_nul(b"sht3xd@45\0").unwrap();  //<__device_dts_ord_94>
        let dev: *const device = device_get_binding(sensor_label.as_ptr());

        if dev.is_null() {
            info!("sensor_label: {:?}", sensor_label);
            info!("dev pointer: {:?}", dev);
            info!("Error: Device not found!");
            return;
        }

        if device_is_ready(dev) {
            info!("SHT3Xd ready!");
        }
        else {
            info!("SHT3Xd is NOT ready");
        }
    }
}