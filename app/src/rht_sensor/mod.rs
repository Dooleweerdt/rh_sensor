// Declare the sub-modules (the actual implementations)
pub mod sht3x;
pub mod onewire;

// The "Interface" definition
pub trait RhtSensor {
    fn init(&mut self) -> Result<(), i32>;
    fn read_data(&mut self) -> Result<(f32, f32), i32>;
}