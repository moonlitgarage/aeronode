use std::{error::Error, io::{self, Write}, time::Duration};

use serialport::SerialPort;

use super::{errors::RpcError, hardware::HardwarConnection, message::{self, create_control_input}};



pub struct SerialWrapper {
    port: Box<dyn SerialPort>,
}

impl SerialWrapper {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let port_name = "/dev/tty.usbmodem11";
        let baud = 115200;
        
        let port: Box<dyn SerialPort> = serialport::new(port_name, baud)
            .timeout(Duration::from_millis(10))
            .open()?;

        Ok(SerialWrapper {
            port
        })
    }
}

impl HardwarConnection for SerialWrapper {
    fn send(&mut self, _data: &aeroapi::data::sensors::Sensors) -> Result<(), RpcError> {

        let message = "Hello, Pico!\n";
        self.port.write(message.as_bytes()).map_err(|e| RpcError(e.to_string()))?;

        Ok(())
    }

    fn read(&mut self) -> Result<aeroapi::data::commands::Controller, RpcError> {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        match self.port.read(serial_buf.as_mut_slice()) {
            Ok(0) => {
                // No data received
                println!("No data received");
            }
            Ok(t) => {
                println!("Received: {:?}", serial_buf);
                io::stdout().flush().map_err(|e| RpcError(e.to_string()))?;
                let chipid = &serial_buf[2];
                let recid = &serial_buf[3];

                let err_reg = &serial_buf[4];
                let status_reg = &serial_buf[5];

                
                let data0 = &serial_buf[6];
                let data1 = &serial_buf[7];
                let data2 = &serial_buf[8];

                let data3 = &serial_buf[9];
                let data4 = &serial_buf[10];
                let data5 = &serial_buf[11];

                let alt = postcard::from_bytes::<aeroapi::data::commands::Controller>(&serial_buf).unwrap();

                println!("#####################################");
                println!("Postcard: {:?}", alt);
                // println!("Chip ID: {chipid}, {chipid:#X}");
                // println!("Received ID: {recid}, {recid:#X}");
                // println!("-------------------------------------");
                // println!("Error Register: {err_reg}, {err_reg:#X}");
                // println!("Status Register: {status_reg}, {status_reg:#X}");
                // println!("-------------------------------------");
                // println!("Data0: {data0}, {data0:#X}");
                // println!("Data1: {data1}, {data1:#X}");
                // println!("Data2: {data2}, {data2:#X}");
                // println!("-------------------------------------");
                // println!("Data3: {data3}, {data3:#X}");
                // println!("Data4: {data4}, {data4:#X}");
                // println!("Data5: {data5}, {data5:#X}");
                // println!("#####################################");
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                println!("Timeout occurred, no data received");
            }
            Err(e) => eprintln!("Error: {}", e),
        }

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