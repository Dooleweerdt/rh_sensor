use crate::comm::DataTransport;
use crate::comm::CommError;
use crate::application::SensorMsg;
use serde::Serialize;
use log::info;
use core::ffi::c_void;


#[derive(Serialize)]
struct MqttPayload {
    source: u32,
    temperature: f32,
    humidity: f32,
}

unsafe extern "C" {
    fn wifi_init();
    fn wifi_connect(ssid: *const c_void, psk: *const c_void) -> i32;
    fn wifi_wait_for_ip_addr();
    fn wifi_bridge_get_ssid() -> *const core::ffi::c_void;
    fn wifi_bridge_get_psk() -> *const core::ffi::c_void;
}

pub struct MqttTransport {
    broker_address: &'static str,
}

impl MqttTransport {
    pub fn new(broker_address: &'static str) -> Self {
        Self { broker_address }
    }
}

#[cfg(CONFIG_WIFI)]
impl DataTransport for MqttTransport {
    fn connect(&mut self) -> Result<(), CommError> {
        unsafe {
            // @TODO:Add flag for only initializing once and in case of errors where re-connect is
            // required! 
            wifi_init();
            let ssid = wifi_bridge_get_ssid();
            let psk = wifi_bridge_get_psk();
            let status = wifi_connect(ssid, psk);
            if status > 0 {
                info!("Wifi connection error");
                return Err(CommError::ConnectionFailed);
            }
            wifi_wait_for_ip_addr();
        }
        Ok(()) // Assume success for this example
    }

    fn send_data(&mut self, msg: &SensorMsg) -> Result<(), CommError> {
        let wire_payload = MqttPayload {
            source: msg.source,
            humidity: msg.hum,
            temperature: msg.temp,
        };

        let mut buf = [0u8; 128];
        let bytes_written = serde_json_core::to_slice(&wire_payload, &mut buf).map_err(|_| CommError::SerializationFailed)?;

        if let Ok(json_str) = core::str::from_utf8(&buf[..bytes_written]) {
            info!("Successfully generated JSON payload: {}", json_str);
        }

        // info!("Serialized MQTT payload: {}", core::str::from_utf8(&buf[..serialized]).unwrap());
        info!("Pretending to send MQTT payload: source={}, temp={}C, hum={}%", wire_payload.source, wire_payload.temperature, wire_payload.humidity);

        // TBD...
        // // 3. Fire the bytes down the Zephyr POSIX/MQTT network stack via FFI
        // crate::ffi::zephyr_mqtt_publish(buf, serialized)
        //     .map_err(|_| "Network stack reject outbound message")
        Ok(())
    }
}

