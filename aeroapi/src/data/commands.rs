use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Controller {
    pub channels: Channels,
    pub switches: Switches,
}

impl Controller {
    pub fn new() -> Self {
        let channels = Channels {
            throttle: 0,
            yaw: 0,
            pitch: 0,
            roll: 0,
        };

        let switches = Switches {
            arm: false,
            autopilot: false,
        };

        Controller { channels, switches }
    }

    pub fn update_channels(self, channels: Channels) -> Self {
        Controller { channels, ..self }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Channels {
    pub throttle: u8,
    pub yaw: u8,
    pub pitch: u8,
    pub roll: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Switches {
    pub arm: bool,
    pub autopilot: bool,
}
