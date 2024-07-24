use crate::rpc::message::create_control_input;
use crate::rpc::errors::RpcError;
use crate::rpc::hardware::HardwareConnection;

pub struct PreProgrammed {
    current: usize,
    inputs: Vec<aeroapi::data::commands::Controller>,
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

impl HardwareConnection for PreProgrammed {
    fn send(&mut self, _data: &aeroapi::data::sensors::Sensors) -> Result<(), RpcError> {
        Ok(())
    }

    fn read(&mut self) -> Result<aeroapi::data::commands::Controller, RpcError> {
        let current_input = self.inputs[self.current].clone();
        self.current = (self.current + 1) % self.inputs.len();
        Ok(current_input)
    }
}