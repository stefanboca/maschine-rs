#![feature(slice_as_chunks)]

pub mod devices;
mod error;
mod events;

pub use devices::{Device, MaschineMikroMk2, MaschineMk2};
pub use error::Error;
pub use events::{Event, EventContext, EventTask};
pub use raqote::Color;

pub fn get_device() -> Result<Box<dyn Device>, Error> {
    if let Ok(device) = MaschineMikroMk2::new() {
        Ok(Box::new(device))
    } else if let Ok(device) = MaschineMk2::new() {
        Ok(Box::new(device))
    } else {
        Err(Error::NoDevices)
    }
}
