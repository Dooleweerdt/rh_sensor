use zephyr_sys::k_timer;
use zephyr::device::gpio::GpioPin;

use crate::application::blink::Blink; 

pub struct GpioBlink {
    led: GpioPin,
}

impl GpioBlink {
    pub fn new(led: GpioPin) -> Self {
        Self{led}
    }
}

impl Blink for GpioBlink {
    fn set_period_ms(&mut self, period_ms: u32) -> Result<i32, i32> {
        return Err(-1);
    }

    fn set_blink_off(&mut self) -> Result<i32, i32> {
        return Err(-1);
    }
}

