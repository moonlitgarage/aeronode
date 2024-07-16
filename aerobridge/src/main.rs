use serde_json;

// Include the structs and enums you provided here
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlInput {
    pub channels: Vec<Channel>,
    pub switch_1: bool,
    pub switch_2: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelId {
    #[default]
    LeftY,
    LeftX,
    RightY,
    RightX,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: ChannelId,
    pub channel_val: i64,
    pub min: i64,
    pub max: i64,
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

fn main() {
    // Example 1: Creating and serializing a ControlInput
    let control_input = ControlInput {
        channels: vec![
            Channel {
                channel_id: ChannelId::LeftY,
                channel_val: 100,
                min: 0,
                max: 255,
            },
            Channel {
                channel_id: ChannelId::RightX,
                channel_val: -50,
                min: -100,
                max: 100,
            },
        ],
        switch_1: true,
        switch_2: false,
    };

    let serialized = serde_json::to_string(&control_input).unwrap();
    println!("Serialized ControlInput: {}", serialized);

    // Example 2: Deserializing a ControlInput
    let deserialized: ControlInput = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized ControlInput: {:?}", deserialized);

    // Example 3: Creating and serializing a SensorData
    let sensor_data = SensorData {
        imu: Imu {
            x: 0.5,
            y: -0.2,
            z: 9.8,
        },
        altitude: 123.45,
    };

    let serialized = serde_json::to_string(&sensor_data).unwrap();
    println!("Serialized SensorData: {}", serialized);

    // Example 4: Deserializing a SensorData
    let deserialized: SensorData = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized SensorData: {:?}", deserialized);

    // Example 5: Modifying a ControlInput
    let mut control_input = ControlInput::default();
    control_input.switch_1 = true;
    control_input.channels.push(Channel {
        channel_id: ChannelId::LeftX,
        channel_val: 75,
        min: 0,
        max: 100,
    });

    println!("Modified ControlInput: {:?}", control_input);

    // Example 6: Comparing ControlInputs
    let control_input_1 = ControlInput::default();
    let control_input_2 = control_input.clone();
    println!("ControlInputs are equal: {}", control_input_1 == control_input_2);
}