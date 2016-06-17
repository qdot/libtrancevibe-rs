use libusb;
use std::time::Duration;

/// USB VID and PID for module. All TranceVibes (originals, Drmn Trancevibe,
/// Harnett USBMC-01) should use this pair for identification
const VID: u16 = 0xb49;
const PID: u16 = 0x64f;

/// Struct that holds USB context for a device, as well as its bus/address. Used
/// to statically identify a TranceVibrator on a system (since multiple devices
/// will have same VID/PID and no unique serial)
pub struct TranceVibeDeviceInfo {
    /// libusb context used to fetch device.
    context: libusb::Context,
    /// USB Bus Number of the Trance Vibrator device
    pub bus_number: u8,
    /// USB Address Number of the Trance Vibrator device
    pub address: u8,
}

/// Struct representing an open, usable device.
///
/// Given a successfully opened libusb::DeviceHandle, this struct manages the
/// handle lifetime and gives write access for setting the running speed of
/// the TranceVibrator.
pub struct TranceVibeDevice<'a> {
    device: libusb::DeviceHandle<'a>,
}

impl<'a> TranceVibeDevice<'a> {
    /// Creates a new TranceVibrator device, using an opened libusb::DeviceHandle
    fn new(device: libusb::DeviceHandle<'a>) -> TranceVibeDevice<'a> {
        TranceVibeDevice { device: device }
    }

    /// Set the TranceVibrator to a certain running speed.
    ///
    /// Allows the user to set the TranceVibrator speed to somewhere between 0
    /// (off) and 255 (full power). Command may block for up to one second.
    ///
    /// Returns a libusb::Result containing either the amount of bytes written
    /// (should always be 1), or an error.
    pub fn set(&mut self, speed: u8) -> libusb::Result<usize> {
        let timeout = Duration::from_secs(1);
        // TODO If we aren't writing any bytes, do we really need this?
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
    /// Creates a new TranceVibrator Device Info struct
    ///
    /// Allows the user to try and open the device contained at the bus/address
    /// given.
    fn new(bus_number: u8, address: u8) -> TranceVibeDeviceInfo {
        TranceVibeDeviceInfo {
            context: match libusb::Context::new() {
                Ok(c) => c,
                Err(_) => panic!("Can't create context!"),
            },
            bus_number: bus_number,
            address: address,
        }
    }

    /// Tries to open the device represented by this struct
    ///
    /// Tries to open and return device represented by the address data in this
    /// struct. Returns a Result with an opened TranceVibeDevice.
    ///
    /// # Error
    ///
    /// Returns libusb specific errors if any arise. If the device represented
    /// by this struct no longer exists, returns libusb::Error::NoDevice.
    pub fn open(&mut self) -> libusb::Result<TranceVibeDevice> {
        let devices = match self.context.devices() {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        for mut dev in devices.iter() {
            let device_desc = match dev.device_descriptor() {
                Ok(d) => d,
                Err(e) => return Err(e),
            };
            if device_desc.vendor_id() != VID || device_desc.product_id() != PID ||
               dev.bus_number() != self.bus_number || dev.address() != self.address {
                continue;
            }
            match dev.open() {
                Ok(h) => return Ok(TranceVibeDevice::new(h)),
                Err(e) => return Err(e),
            };
        }
        return Err(libusb::Error::NoDevice);
    }
}

/// Returns vector of all current available connected TranceVibrators.
///
/// Polls the system and returns TranceVibeDeviceInfo structs for all connected
/// TranceVibrators. These structs can then have TranceVibeDevice::open() called
/// on them to actually open the device.
///
/// # Error
///
/// Returns libusb specific errors if any arise.
pub fn get_devices() -> libusb::Result<Vec<TranceVibeDeviceInfo>> {
    let mut context = match libusb::Context::new() {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    let mut devices = Vec::<TranceVibeDeviceInfo>::new();
    let connected_devices = match context.devices() {
        Ok(d) => d,
        Err(e) => return Err(e),
    };

    for mut device in connected_devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(e) => return Err(e),
        };
        if device_desc.vendor_id() != VID || device_desc.product_id() != PID {
            continue;
        }
        devices.push(TranceVibeDeviceInfo::new(device.bus_number(), device.address()));
    }
    Ok(devices)
}
