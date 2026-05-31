use crate::application::SensorMsg;
use zephyr::thread::{ThreadData, ThreadStack, ReadyThread};
use core::ffi::{c_void, CStr};
use core::ptr::addr_of_mut;
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
        let mut msg = SensorMsg { source: 0, temp: 0.0, hum: 0.0 };

        log::info!("Communication thread started (Wi-Fi/BT ready)");
        
        // Create a pointer to hold which channel woke us up 
        let mut chan_ptr: *const c_void = core::ptr::null();

        loop {
            // This blocks the thread. No semaphore needed!
            // - It specifically waits for the 'wifi_lis' subscriber defined in C.
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

// Allocate the Zephyr thread's stack and a single thread pool structure. 
const STACK_SIZE: usize = 4096;
static COMM_THREAD_STACK: ThreadStack<STACK_SIZE> = ThreadStack::new();
static COMM_THREAD_POOL: [ThreadData<usize>; 1] = [ThreadData::new()];

static mut COMM_INTERFACE_STORAGE: Option<CommInterface<dyn DataTransport>> = None;

extern "C" fn comm_runner(p1: *mut c_void, _p2: *mut c_void, _p3: *mut c_void) {
    // p1 points directly to the UnsafeCell container managed by ThreadData.
    // Zephyr-thread.rs module passes the init data pointer as the first argument to k_thread_create.
    unsafe {
        let init_data_ptr = p1 as *mut Option<usize>;
        if let Some(interface_address) = *init_data_ptr {
            let comm_if = &mut *(interface_address as *mut CommInterface<dyn DataTransport>);
            comm_if.run();
        } else {
            panic!("Comm thread woke up with uninitialized interface data!");
        }
    }
}

pub fn spawn_comm_thread<T: DataTransport>(transport: &'static mut T, priority: i32) 
where 
    T: DataTransport + 'static,
{
    let thread_name = CStr::from_bytes_with_nul(b"comm_runner\0").unwrap();

    unsafe {
        let comm_if: &'static mut dyn DataTransport = transport;
        COMM_INTERFACE_STORAGE = Some(CommInterface::new(comm_if));
        
        // Get a raw pointer to the Option-wrapped CommInterface in static storage
        let storage_raw_ptr: *mut Option<CommInterface<dyn DataTransport>> = addr_of_mut!(COMM_INTERFACE_STORAGE);
        
        // Extract the raw pointer to the concrete inner CommInterface struct
        let interface_raw_ptr: *mut CommInterface<dyn DataTransport> = (*storage_raw_ptr).as_mut().unwrap();
        
        // Convert the raw pointer straight into a plain integer address (usize)
        let comm_if_addr = interface_raw_ptr as usize;

        // Invoke the initialization parameters from the Zephyr thread.rs module
        // - Pass the raw comm_runner pointer (comm_if_addr)
        let ready_thread: ReadyThread = ThreadData::acquire(
            &COMM_THREAD_POOL,
            core::slice::from_ref(&COMM_THREAD_STACK),
            comm_if_addr     , // Pass our interface pointer down as the Send payload argument (T)
            Some(comm_runner),
            priority,
            thread_name,
        );
        
        ready_thread.start();
    }
}