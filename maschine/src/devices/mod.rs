mod maschine_mikro_mk2;
mod maschine_mk2;

pub use maschine_mikro_mk2::MaschineMikroMk2;
pub use maschine_mk2::MaschineMk2;

use crate::{events::Button, Canvas, Colour, Error, EventTask};

///
/// Common device behaviours
///
pub trait Device: EventTask {
    type Pixel: Clone;

    ///
    /// Instantiate a new Device
    ///
    /// **Arguments**
    /// - device - the HID device to control
    fn new() -> Result<Self, Error>
    where
        Self: Sized;

    ///
    /// Set the State of a Button LED
    ///
    /// **Arguments**
    /// - button - Button associated with a LED
    /// - colour - Colour to apply
    fn set_button_led(&mut self, button: Button, colour: Colour);

    ///
    /// Set the State of a Pad LED
    ///
    /// **Arguments**
    /// - pad - Pad number
    /// - colour - Colour to apply
    fn set_pad_led(&mut self, pad: u8, colour: Colour);

    ///
    /// Get a specific display of the Device
    ///
    /// **Arguments**
    /// - display_idx - Display index
    fn get_display(&mut self, display_idx: u8) -> Result<Box<&mut dyn Canvas<Self::Pixel>>, Error>;
}
