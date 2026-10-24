use zephyr::timer::{Callback, StoppedTimer, CallbackTimer};
use zephyr::time::{Duration, NoWait};
use zephyr::device::gpio::GpioPin;
use zephyr::sync::Arc;
use zephyr::sync::SpinMutex;

use core::pin::Pin;

#[cfg(CONFIG_RUST_ALLOC)]
use alloc::boxed::Box;

use crate::application::blink::Blink;

type InnerTimer = Option<Pin<Box<CallbackTimer<Arc<BlinkInner>>>>>;

pub struct GpioBlink {
    inner: Arc<BlinkInner>,
}

pub struct BlinkInner {
    led: SpinMutex<GpioPin>,
    timer: SpinMutex<InnerTimer>,
}

impl GpioBlink {
    fn on_timer_expired(inner: &Arc<BlinkInner>) {
        let mut led = inner.led.lock().unwrap();
        led.toggle_pin();
    }

    pub fn new(led: GpioPin) -> Self {
        let pin = SpinMutex::new(led);
        let tm = SpinMutex::new(None);
        Self{inner: Arc::new(BlinkInner{ led:pin, timer:tm }) }
    }
}

impl Blink for GpioBlink {
    fn set_period_ms(&mut self, period_ms: u32) -> Result<(), i32> {
        // Lock the mutex and take ownership of the timer out of the Option
        let mut guard = self.inner.timer.lock().unwrap();

        /*if let Some(running_timer) = guard.take() {
            running_timer.as_mut().stop();
        }*/
        let tm_stopped: StoppedTimer = if let Some(running_timer) = guard.take() {
            // SAFETY: We are immediately passing the unpinned `CallbackTimer` to `.stop()`, 
            // which deregisters it from Zephyr's C kernel before it can ever move in RAM.
            let boxed_timer = unsafe { Pin::into_inner_unchecked(running_timer) };
            (*boxed_timer).stop()
            //running_timer.as_mut().stop()
        } else {
            // No timer was running - return a new one
            StoppedTimer::new()
        };

        let cb = Callback {
            call: Self::on_timer_expired, 
            data: Arc::clone(&self.inner),
        };
            
        let delay = NoWait;
        let d = Duration::from_ticks(period_ms.into());

        // Make sure to store the return value, so it doesn't go out of scope...
        *guard = Some(tm_stopped.start_callback(cb, delay, d)); 

        Ok(())
    }

    //fn set_blink_off(&mut self) -> Result<(), i32> {
    //    Err(-1)
    //}
}

// Thread-safety guarantees for the Zephyr timer
// SAFETY: Required... TBD...
unsafe impl Sync for BlinkInner {}
// SAFETY: Required... TBD...
unsafe impl Send for BlinkInner {}
