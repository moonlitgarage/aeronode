#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

mod core;
mod message;
mod io;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    core::main::main(spawner).await;
    // let p = embassy_rp::init(Default::default());
    // let mut led = Output::new(p.PIN_13, Level::Low);

    // loop {
    //     info!("led on!");
    //     led.set_high();
    //     Timer::after_secs(1).await;

    //     info!("led off!");
    //     led.set_low();
    //     Timer::after_secs(1).await;
    // }
    loop {}
}

// use panic_probe as _;

// #[link_section = ".boot2"]
// #[used]
// pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

// #[rp2040_hal::entry]
// fn entry() -> ! {
//     crate::core::main::main();

//     loop {}
// }