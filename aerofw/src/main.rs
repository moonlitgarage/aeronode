#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

mod pilot;
mod constants;
mod controllers;
mod io;
mod message;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut core = pilot::core::Core::new();
    let status = core.run(spawner).await;
    match status {
        Ok(()) => info!("this should not happened"),
        Err(_) => info!("ayo")
    }

    loop {}
}
