pub enum MessageType {
    Binary,
    Text,
}

pub enum Input {
    USB
}

pub enum Output {
    Display
}

pub enum ControlUnit {
    Controller
}

pub enum Address{
    I(Input),
    O(Output),
    C(ControlUnit)
}

pub trait MessageData {
    fn read() -> [u8; 64];
}

pub struct Message {
    from: Address,
    to: Address,
    message_type: MessageType,
    data: [u8; 64],
}
