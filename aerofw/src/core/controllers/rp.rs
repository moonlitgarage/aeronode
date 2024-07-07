use embassy_rp::{gpio::{Level, Output}, i2c::{I2c, Config, Blocking}, peripherals::{I2C0, PIN_13}};

use crate::core::controller::{Controller, PinState};

// pinout used: https://learn.adafruit.com/assets/120082
// todo: abstract pin assignment out for usage amongst different 
// boards of the same family
pub struct RP<'a> {
    internal_led: Output<'a, PIN_13>,
    i2c0: I2c<'a, I2C0, Blocking>,
}

impl RP<'_> {
    pub fn new() -> Self {
        let p = embassy_rp::init(Default::default());

        let internal_led = Output::new(p.PIN_13, Level::Low);
        let sda = p.PIN_0;
        let scl = p.PIN_1;

        let i2c0 = embassy_rp::i2c::I2c::new_blocking(
            p.I2C0,
            scl, 
            sda, 
            Config::default(),
        );

        Self {
            internal_led,
            i2c0,
        }
    }
}

impl From<PinState> for Level {
    fn from(pin_state: PinState) -> Self {
        match pin_state {
            PinState::High => Level::High,
            PinState::Low => Level::Low,
        }
    }
}


impl Controller for RP<'_> {
    fn manage_onboard_led(&mut self, pin_state: PinState) {
        match pin_state {
            PinState::High => {
                self.internal_led.set_high();
            }
            PinState::Low => {
                self.internal_led.set_low();
            }
        }
    }

    fn write_to_i2c(&mut self, addr: u8, data: &[u8]) -> Result<(), ()> {
        self.i2c0.blocking_write(addr, data).map_err(|_| ())
    }
}
