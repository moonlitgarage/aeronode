use aeroapi::data::commands::Switches;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelId {
    LeftY,
    LeftX,
    RightY,
    RightX,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: ChannelId,
    pub channel_val: i64,
    pub min: i64,
    pub max: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlInput {
    pub channels: Vec<Channel>,
    pub switch_1: bool,
    pub switch_2: bool,
}

impl From<aeroapi::data::commands::Controller> for ControlInput {
    fn from(controller: aeroapi::data::commands::Controller) -> Self {
        let channels = vec![
            Channel { channel_id: ChannelId::LeftY, channel_val: controller.channels.throttle as i64, min: 0, max: 100 },
            Channel { channel_id: ChannelId::LeftX, channel_val: controller.channels.yaw as i64, min: 0, max: 100 },
            Channel { channel_id: ChannelId::RightY, channel_val: controller.channels.pitch as i64, min: 0, max: 100 },
            Channel { channel_id: ChannelId::RightX, channel_val: controller.channels.roll as i64, min: 0, max: 100 },
        ];
        
        ControlInput { channels, switch_1: controller.switches.arm, switch_2: controller.switches.autopilot }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorData {
    pub imu: Imu,
    pub altitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Imu {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl SensorData {
    pub fn to_aeroapi_sensor(&self) -> aeroapi::data::sensors::Sensors {
        aeroapi::data::sensors::Sensors::new(
            Some(self.altitude as f32),
            Some(aeroapi::data::commons::Vec3d{x: self.imu.x as f32, y: self.imu.y as f32, z: self.imu.z as f32}),
            None,
            None,
        )
    }
}

pub fn create_control_input(channel_values: Vec<i64>, switch_1: bool, switch_2: bool) -> aeroapi::data::commands::Controller {
    if channel_values.len() != 4 {
        panic!("There must be exactly 4 channel values");
    }
    
    let channels = aeroapi::data::commands::Channels {
        throttle: channel_values[0] as u8,
        yaw: channel_values[1] as u8,
        pitch: channel_values[2] as u8,
        roll: channel_values[3] as u8,
    };

    let switches = aeroapi::data::commands::Switches {
        arm: switch_1,
        autopilot: switch_2,
    };
    
    aeroapi::data::commands::Controller { channels, switches }
}
