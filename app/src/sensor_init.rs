use log::info;
use zephyr_sys::device;

// Use Rust pointer syntax - not C style!
unsafe extern "C" {
    pub fn is_sensor_ready() -> *const device;
}

pub fn check_sensor_ready() -> *const device {
    unsafe {
        let dev = crate::sensor_init::is_sensor_ready();
        if dev.is_null() {
            info!("Sensor is NOT ready for Rust");
            return core::ptr::null();
        }
        else {
            info!("Sensor is ready for Rust!");
            return dev;
        }
    }
}