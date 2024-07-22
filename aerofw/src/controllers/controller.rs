use embassy_rp::peripherals::PIN_13;

use crate::{amelia::error::Error, constants};

pub enum PinState {
    High,
    Low,
}

pub trait Controller {
    fn set_status_led(&mut self, pin_state: PinState);
    fn write_to_i2c(&mut self, addr: u8, data: &[u8]) -> Result<(), Error>;
    async fn write_to_usb(&mut self, data: [u8; constants::usb::converted::MAX_PACKET_SIZE_USIZE]) -> Result<(), Error>;
    async fn read_from_usb(&mut self) -> Result<[u8; constants::usb::converted::MAX_PACKET_SIZE_USIZE], Error>;
    async fn spi_read(&mut self, tx_buf: &[u8], rx_buf: &mut[u8]) -> Result<(), Error>;
}
