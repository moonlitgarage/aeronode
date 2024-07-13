#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

// NOTE: keep module declarations in lexicographical order
mod amelia;
mod constants;
mod controllers;
mod io;
mod message;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let status = amelia::core::run(spawner).await;
    match status {
        Ok(()) => info!("this should not happened"),
        Err(_) => info!("ayo")
    }

    loop {}
}
