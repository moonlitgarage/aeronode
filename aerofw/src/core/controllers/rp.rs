use rp2040_hal as rp;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use crate::core::controller::{Controller, PinState};

const FREQ_CRYSTAL: u32 = 12_000_000; // 12 MHz

pub struct RP {
    timer: rp::Timer,
    pin13: rp2040_hal::gpio::Pin<rp2040_hal::gpio::bank0::Gpio13, rp2040_hal::gpio::FunctionSio<rp2040_hal::gpio::SioOutput>, rp2040_hal::gpio::PullDown>
}

enum Pin {
    Gpio13,
}

impl RP {
    pub fn new() -> Self {
        let mut pac = rp::pac::Peripherals::take().unwrap();
        let mut watchdog = rp::Watchdog::new(pac.WATCHDOG);

        let clocks = rp::clocks::init_clocks_and_plls(
            FREQ_CRYSTAL,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .unwrap();

        let timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

        let sio = rp::Sio::new(pac.SIO);
        let pins = rp::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let pin13 = pins.gpio13.into_push_pull_output();

        Self { 
            timer, 
            pin13 
        }
    }
}

impl RP {

    fn get_pin(&mut self, pin: Pin) -> &mut impl OutputPin {
        match pin {
            Pin::Gpio13 => &mut self.pin13,
        }
    }

    fn set_pin_state(&mut self, pin_name: Pin, state: PinState) {
        let pin = self.get_pin(pin_name);
        match state {
            PinState::High => {
                pin.set_high().unwrap();
            }
            PinState::Low => {
                pin.set_low().unwrap();
            }
        }
    }

    pub fn delay_ms(&mut self, ms: u32) {
        self.timer.delay_ms(ms);
    }
}

impl Controller for RP {
    fn manage_onboard_led(&mut self, pin_state: PinState) {
        self.set_pin_state(Pin::Gpio13, pin_state)
    }
}
