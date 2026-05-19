use crate::application::SensorMsg;
use core::ffi::c_void;
use log::info;

unsafe extern "C" {
    fn zbus_bridge_wait_read(sub: *const c_void, chan: *mut *const c_void, msg: *mut SensorMsg) -> i32;
    static wifi_sub: core::ffi::c_void;
}

/// This fetches the ZBUS_SUBSCRIBER_DEFINE(wifi_sub) in bridge.c
pub fn get_sub_ptr() -> *const core::ffi::c_void {
    unsafe { &wifi_sub as *const _ }
}

#[zephyr::thread(stack_size = 8192)]
pub fn comm_thread() {
    let mut msg = SensorMsg { source: 0, temp: 0.0, hum: 0.0 };

    log::info!("Communication thread started (Wi-Fi/BT ready)");
    
    // We need a pointer to hold which channel woke us up 
    // (useful if one listener observes multiple channels)
    let mut chan_ptr: *const c_void = core::ptr::null();

    loop {
        // This blocks the thread. No semaphore needed!
        // It specifically waits for the 'wifi_lis' subscriber defined in C.
        let ret = unsafe { zbus_bridge_wait_read(get_sub_ptr(), &mut chan_ptr, &mut msg) };

        if ret == 0 {
            info!("Zbus woke me up! Temp: {}, Hum: {}", msg.temp, msg.hum);
        }
    }
}