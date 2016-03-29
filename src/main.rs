extern crate libtrancevibe;

fn main() {
    let devices = libtrancevibe::get_devices().unwrap();
    println!("Number of trancevibes connected: {}", devices.len());
}
