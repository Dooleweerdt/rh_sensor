use crate::comm::DataTransport;
use crate::comm::CommError;
use crate::application::SensorMsg;
//use serde::Serialize;
use log::info;

// #[derive(Serialize)]
struct MqttPayload {
    src: u32,
    t: f32,
    h: f32,
}

pub struct MqttTransport {
    broker_address: &'static str,
}

impl MqttTransport {
    pub fn new(broker_address: &'static str) -> Self {
        Self { broker_address }
    }
}

impl DataTransport for MqttTransport {
    fn connect(&mut self) -> Result<(), CommError> {
        // TBD...
        // // Invoke your Zephyr C FFI function to run the MQTT handshake
        // crate::ffi::zephyr_mqtt_connect(self.broker_address)
        //     .map_err(|_| "Failed to connect to Mosquitto broker")

        Ok(()) // Assume success for this example
    }

    fn send_data(&mut self, msg: &SensorMsg) -> Result<(), CommError> {
        let wire_payload = MqttPayload {
            src: msg.source,
            t: msg.temp,
            h: msg.hum,
        };

        //let mut buf = [0u8; 128];
        // let serialized = serde_json_core::to_slice(&wire_payload, &mut buf)
        //     .map_err(|_| "JSON serialization overflowed buffer")?;

        // info!("Serialized MQTT payload: {}", core::str::from_utf8(&buf[..serialized]).unwrap());
        info!("Pretending to send MQTT payload: src={}, temp={}C, hum={}%", wire_payload.src, wire_payload.t, wire_payload.h);

        // TBD...
        // // 3. Fire the bytes down the Zephyr POSIX/MQTT network stack via FFI
        // crate::ffi::zephyr_mqtt_publish(buf, serialized)
        //     .map_err(|_| "Network stack reject outbound message")
        Ok(())
    }
}