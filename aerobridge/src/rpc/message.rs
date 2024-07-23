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

pub fn create_control_input(channel_values: Vec<i64>, switch_1: bool, switch_2: bool) -> ControlInput {
    if channel_values.len() != 4 {
        panic!("There must be exactly 4 channel values");
    }
    
    let channels = vec![
        Channel { channel_id: ChannelId::LeftY, channel_val: channel_values[0], min: 0, max: 100 },
        Channel { channel_id: ChannelId::LeftX, channel_val: channel_values[1], min: 0, max: 100 },
        Channel { channel_id: ChannelId::RightY, channel_val: channel_values[2], min: 0, max: 100 },
        Channel { channel_id: ChannelId::RightX, channel_val: channel_values[3], min: 0, max: 100 },
    ];
    
    ControlInput { channels, switch_1, switch_2 }
}
