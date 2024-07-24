use std::error;

use crate::rpc::client::{DataType, SimData};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    pub controller: aeroapi::data::commands::Controller,
    pub sensors: aeroapi::data::sensors::Sensors,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            controller: aeroapi::data::commands::Controller {
                channels: aeroapi::data::commands::Channels {
                    throttle: 0u8,
                    yaw: 0u8,
                    pitch: 0u8,
                    roll: 0u8,
                },
                switches: aeroapi::data::commands::Switches {
                    arm: false,
                    autopilot: false,
                },
            },
            sensors: aeroapi::data::sensors::Sensors::new(None, None, None, None)
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn update_message(&mut self, simdata: SimData) {
        match simdata.data_type {
            DataType::ControlInput => {
                self.controller = simdata.control_input.unwrap();
            },
            DataType::SensorData => {
                self.sensors = simdata.sensor_data.unwrap();
            },
        }
    }
}
