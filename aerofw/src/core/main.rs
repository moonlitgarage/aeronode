use crate::core::controller::PinState;
use crate::core::controllers::rp::RP;

use super::controller::Controller;

pub fn main() {
    let mut rp = RP::new();

    loop {
        rp.manage_onboard_led(PinState::High);
        rp.delay_ms(500);
        rp.manage_onboard_led(PinState::Low);
        rp.delay_ms(100);
    }
}
