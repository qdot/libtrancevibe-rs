extern crate libtrancevibe;

fn main() {
    let mut tv = libtrancevibe::TranceVibe::new();
    match tv.count() {
        Ok(c) => println!("Number of Trance Vibrators Available: {}", c),
        Err(_) => println!("Cannot count devices!")
    }
}
