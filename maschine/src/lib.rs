use devices::MaschineMk2;
use hidapi::HidApi;

mod colour;
mod controller;
pub mod devices;
mod display;
mod error;
mod events;
pub mod fonts;

pub use colour::Colour;
pub use controller::Controller;
pub use display::{Canvas, Font, Pixel, ScrollDirection};
pub use error::Error;
pub use events::{Event, EventContext, EventHandler, EventTask};

pub fn get_device(hid_api: &HidApi) -> Result<devices::MaschineMk2, error::Error> {
    let device = hid_api
        .open(devices::MaschineMk2::VENDOR_ID, MaschineMk2::PRODUCT_ID)
        .expect("Cannot open device");

    Ok(devices::MaschineMk2::new(device))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
