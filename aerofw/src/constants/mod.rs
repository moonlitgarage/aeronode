pub mod usb {
    pub const MANUFACTURER: &str = "koerolabs";
    pub const PRODUCT: &str = "aeronode";
    pub const SERIAL_NUMBER: &str = "1";
    pub const MAX_POWER: u16 = 100;
    pub const MAX_PACKET_SIZE: u8 = 64;
    pub const DEVICE_CLASS: u8 = 0xef;
    pub const DEVICE_SUB_CLASS: u8 = 0x02;
    pub const DEVICE_PROTOCOL: u8 = 0x01;
    pub const COMPOSITE_WITH_IADS: bool = true;

    pub mod converted {
        use super::MAX_PACKET_SIZE;

        pub const MAX_PACKET_SIZE_USIZE: usize = MAX_PACKET_SIZE as usize;
    }
}