use libusb;
use std::time::Duration;

const VID: u16 = 0xb49;
const PID: u16 = 0x64f;

struct TranceVibeDevice<'a> {
    handle: libusb::DeviceHandle<'a>,
    language: libusb::Language,
    timeout: Duration
}

pub struct TranceVibe<'a> {
    context: libusb::Context,
    device: Option<TranceVibeDevice<'a>>,
    inbound: Option<libusb::TransferType>,
    outbound: Option<libusb::TransferType>,
    opened: bool,
}

impl<'a> TranceVibe<'a> {
    pub fn new() -> TranceVibe<'a> {
        TranceVibe {
            context: match libusb::Context::new() {
                Ok(c) => c,
                Err(e) => panic!("Cannot initialize libusb context!")
            },
            device: None,
            inbound: None,
            outbound: None,
            opened: false,
        }
    }

    pub fn count(&mut self) -> libusb::Result<u8> {
        let timeout = Duration::from_secs(1);
        let mut count = 0u8;
        for mut device in try!(self.context.devices()).iter() {
            let device_desc = match device.device_descriptor() {
                Ok(d) => d,
                Err(_) => continue
            };
            if device_desc.vendor_id() != VID {
                continue;
            }
            if device_desc.product_id() != PID {
                continue;
            }
            count += 1;
        }
        return Ok(count);
    }

    pub fn open(&mut self, index: u8) -> libusb::Result<()> {
        return Ok(());
    }

    pub fn close(&mut self) -> libusb::Result<()> {
        return Ok(());
    }

    pub fn set(&self, speed : u8) -> libusb::Result<()> {
        return Ok(());
    }
}
