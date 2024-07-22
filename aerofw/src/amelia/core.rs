
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

        let tx_buf = [0x82_u8, 0];
        let mut rx_buf = [0_u8; 16];

        info!("SPI write: {:?}", tx_buf);
        controller.spi_read(&tx_buf, &mut rx_buf).await?;
        info!("SPI read: {:?}", rx_buf);

        controller.write_to_usb(zero_pad(&rx_buf)).await?;
        controller.write_to_usb(read_buffer).await?;

        Timer::after_millis(1000).await;
        controller.set_status_led(PinState::Low);
        Timer::after_millis(1000).await;


        controller.set_status_led(PinState::High);
        info!("Loop done");
    }
}