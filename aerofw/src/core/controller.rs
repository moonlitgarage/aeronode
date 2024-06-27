pub enum PinState {
    High,
    Low,
}

pub trait Controller {
    fn manage_onboard_led(&mut self, pin_state: PinState);
}
