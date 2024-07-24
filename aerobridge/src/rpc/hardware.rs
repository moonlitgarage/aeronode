use crate::rpc::errors::RpcError;
use aeroapi::data;

pub trait HardwareConnection: Send {
    fn send(&mut self, data: &data::sensors::Sensors) -> Result<(), RpcError>;
    fn read(&mut self) -> Result<data::commands::Controller, RpcError>;
}
