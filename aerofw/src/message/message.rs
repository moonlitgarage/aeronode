pub enum MessageType {
    Binary,
    Text,
}

pub struct Message {
    from: u8,
    to: u8,
    message_type: MessageType,
    data: [u8],
}