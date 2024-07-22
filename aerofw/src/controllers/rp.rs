use cortex_m::prelude::_embedded_hal_blocking_spi_Transfer;
use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, gpio::{Level, Output}, i2c::{Blocking, Config, I2c}, peripherals::{I2C0, PIN_13, PIN_25, PIN_5, SPI0, USB}, usb::{Driver, InterruptHandler}};
use embassy_time::Timer;
use embassy_usb::{class::cdc_acm::{CdcAcmClass, State}, UsbDevice};
use static_cell::StaticCell;
use embassy_rp::spi;

use crate::{constants, controllers::controller::{Controller, PinState}};
use crate::amelia::error::Error;

pub struct RP<'a> {
    status_led: Output<'a, PIN_25>,
    i2c0: I2c<'a, I2C0, Blocking>,
    class: CdcAcmClass<'static, Driver<'static, USB>>,
    spi: spi::Spi<'static, SPI0, spi::Async>,
    cs: Output<'a, PIN_5>,
}

static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

impl<'a> RP<'a> {
    pub fn new(spawner: Spawner) -> Self {

        bind_interrupts!(struct Irqs {
            USBCTRL_IRQ => InterruptHandler<USB>;
        });
        
        let p = embassy_rp::init(Default::default());
        let driver = Driver::new(p.USB, Irqs);


        let status_led = Output::new(p.PIN_25, Level::Low);
        let i2c0 = embassy_rp::i2c::I2c::new_blocking(
            p.I2C0,
            p.PIN_1, 
            p.PIN_0, 
            Config::default(),
        );

        let miso = p.PIN_4;
        let mosi = p.PIN_3;
        let clk  = p.PIN_2;
        let cs  = Output::new(p.PIN_5, Level::High);
        let mut config = spi::Config::default();
        config.frequency = 1_000_000; // 5MHz
        let spi = spi::Spi::new(
            p.SPI0, 
            clk, 
            mosi, 
            miso,
            p.DMA_CH0,
            p.DMA_CH1,
            config,
        );

        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some(constants::usb::MANUFACTURER);
        config.product = Some(constants::usb::PRODUCT);
        config.serial_number = Some(constants::usb::SERIAL_NUMBER);
        config.max_power = constants::usb::MAX_POWER;
        config.max_packet_size_0 = constants::usb::MAX_PACKET_SIZE;
        
        config.device_class = constants::usb::DEVICE_CLASS;
        config.device_sub_class = constants::usb::DEVICE_SUB_CLASS;
        config.device_protocol = constants::usb::DEVICE_PROTOCOL;
        config.composite_with_iads = constants::usb::COMPOSITE_WITH_IADS;

        
        let mut builder = embassy_usb::Builder::new(
            driver,
            config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [], // no msos descriptors
            CONTROL_BUF.init([0; 64]),
        );

        static STATE: StaticCell<State> = StaticCell::new();
        let state = STATE.init(State::new());
        
        let class = CdcAcmClass::new(&mut builder, state, 64);
    
        // Build the builder.
        let usb = builder.build();

        unwrap!(spawner.spawn(usb_task(usb)));

        Self {
            status_led,
            i2c0,
            class,
            spi,
            cs,
        }
    }
}

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, Driver<'static, USB>>) -> ! {
    usb.run().await
}

impl From<PinState> for Level {
    fn from(pin_state: PinState) -> Self {
        match pin_state {
            PinState::High => Level::High,
            PinState::Low => Level::Low,
        }
    }
}


impl<'a> Controller for RP<'a> {
    fn set_status_led(&mut self, pin_state: PinState) {
        match pin_state {
            PinState::High => {
                self.status_led.set_high();
            }
            PinState::Low => {
                self.status_led.set_low();
            }
        }
    }

    fn write_to_i2c(&mut self, addr: u8, data: &[u8]) -> Result<(), Error> {
        self.i2c0.blocking_write(addr, data).map_err(|_| (Error::Io))
    }

    async fn write_to_usb(&mut self, data: [u8; constants::usb::converted::MAX_PACKET_SIZE_USIZE]) -> Result<(), Error> {
        self.class.write_packet(&data).await?;
        Ok(())
    }

    async fn read_from_usb(&mut self) -> Result<[u8; constants::usb::converted::MAX_PACKET_SIZE_USIZE], Error> {
        let mut usb_buffer = [0_u8; constants::usb::converted::MAX_PACKET_SIZE_USIZE];
        self.class.read_packet(&mut usb_buffer).await?;
        Ok(usb_buffer)
    }

    async fn spi_read(&mut self, tx_buf: &[u8], rx_buf: &mut[u8]) -> Result<(), Error> {
        self.cs.set_low();
        Timer::after_micros(1).await;
        self.spi.transfer(rx_buf, tx_buf).await.map_err(|_| Error::Io)?;
        self.cs.set_high();
    
        Ok(())
    }
}


