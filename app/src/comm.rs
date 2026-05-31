use crate::application::SensorMsg;
use zephyr::thread;
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

#[derive(Debug, Clone, Copy)]
pub enum CommError {
    ConnectionFailed,
    SendFailed,
}

pub trait DataTransport {
    // Connects or performs handshakes (MQTT login or BLE advertising start)
    fn connect(&mut self) -> Result<(), CommError>;

    // Sends the structured sensor payload down the channel
    fn send_data(&mut self, msg: &SensorMsg) -> Result<(), CommError>;
}

pub struct CommInterface<T: DataTransport + 'static + ?Sized> {
    transport: &'static mut T,
}

impl <T: DataTransport + 'static + ?Sized> CommInterface<T> 
{
    pub fn new(transport: &'static mut T) -> Self {
        Self { transport }
    }

    pub fn run(&mut self) {
        // let transport = unsafe {
        //     match &mut TRANSPORT {
        //         Some(t) => t,
        //         None => {
        //             // If init() was not called before the scheduler woke the thread, freeze
        //             panic!("Communication background thread initialized without a mapped data route!");
        //         }
        //     }
        // };    
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

                if let Err(e) = self.transport.connect() {
                    log::error!("Failed to connect transport: {:?}", e);
                    continue; // Skip sending if we can't connect
                }

                if let Err(e) = self.transport.send_data(&msg) {
                    log::error!("Failed to send data: {:?}", e);
                    continue; // Handle send failure (e.g., retry logic could go here)
                }
            }
        }
    }
}

// Allocate a static workspace buffer block for the thread's stack. 
// Zephyr requires this memory to be statically reserved.
static mut COMM_THREAD_STACK: [u8; 4096] = [0; 4096];
static mut COMM_THREAD_DATA: Option<CommInterface<dyn DataTransport>> = None;

extern "C" fn comm_runner(p1: *mut c_void, _p2: *mut c_void, _p3: *mut c_void) {
    // Cast the user data pointer back into our concrete controller type
    let comm_if = unsafe { &mut *(p1 as *mut CommInterface<dyn DataTransport>) };
    comm_if.run();
}

pub fn spawn_comm_thread<T: DataTransport>(transport: &'static mut T, priority: i32) 
where 
    T: DataTransport + 'static,
{
    let mut comm_if = CommInterface::new(transport);

    unsafe {
        COMM_THREAD_DATA = Some(core::mem::transmute(comm_if));
        
        let comm_if_ptr = COMM_THREAD_DATA.as_mut().unwrap() as *mut _ as *mut c_void;

        // 2. Invoke the initialization parameters from your local thread.rs module
        // We pass the concrete trampoline pointer and the stack storage boundaries
        thread::create(
            &mut COMM_THREAD_STACK,
            comm_runner,
            comm_if_ptr,           // p1
            core::ptr::null_mut(), // p2
            core::ptr::null_mut(), // p3
            priority,
            0, // options
            0, // delay
        );
    }
}