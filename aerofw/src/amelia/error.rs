use embassy_usb::driver::EndpointError;

pub enum Error {
    Io,
    BufferOverflow,
    DeviceNotAvailable,
}

impl From<EndpointError> for Error {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => Error::BufferOverflow,
            EndpointError::Disabled => Error::DeviceNotAvailable,
        }
    }
}
