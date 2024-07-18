use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::unbounded_channel;
use log::{error, info};
use serde_json;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use xmlrpc::{Request, Value};


use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::message::SensorData;
use crate::message::ControlInput;
use crate::message::Channel;
use crate::message::ChannelId;
use crate::message::Imu;

use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct RpcError(String);

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for RpcError {}

trait AbstractConn: Send {
    fn send(&mut self, data: &SensorData) -> Result<(), RpcError>;
    fn read(&mut self) -> Result<ControlInput, RpcError>;
}
struct SerialWrapper {
}

impl SerialWrapper {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SerialWrapper {})
    }
}

impl AbstractConn for SerialWrapper {
    fn send(&mut self, _data: &SensorData) -> Result<(), RpcError> {
        Ok(())
    }

    fn read(&mut self) -> Result<ControlInput, RpcError> {
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
    fn send(&mut self, _data: &SensorData) -> Result<(), RpcError> {
        Ok(())
    }

    fn read(&mut self) -> Result<ControlInput, RpcError> {
        let current_input = self.inputs[self.current].clone();
        self.current = (self.current + 1) % self.inputs.len();
        Ok(current_input)
    }
}

struct Node {
    conn: Box<dyn AbstractConn + Send>,
}

impl Node {
    fn new() -> Self {
        let conn: Box<dyn AbstractConn + Send> = Box::new(PreProgrammed::new());
        Node { conn }
    }

    fn send_data(&mut self, data: &SensorData) -> Result<(), RpcError> {
        self.conn.send(data)
    }

    fn receive_control_input(&mut self) -> Result<ControlInput, RpcError> {
        self.conn.read()
    }
}

struct AeroBridge {
    node: Node,
    server_url: String,
    running: bool,
    connected: bool,
    tx: UnboundedSender<String>,
}

impl AeroBridge {
    fn new(server_url: String, tx: UnboundedSender<String>) -> Self {
        AeroBridge {
            node: Node::new(),
            server_url,
            running: false,
            connected: false,
            tx,
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

    async fn run(&mut self) -> Result<(), RpcError> {
        self.running = true;
        self.connect();
    
        let start_request = Request::new("start");
        let start_result = start_request.call_url(&self.server_url).map_err(|e| RpcError("Failed to start".to_string()))?;
        println!("Start result: {:?}", start_result);
    
        while self.running {
            let get_sensor_data_request = Request::new("get_sensor_data");
            let sensor_data_value = get_sensor_data_request.call_url(&self.server_url).map_err(|e| RpcError("Failed to start".to_string()))?;

            let sensor_data = self.parse_sensor_data(sensor_data_value).map_err(|e| RpcError("Failed to start".to_string()))?;
                        
            self.node.send_data(&sensor_data)?;
            
            let ci = self.node.receive_control_input()?;
            let ci_json = serde_json::to_string(&ci).map_err(|e| RpcError("Failed to start".to_string())).map_err(|e| RpcError("Failed to start".to_string()))?;
            self.tx.send(ci_json.clone()).map_err(|e| RpcError("Failed to start".to_string()))?;
            
            let handle_control_input_request = Request::new("handle_control_input").arg(ci_json);
            handle_control_input_request.call_url(&self.server_url).map_err(|e| RpcError("Failed to start".to_string())).map_err(|e| RpcError("Failed to start".to_string()))?;
            
            // tokio::time::sleep(Duration::from_millis(500)).await;
        }
    
        Ok(())
    }
}


pub fn run(tx: UnboundedSender<String>, running: Arc<AtomicBool>) -> Result<(), RpcError> {
    let mut aero_bridge = AeroBridge::new("http://localhost:8000/RPC2".to_string(), tx);

    aero_bridge.connect();

        
    let start_request = Request::new("start");
    let start_result = start_request.call_url(&aero_bridge.server_url).map_err(|e| RpcError("Failed to start".to_string()))?;
    println!("Start result: {:?}", start_result);

    
    while running.load(Ordering::SeqCst) {
        let get_sensor_data_request = Request::new("get_sensor_data");
        let sensor_data_value = get_sensor_data_request.call_url(&aero_bridge.server_url)
            .map_err(|e| RpcError("Failed to get sensor data".to_string()))?;

        let sensor_data = aero_bridge.parse_sensor_data(sensor_data_value)
            .map_err(|e| RpcError("Failed to parse sensor data".to_string()))?;
                    
        aero_bridge.node.send_data(&sensor_data)?;
        
        let ci = aero_bridge.node.receive_control_input()?;
        let ci_json = serde_json::to_string(&ci)
            .map_err(|e| RpcError("Failed to serialize control input".to_string()))?;
        aero_bridge.tx.send(ci_json.clone())
            .map_err(|e| RpcError("Failed to send control input".to_string()))?;
        
        let handle_control_input_request = Request::new("handle_control_input").arg(ci_json);
        handle_control_input_request.call_url(&aero_bridge.server_url)
            .map_err(|e| RpcError("Failed to handle control input".to_string()))?;
        
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}