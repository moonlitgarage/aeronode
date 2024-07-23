use std::error::Error;

use super::{errors::RpcError, hardware::HardwarConnection, message::{self, create_control_input}};



struct SerialWrapper {
}

impl SerialWrapper {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SerialWrapper {})
    }
}

impl HardwarConnection for SerialWrapper {
    fn send(&mut self, _data: &aeroapi::data::sensors::Sensors) -> Result<(), RpcError> {
        Ok(())
    }

    fn read(&mut self) -> Result<aeroapi::data::commands::Controller, RpcError> {
        Ok(aeroapi::data::commands::Controller {
            channels: aeroapi::data::commands::Channels {
                throttle: 50,
                yaw: 50,
                pitch: 50,
                roll: 50,
            },
            switches: aeroapi::data::commands::Switches {
                arm: false,
                autopilot: false,
            }
        })
    }
}