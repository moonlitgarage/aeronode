use std::error::Error;
use std::thread;
use std::time::Duration;
use log::{error, info};
use serde_json;
use xmlrpc::{Request, Value};

use crate::message::SensorData;
use crate::message::ControlInput;
use crate::message::Channel;
use crate::message::ChannelId;
use crate::message::Imu;

trait AbstractConn {
    fn send(&mut self, data: &SensorData) -> Result<(), Box<dyn Error>>;
    fn read(&mut self) -> Result<ControlInput, Box<dyn Error>>;
}

struct SerialWrapper {
}

impl SerialWrapper {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SerialWrapper {})
    }
}

impl AbstractConn for SerialWrapper {
    fn send(&mut self, _data: &SensorData) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn read(&mut self) -> Result<ControlInput, Box<dyn Error>> {
        Ok(create_control_input(vec![50, 0, 50, 50], false, false))
    }
}


fn create_control_input(channel_values: Vec<i64>, switch_1: bool, switch_2: bool) -> ControlInput {
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

struct PreProgrammed {
    current: usize,
    inputs: Vec<ControlInput>,
}

impl PreProgrammed {
    fn new() -> Self {
        let inputs = vec![
            // forward
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),

            // yaw right
            create_control_input(vec![50, 100, 50, 50], false, false),
            create_control_input(vec![50, 100, 50, 50], false, false),
            create_control_input(vec![50, 100, 50, 50], false, false),
            create_control_input(vec![50, 100, 50, 50], false, false),
            create_control_input(vec![50, 100, 50, 50], false, false),

            // forward
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),
            create_control_input(vec![50, 50, 100, 50], false, false),

            // yaw left
            create_control_input(vec![50, 0, 50, 50], false, false),
            create_control_input(vec![50, 0, 50, 50], false, false),
            create_control_input(vec![50, 0, 50, 50], false, false),
            create_control_input(vec![50, 0, 50, 50], false, false),
            create_control_input(vec![50, 0, 50, 50], false, false),

            // back
            create_control_input(vec![50, 50, 0, 50], false, false),
            create_control_input(vec![50, 50, 0, 50], false, false),
            create_control_input(vec![50, 50, 0, 50], false, false),
            create_control_input(vec![50, 50, 0, 50], false, false),
            create_control_input(vec![50, 50, 0, 50], false, false),
            create_control_input(vec![50, 50, 0, 50], false, false),
        ];
        PreProgrammed { current: 0, inputs }
    }
}

impl AbstractConn for PreProgrammed {
    fn send(&mut self, _data: &SensorData) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn read(&mut self) -> Result<ControlInput, Box<dyn Error>> {
        let current_input = self.inputs[self.current].clone();
        self.current = (self.current + 1) % self.inputs.len();
        Ok(current_input)
    }
}

struct Node {
    conn: Box<dyn AbstractConn>,
}

impl Node {
    fn new() -> Self {
        let conn = Box::new(PreProgrammed::new());
        Node { conn }
    }

    fn send_data(&mut self, data: &SensorData) -> Result<(), Box<dyn Error>> {
        self.conn.send(data)
    }

    fn receive_control_input(&mut self) -> Result<ControlInput, Box<dyn Error>> {
        self.conn.read()
    }
}

struct AeroBridge {
    node: Node,
    server_url: String,
    running: bool,
    connected: bool,
}

impl AeroBridge {
    fn new(server_url: String) -> Self {
        AeroBridge {
            node: Node::new(),
            server_url,
            running: false,
            connected: false,
        }
    }

    fn parse_sensor_data(&self, value: Value) -> Result<SensorData, Box<dyn Error>> {
        match value {
            Value::Struct(map) => {
                let altitude = map.get("altitude")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing or invalid altitude")?;

                let imu = map.get("imu")
                    .and_then(|v| v.as_struct())
                    .ok_or("Missing or invalid IMU data")?;

                let x = imu.get("x")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing or invalid IMU x value")?;
                let y = imu.get("y")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing or invalid IMU y value")?;
                let z = imu.get("z")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing or invalid IMU z value")?;

                Ok(SensorData {
                    altitude,
                    imu: Imu { x, y, z },
                })
            },
            _ => Err("Invalid sensor data format".into()),
        }
    }

    fn connect(&mut self) {
        let request = Request::new("get_sensor_data");
        match request.call_url(&self.server_url) {
            Ok(_) => {
                self.connected = true;
                info!("Connected to RPC server successfully");
            }
            Err(e) => {
                error!("Cannot establish connection: {}", e);
                self.connected = false;
            }
        }
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.running = true;
        self.connect();
    
        let start_request = Request::new("start");
        let start_result = start_request.call_url(&self.server_url)?;
        println!("Start result: {:?}", start_result);
    
        while self.running {
            let get_sensor_data_request = Request::new("get_sensor_data");
            let sensor_data_value = get_sensor_data_request.call_url(&self.server_url)?;

            let sensor_data = self.parse_sensor_data(sensor_data_value)?;
                        
            self.node.send_data(&sensor_data)?;
            
            let ci = self.node.receive_control_input()?;
            let ci_json = serde_json::to_string(&ci)?;
            
            let handle_control_input_request = Request::new("handle_control_input").arg(ci_json);
            handle_control_input_request.call_url(&self.server_url)?;
            
            thread::sleep(Duration::from_millis(500));
        }
    
        Ok(())
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    
    let mut aero_bridge = AeroBridge::new("http://localhost:8000/RPC2".to_string());
    aero_bridge.run()?;
    
    info!("Program finished");
    Ok(())
}