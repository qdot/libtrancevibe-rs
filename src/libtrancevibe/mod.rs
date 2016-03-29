use libusb;
use std::time::Duration;

const VID: u16 = 0xb49;
const PID: u16 = 0x64f;

pub struct TranceVibe<'a> {
    context: libusb::Context,
    device: Option<libusb::Device<'a>>,
    handle: Option<libusb::DeviceHandle<'a>>,
    bus_number: u8,
    address: u8,
    opened: bool,
}

impl<'a> TranceVibe<'a> {
    pub fn new(bus_number: u8, address: u8) -> TranceVibe<'a> {
        TranceVibe {
            context: libusb::Context::new().unwrap(),
            device: None,
            bus_number: bus_number,
            address: address,
            handle: None,
            opened: false,
        }
    }

    pub fn open(&'a mut self) -> libusb::Result<()> {
        for mut dev in try!(self.context.devices()).iter() {
            let device_desc = match dev.device_descriptor() {
                Ok(d) => d,
                Err(_) => continue
            };
            if device_desc.vendor_id() != VID ||
                device_desc.product_id() != PID ||
                dev.bus_number() != self.bus_number ||
                dev.address() != self.address {
                    continue;
                }
            self.device = Some(dev);
            break;
        }
        return Ok(());
    }

    pub fn close(&'a mut self) -> libusb::Result<()> {
        return Ok(());
    }

    pub fn set(&'a mut self, speed : u8) -> libusb::Result<()> {
        return Ok(());
    }
}

pub fn get_devices<'a>() -> libusb::Result<Vec<TranceVibe<'a>>> {
    let mut context = match libusb::Context::new() {
        Ok(c) => c,
        Err(e) => panic!("Cannot initialize libusb context!")
    };
    let timeout = Duration::from_secs(1);
    let mut devices = Vec::<TranceVibe>::new();
    for mut device in try!(context.devices()).iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue
        };
        if device_desc.vendor_id() != VID ||
            device_desc.product_id() != PID {
                continue;
            }
        devices.push(TranceVibe::new(device.bus_number(), device.address()));
    }
    return Ok(devices);
}
