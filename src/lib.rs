//! This crate provides access and control for Rez TranceVibrator devices

extern crate libusb;

// Only provide outside access via get_devices. Everything should be generated
// from the output there.
pub use ::libtrancevibe::{get_devices};

mod libtrancevibe;s
