mod colour;
pub mod devices;
mod display;
mod error;
mod events;
pub mod fonts;

pub use colour::Colour;
pub use devices::Device;
pub use devices::{MaschineMikroMk2, MaschineMk2};
pub use display::{Canvas, Font, MonoPixel, ScrollDirection};
pub use error::Error;
pub use events::{Event, EventContext, EventTask};

pub trait GetDevice {
    fn get_device() -> Option<Box<dyn Device<Pixel = Self>>> {
        None
    }
}

impl GetDevice for MonoPixel {
    fn get_device() -> Option<Box<dyn Device<Pixel = Self>>> {
        if let Ok(device) = MaschineMikroMk2::new() {
            return Some(Box::new(device));
        }

        if let Ok(device) = MaschineMk2::new() {
            return Some(Box::new(device));
        }

        None
    }
}

pub fn get_device<P: GetDevice>() -> Result<Box<dyn Device<Pixel = P>>, Error> {
    match P::get_device() {
        Some(device) => Ok(device),
        None => Err(Error::NoDevices),
    }
}
