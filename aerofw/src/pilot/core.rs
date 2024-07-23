
use crate::constants;
use crate::controllers::controller::PinState;
use crate::controllers::{controller::Controller, rp::RP};
use crate::pilot::error::Error;

use aeroapi::data::commands::Channels;
use aeroapi::data::sensors::Sensors;
use embassy_executor::Spawner;
use embassy_time::Timer;
use defmt::info;
use aeroapi::data;

enum SupportedControllers {
    RP2040,
}

fn get_controller(name: SupportedControllers, spawner: Spawner) -> impl Controller {
    match name {
        SupportedControllers::RP2040 => RP::new(spawner)
    }
}

#[derive(Debug)]
struct State {
    sensors: data::sensors::Sensors,
}

pub struct Core {
    state: State,
}

impl Core {
    pub fn new() -> Self {
        let state = State {
            sensors: data::sensors::Sensors::new(None, None, None, None),
        };
        
        Core {
            state,
        }
    }

    fn update_sensors(&mut self, sensors: Sensors) {
        self.state.sensors = sensors;
    }
}

impl Core {
    pub async fn run(&mut self, spawner: Spawner) -> Result<(), Error> {
        info!("Running core");
        let mut controller = get_controller(SupportedControllers::RP2040, spawner);
    
        controller.set_status_led(PinState::High);
        loop {
            info!("Running loop");
            let read_buffer = controller.read_from_usb().await?;
    
            // set power control register -------------------------------------
            let reg_pwr_ctrl = 0x1b_u8;
            let val_pwr_ctrl = 0b00110011_u8;
    
            let tx_buf = [reg_pwr_ctrl, val_pwr_ctrl];
            let mut rx_buf = [0_u8; 16];
    
            controller.spi_read(&tx_buf, &mut rx_buf).await?;
            // ----------------------------------------------------------------
    
            // set chip id register ------------------------------------------
            let reg_chip_id = 0x80_u8;
            let val_chip_id = 0x00_u8;
    
            let tx_buf = [reg_chip_id, val_chip_id];
            let mut rx_buf = [0_u8;60];
    
            controller.spi_read(&tx_buf, &mut rx_buf).await?;
            // ----------------------------------------------------------------
    
            // set chip id register ------------------------------------------
            let reg_cal = 0xb1_u8;
            let val_cal = 0x00_u8;
    
            let tx_buf = [reg_cal, val_cal];
            let mut rx_buf2 = [0_u8; 24];
    
            controller.spi_read(&tx_buf, &mut rx_buf2).await?;
            // ----------------------------------------------------------------
    
            
    
            // controller.write_to_usb(zero_pad(&rx_buf)).await?;
            // controller.write_to_usb(zero_pad(&rx_buf2)).await?;
    
            let mut write_buffer = [0_u8; constants::usb::converted::MAX_PACKET_SIZE_USIZE];
            let updated_sensors = self.state.sensors.with_altimeter(10.0);
            self.update_sensors(updated_sensors);

            let command = create_control_input(0, 100, 0, 0);
            let ser = postcard::to_slice(&command, &mut write_buffer).unwrap();
    
            controller.write_to_usb(write_buffer).await?;
    
            Timer::after_millis(1000).await;
            controller.set_status_led(PinState::Low);
            Timer::after_millis(1000).await;
    
    
            controller.set_status_led(PinState::High);
            info!("Loop done");
        }
    }
}

fn create_control_input(throttle: u8, yaw: u8, pitch: u8, roll: u8) -> data::commands::Controller {
    let command = data::commands::Controller::new();
    command.update_channels(Channels {throttle, yaw, pitch, roll})
}
