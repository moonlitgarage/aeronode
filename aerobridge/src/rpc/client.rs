use std::error::Error;
use log::{error, info};
use serde_json;
use tokio::sync::mpsc::UnboundedSender;
use xmlrpc::{Request, Value};


use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::rpc::message::{self, create_control_input};
use crate::rpc::errors::RpcError;
use crate::rpc::hardware::HardwarConnection;
use crate::rpc::preprogrammed::PreProgrammed;

struct SerialWrapper {
}

impl SerialWrapper {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SerialWrapper {})
    }
}

impl HardwarConnection for SerialWrapper {
    fn send(&mut self, _data: &message::SensorData) -> Result<(), RpcError> {
        Ok(())
    }

    fn read(&mut self) -> Result<message::ControlInput, RpcError> {
        Ok(create_control_input(vec![50, 0, 50, 50], false, false))
    }
}

struct Node {
    conn: Box<dyn HardwarConnection + Send>,
}

impl Node {
    fn new() -> Self {
        let conn: Box<dyn HardwarConnection + Send> = Box::new(PreProgrammed::new());
        Node { conn }
    }

    fn send_data(&mut self, data: &message::SensorData) -> Result<(), RpcError> {
        self.conn.send(data)
    }

    fn receive_control_input(&mut self) -> Result<message::ControlInput, RpcError> {
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

    fn parse_sensor_data(&self, value: Value) -> Result<message::SensorData, Box<dyn Error>> {
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

                Ok(message::SensorData {
                    altitude,
                    imu: message::Imu { x, y, z },
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