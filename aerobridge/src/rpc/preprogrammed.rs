use aeroapi::data::commands::Controller;

use crate::rpc::message::create_control_input;
use crate::rpc::errors::RpcError;
use crate::rpc::hardware::HardwareConnection;

pub struct PreProgrammed {
    current: usize,
    inputs: Vec<aeroapi::data::commands::Controller>,
}

fn inputs_1() -> Vec<Controller> {
    vec![
            // forward
            // Takeoff and hover
            create_control_input(vec![55, 50, 50, 50], false, false),
            create_control_input(vec![60, 50, 50, 50], false, false),
            create_control_input(vec![65, 50, 50, 50], false, false),
            create_control_input(vec![70, 50, 50, 50], false, false),

            // Move forward slowly
            create_control_input(vec![70, 50, 55, 50], false, false),
            create_control_input(vec![70, 50, 60, 50], false, false),
            create_control_input(vec![70, 50, 65, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 70, 50], false, false),
            create_control_input(vec![70, 50, 65, 50], false, false),
            create_control_input(vec![70, 50, 60, 50], false, false),
            create_control_input(vec![70, 50, 55, 50], false, false),
            create_control_input(vec![70, 50, 50, 50], false, false),

            // Yaw right gradually
            create_control_input(vec![70, 55, 50, 50], false, false),
            create_control_input(vec![70, 60, 50, 50], false, false),
            create_control_input(vec![70, 65, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 70, 50, 50], false, false),
            create_control_input(vec![70, 65, 50, 50], false, false),
            create_control_input(vec![70, 60, 50, 50], false, false),
            create_control_input(vec![70, 55, 50, 50], false, false),
            create_control_input(vec![70, 50, 50, 50], false, false),

            // Move forward and slightly right
            create_control_input(vec![70, 52, 55, 52], false, false),
            create_control_input(vec![70, 54, 60, 54], false, false),
            create_control_input(vec![70, 56, 65, 56], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 58, 70, 58], false, false),
            create_control_input(vec![70, 56, 65, 56], false, false),
            create_control_input(vec![70, 54, 60, 54], false, false),
            create_control_input(vec![70, 52, 55, 52], false, false),
            create_control_input(vec![70, 50, 50, 50], false, false),

            // Yaw left gradually
            create_control_input(vec![70, 45, 50, 50], false, false),
            create_control_input(vec![70, 40, 50, 50], false, false),
            create_control_input(vec![70, 35, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 30, 50, 50], false, false),
            create_control_input(vec![70, 35, 50, 50], false, false),
            create_control_input(vec![70, 40, 50, 50], false, false),
            create_control_input(vec![70, 45, 50, 50], false, false),
            create_control_input(vec![70, 50, 50, 50], false, false),

            // Move backward slowly
            create_control_input(vec![70, 50, 45, 50], false, false),
            create_control_input(vec![70, 50, 40, 50], false, false),
            create_control_input(vec![70, 50, 35, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 30, 50], false, false),
            create_control_input(vec![70, 50, 35, 50], false, false),
            create_control_input(vec![70, 50, 40, 50], false, false),
            create_control_input(vec![70, 50, 45, 50], false, false),
            create_control_input(vec![70, 50, 50, 50], false, false),

            // Land slowly
            create_control_input(vec![65, 50, 50, 50], false, false),
            create_control_input(vec![60, 50, 50, 50], false, false),
            create_control_input(vec![55, 50, 50, 50], false, false),
            create_control_input(vec![50, 50, 50, 50], false, false),
        ]
}


fn shikanoko() -> Vec<Controller> {
    vec![
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),


        create_control_input(vec![50, 50, 50, 25], false, false),
        create_control_input(vec![50, 50, 50, 0], false, false),
        create_control_input(vec![50, 50, 50, 0], false, false),
        create_control_input(vec![50, 50, 50, 25], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 50, 75], false, false),
        create_control_input(vec![50, 50, 50, 100], false, false),
        create_control_input(vec![50, 50, 50, 100], false, false),
        create_control_input(vec![50, 50, 50, 75], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 50, 25], false, false),
        create_control_input(vec![50, 50, 50, 0], false, false),
        create_control_input(vec![50, 50, 50, 0], false, false),
        create_control_input(vec![50, 50, 50, 25], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 50, 75], false, false),
        create_control_input(vec![50, 50, 50, 100], false, false),
        create_control_input(vec![50, 50, 50, 100], false, false),
        create_control_input(vec![50, 50, 50, 75], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 50, 25], false, false),
        create_control_input(vec![50, 50, 50, 0], false, false),
        create_control_input(vec![50, 50, 50, 0], false, false),
        create_control_input(vec![50, 50, 50, 25], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 50, 75], false, false),
        create_control_input(vec![50, 50, 50, 100], false, false),
        create_control_input(vec![50, 50, 50, 100], false, false),
        create_control_input(vec![50, 50, 50, 75], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),



        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),

        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),

        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),

        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),

        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),        create_control_input(vec![50, 50, 50, 50], false, false),
        create_control_input(vec![50, 50, 50, 50], false, false),

        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),
        create_control_input(vec![50, 50, 100, 50], false, false),

        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
        create_control_input(vec![50, 50, 0, 50], false, false),
    ]
}

fn rotate() -> Vec<Controller> {
    vec![
        create_control_input(vec![50, 60, 50, 50], false, false),
    ]
}

impl PreProgrammed {
    pub fn new() -> Self {
        let inputs = inputs_1();
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