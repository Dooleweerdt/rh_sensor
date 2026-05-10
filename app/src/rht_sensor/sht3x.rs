use crate::rht_sensor::RhtSensor;
use zephyr_sys::device;

use log::info;
use crate::rht_sensor_init::check_sensor_ready;


pub struct Sht3x {
    pub dev: *const device,
}

impl RhtSensor for Sht3x {
    fn init(&mut self) -> Result<(), i32> {
        let dev = check_sensor_ready();

        if dev.is_null() {
            info!("Error: Device not found!");
            return Err(-1);
        }
        self.dev = dev;
        Ok(())
    }
    fn read_data(&mut self) -> Result<(f32, f32), i32> {
        Ok((25.0, 50.0))
    }
}
