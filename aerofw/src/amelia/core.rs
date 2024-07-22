
use crate::controllers::controller::PinState;
use crate::controllers::{controller::Controller, rp::RP};
use crate::amelia::error::Error;

use embassy_executor::Spawner;
use embassy_time::Timer;
use defmt::info;

enum SupportedControllers {
    RP2040,
}

fn get_controller(name: SupportedControllers, spawner: Spawner) -> impl Controller {
    match name {
        SupportedControllers::RP2040 => RP::new(spawner)
    }
}

fn zero_pad(data: &[u8]) -> [u8; 64] {
    let mut padded_data = [0u8; 64];
    padded_data[..data.len()].copy_from_slice(data);
    padded_data
}


pub async fn run(spawner: Spawner) -> Result<(), Error> {
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

        controller.write_to_usb(zero_pad(&rx_buf)).await?;
        controller.write_to_usb(zero_pad(&rx_buf2)).await?;
        controller.write_to_usb(read_buffer).await?;

        Timer::after_millis(1000).await;
        controller.set_status_led(PinState::Low);
        Timer::after_millis(1000).await;


        controller.set_status_led(PinState::High);
        info!("Loop done");
    }
}