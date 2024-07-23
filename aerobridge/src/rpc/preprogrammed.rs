use crate::rpc::message::{self, create_control_input};
use crate::rpc::errors::RpcError;
use crate::rpc::hardware::HardwarConnection;

pub struct PreProgrammed {
    current: usize,
    inputs: Vec<message::ControlInput>,
}

impl PreProgrammed {
    pub fn new() -> Self {
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

impl HardwarConnection for PreProgrammed {
    fn send(&mut self, _data: &message::SensorData) -> Result<(), RpcError> {
        Ok(())
    }

    fn read(&mut self) -> Result<message::ControlInput, RpcError> {
        let current_input = self.inputs[self.current].clone();
        self.current = (self.current + 1) % self.inputs.len();
        Ok(current_input)
    }
}