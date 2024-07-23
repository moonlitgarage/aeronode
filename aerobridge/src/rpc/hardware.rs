use crate::rpc::message;
use crate::rpc::errors::RpcError;

pub trait HardwarConnection: Send {
    fn send(&mut self, data: &aeroapi::data::sensors::Sensors) -> Result<(), RpcError>;
    fn read(&mut self) -> Result<aeroapi::data::commands::Controller, RpcError>;
}
