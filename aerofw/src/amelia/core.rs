
use crate::controllers::controller::PinState;
use crate::controllers::{controller::Controller, rp::RP};
use crate::amelia::error::Error;

use embassy_executor::Spawner;

enum SupportedControllers {
    RP2040,
}

fn get_controller(name: SupportedControllers, spawner: Spawner) -> impl Controller {
    match name {
        SupportedControllers::RP2040 => RP::new(spawner)
    }
}

pub async fn run(spawner: Spawner) -> Result<(), Error> {
    // init controller
    let mut controller = get_controller(SupportedControllers::RP2040, spawner);

    // setup communication channels

    // main loop
    controller.set_status_led(PinState::High);
    loop {
        let read_buffer = controller.read_from_usb().await?;
        controller.write_to_usb(read_buffer).await?;
    }
}
