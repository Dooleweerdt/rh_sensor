use zephyr::device::gpio::GpioPin;

use crate::application::button::Button;

pub struct UserButton {
    btn: GpioPin,
}

impl UserButton {
    pub fn new (pin: GpioPin) -> Self {
        Self{btn: pin}
    }
}

impl Button for UserButton {
    fn is_pressed(&mut self) -> bool {
       !self.btn.get()  
    }
}
