extern crate libtrancevibe;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut devices = match libtrancevibe::get_devices() {
        Ok(d) => d,
        Err(_) => panic!("Cannot enumerate devices!")
    };
    println!("Number of devices connected: {}", devices.len());
    if devices.len() == 0 {
        return;
    }
    let mut device = devices.pop().unwrap();
    let mut dev;
    match device.open() {
        Some(d) => dev = *d,
        None => panic!("Can't open device!")
    };

    dev.set(255);
    sleep(Duration::from_secs(1));
    dev.set(0);
}
