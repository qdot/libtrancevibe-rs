extern crate libtrancevibe;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut devices = libtrancevibe::get_devices().expect("Cannot enumerate devices!");

    println!("Number of devices connected: {}", devices.len());
    if devices.len() == 0 {
        return;
    }
    let mut device = devices.pop().unwrap();
    let mut dev = device.open().expect("Can't open device!");
    dev.set(255).expect("Error setting speed!");
    sleep(Duration::from_secs(1));
    dev.set(0).expect("Error setting speed!");
}
