pub enum PinState {
    High,
    Low,
}

pub trait Controller {
    fn manage_onboard_led(&mut self, pin_state: PinState);
    fn write_to_i2c(&mut self, addr: u8, data: &[u8]) -> Result<(), ()>;
}
