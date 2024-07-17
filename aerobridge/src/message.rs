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