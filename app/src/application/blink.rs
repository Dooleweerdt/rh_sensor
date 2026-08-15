pub trait Blink {
    fn set_period_ms(&mut self, period_ms: u32) -> Result<(), i32>;    
    fn set_blink_off(&mut self) -> Result<(), i32>;
}

