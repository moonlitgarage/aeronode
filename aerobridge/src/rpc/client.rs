use std::error::Error;
use log::{error, info};
use serde_json;
use tokio::sync::mpsc::UnboundedSender;
use xmlrpc::{Request, Value};


use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::rpc::message;
use crate::rpc::errors::RpcError;
use crate::rpc::hardware::HardwarConnection;
use crate::rpc::preprogrammed::PreProgrammed;
use crate::rpc::serial::SerialWrapper;

struct Node {
    conn: Box<dyn HardwarConnection + Send>,
}

pub enum NodeConnection {
    Serial,
    PreProgrammed
}

impl Node {
    fn new(mode: NodeConnection) -> Self {
        let conn: Box<dyn HardwarConnection + Send> = match mode {
            NodeConnection::Serial => Box::new(SerialWrapper::new().unwrap()),
            NodeConnection::PreProgrammed => Box::new(PreProgrammed::new()),
            
        };
        Node { conn }
    }

    fn send_data(&mut self, data: &aeroapi::data::sensors::Sensors) -> Result<(), RpcError> {
        self.conn.send(data)
    }

    fn receive_control_input(&mut self) -> Result<aeroapi::data::commands::Controller, RpcError> {
        self.conn.read()
    }
}

struct AeroBridge {
    node: Node,
    server_url: String,
    running: bool,
    connected: bool,
    tx: UnboundedSender<aeroapi::data::commands::Controller>,
}

impl AeroBridge {
    fn new(mode: NodeConnection, server_url: String, tx: UnboundedSender<aeroapi::data::commands::Controller>) -> Self {
        AeroBridge {
            node: Node::new(mode),
            server_url,
            running: false,
            connected: false,
            tx,
        }
    }

    fn parse_sensor_data(&self, value: Value) -> Result<aeroapi::data::sensors::Sensors, Box<dyn Error>> {
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

                let sensor_data = aeroapi::data::sensors::Sensors::new(
                    Some(altitude as f32),
                    Some(aeroapi::data::commons::Vec3d{x: x as f32, y: y as f32, z: z as f32}),
                    None,
                    None,
                );
                    
                Ok(sensor_data)
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
            self.tx.send(ci.clone()).map_err(|e| RpcError("Failed to start".to_string()))?;
            let ci_json = serde_json::to_string(&message::ControlInput::from(ci)).map_err(|e| RpcError("Failed to start".to_string()))?;
            
            let handle_control_input_request = Request::new("handle_control_input").arg(ci_json);
            handle_control_input_request.call_url(&self.server_url).map_err(|e| RpcError("Failed to start".to_string())).map_err(|e| RpcError("Failed to start".to_string()))?;
            
            // tokio::time::sleep(Duration::from_millis(500)).await;
        }
    
        Ok(())
    }
}


pub fn run(mode: NodeConnection, tx: UnboundedSender<aeroapi::data::commands::Controller>, running: Arc<AtomicBool>) -> Result<(), RpcError> {
    let mut aero_bridge = AeroBridge::new(mode, "http://localhost:8000/RPC2".to_string(), tx);

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
        let ci_json = serde_json::to_string(&message::ControlInput::from(ci))
            .map_err(|e| RpcError("Failed to serialize control input".to_string()))?;
        aero_bridge.tx.send(ci.clone())
            .map_err(|e| RpcError("Failed to send control input".to_string()))?;
        
        let handle_control_input_request = Request::new("handle_control_input").arg(ci_json);
        handle_control_input_request.call_url(&aero_bridge.server_url)
            .map_err(|e| RpcError("Failed to handle control input".to_string()))?;
        
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}