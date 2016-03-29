use libusb;
use std::time::Duration;

const VID: u16 = 0xb49;
const PID: u16 = 0x64f;

pub struct TranceVibe<'a> {
    context: libusb::Context,
    handle: Option<libusb::DeviceHandle<'a>>,
    bus_number: u8,
    address: u8
}

impl<'a> TranceVibe<'a> {
    pub fn new(bus_number: u8, address: u8) -> TranceVibe<'a> {
        TranceVibe {
            context: match libusb::Context::new() {
                Ok(c) => c,
                Err(_) => panic!("Can't create context!")
            },
            bus_number: bus_number,
            address: address,
            handle: None
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
            self.handle = match dev.open() {
                Ok(h) => Some(h),
                Err(_) => panic!("Can't open device!")
            };
            break;
        }
        Ok(())
    }

    pub fn set(&mut self, speed : u8) -> libusb::Result<()> {
        Ok(())
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
    Ok(devices)
}
