use zephyr::timer::{Callback, StoppedTimer};
use zephyr::time::{Duration, NoWait};
use zephyr::device::gpio::GpioPin;
use zephyr::sync::Arc;
use zephyr::sync::Mutex;

use log::info;

use crate::application::blink::Blink; 

pub struct GpioBlink {
    inner: Arc<BlinkInner>,
}

pub struct BlinkInner {
    led: Mutex<GpioPin>,
    timer: Mutex<Option<StoppedTimer>>,
}

impl GpioBlink {
    fn on_timer_expired(inner: &Arc<BlinkInner>) {
        let mut led = inner.led.lock().unwrap();
        led.toggle_pin();
        info!("LED Pin Toggles...");
    }

    pub fn new(led: GpioPin) -> Self {
        let pin = Mutex::new(led);
        let tm = Mutex::new(Some(StoppedTimer::new()));
        Self{inner: Arc::new(BlinkInner{ led:pin, timer:tm }) }
    }
}

impl Blink for GpioBlink {
    fn set_period_ms(&mut self, period_ms: u32) -> Result<(), i32> {
        let cb = Callback {
            call: Self::on_timer_expired, 
            data: Arc::clone(&self.inner),
        };
        let delay = NoWait;
        let d = Duration::from_ticks(period_ms.into());

        // Lock the mutex and take ownership of the timer out of the Option
        let mut guard = self.inner.timer.lock().unwrap();

        if let Some(timer) = guard.take() {
            let _started_timer = timer.start_callback(cb, delay, d);
        }

        return Ok(());
    }

    fn set_blink_off(&mut self) -> Result<(), i32> {
        return Err(-1);
    }
}

// Thread-safety guarantees for the Zephyr timer
unsafe impl Send for GpioBlink {}
unsafe impl Sync for GpioBlink {}
unsafe impl Sync for BlinkInner {}
unsafe impl Send for BlinkInner {}
