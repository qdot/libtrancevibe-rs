use libusb;
use std::time::Duration;

const VID: u16 = 0xb49;
const PID: u16 = 0x64f;

pub struct TranceVibeDeviceInfo {
    context: libusb::Context,
    bus_number: u8,
    address: u8,
}

pub struct TranceVibeDevice<'a> {
    device: libusb::DeviceHandle<'a>,
}

impl<'a> TranceVibeDevice<'a> {
    pub fn new(device: libusb::DeviceHandle<'a>) -> TranceVibeDevice<'a> {
        TranceVibeDevice { device: device }
    }

    pub fn set(&mut self, speed: u8) -> libusb::Result<usize> {
        let timeout = Duration::from_secs(1);
        let bytes: [u8; 0] = [];
        return self.device.write_control(libusb::request_type(libusb::Direction::Out,
                                                              libusb::RequestType::Vendor,
                                                              libusb::Recipient::Interface),
                                         1,
                                         speed as u16,
                                         0,
                                         &bytes,
                                         timeout);
    }
}

impl TranceVibeDeviceInfo {
    pub fn new(bus_number: u8, address: u8) -> TranceVibeDeviceInfo {
        TranceVibeDeviceInfo {
            context: match libusb::Context::new() {
                Ok(c) => c,
                Err(_) => panic!("Can't create context!"),
            },
            bus_number: bus_number,
            address: address,
        }
    }

    pub fn open(&mut self) -> Option<TranceVibeDevice> {
        let devices = match self.context.devices() {
            Ok(d) => d,
            Err(_) => return None,
        };

        for mut dev in devices.iter() {
            let device_desc = match dev.device_descriptor() {
                Ok(d) => d,
                Err(_) => continue,
            };
            if device_desc.vendor_id() != VID || device_desc.product_id() != PID ||
               dev.bus_number() != self.bus_number || dev.address() != self.address {
                continue;
            }
            match dev.open() {
                Ok(h) => return Some(TranceVibeDevice::new(h)),
                Err(_) => panic!("Can't open device!"),
            };
        }
        None
    }
}

pub fn get_devices() -> libusb::Result<Vec<TranceVibeDeviceInfo>> {
    let mut context = match libusb::Context::new() {
        Ok(c) => c,
        Err(_) => panic!("Cannot initialize libusb context!"),
    };
    let mut devices = Vec::<TranceVibeDeviceInfo>::new();
    for mut device in try!(context.devices()).iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };
        if device_desc.vendor_id() != VID || device_desc.product_id() != PID {
            continue;
        }
        devices.push(TranceVibeDeviceInfo::new(device.bus_number(), device.address()));
    }
    Ok(devices)
}
