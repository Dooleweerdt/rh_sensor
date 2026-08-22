use crate::application::blink::Blink;
use crate::driver::gpio_blink::GpioBlink;
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

unsafe extern "C" {
    fn mqtt_bridge_connect() -> i32;
    fn mqtt_bridge_send_data(data: *const u8, len: usize) -> i32;
}

pub struct MqttTransport {
    broker_address: &'static str,
    connected_flag: bool,
    led: GpioBlink,
}

impl MqttTransport {
    pub fn new(broker_address: &'static str, led: GpioBlink) -> Self {
        Self { 
            broker_address, 
            connected_flag : false, 
            led: led,
        }
    }
}

#[cfg(CONFIG_WIFI)]
impl DataTransport for MqttTransport {
    fn connect(&mut self) -> Result<(), CommError> {
        unsafe {
            // @TODO:Add flag for only initializing once and in case of errors where re-connect is
            // required! 
            if self.connected_flag == false {
                wifi_init();
                let ssid = wifi_bridge_get_ssid();
                let psk = wifi_bridge_get_psk();
                let status = wifi_connect(ssid, psk);
                if status < 0 {
                    info!("Wifi connection error");
                    return Err(CommError::ConnectionFailed);
                }
                
                wifi_wait_for_ip_addr();
            
                let mqtt_status = mqtt_bridge_connect();
                if mqtt_status == 0 {
                    info!("Connected to MQTT Broker... Addr = {}, Status = {}", self.broker_address, mqtt_status);
                    let _ = self.led.set_period_ms(1000);
                }
                self.connected_flag = true;
            }
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
        info!("Sending MQTT payload: source={}, temp={}C, hum={}%", wire_payload.source, wire_payload.temperature, wire_payload.humidity);

        unsafe {
            if mqtt_bridge_send_data(buf.as_ptr(),bytes_written) != 0 {
                return Err(CommError::SendFailed);
            
            }
        }
        Ok(())
    }
}

