use crate::rpc::message;
use crate::rpc::errors::RpcError;

pub trait HardwarConnection: Send {
    fn send(&mut self, data: &message::SensorData) -> Result<(), RpcError>;
    fn read(&mut self) -> Result<message::ControlInput, RpcError>;
}
