#![no_std]
#![no_main]

mod core;
use panic_halt as _;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rp2040_hal::entry]
fn entry() -> ! {
    crate::core::main::main();

    loop {}
}