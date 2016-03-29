extern crate libtrancevibe;

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
    match device.open() {
        Ok(_) => println!("Device opened"),
        Err(_) => println!("Can't open device!")
    };
}
