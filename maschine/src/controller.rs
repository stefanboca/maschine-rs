use crate::colour::Colour;
use crate::events::{Button, EventTask};
use crate::{Canvas, Error};

///
/// Common controller behaviours
///
pub trait Device<P: Clone>: EventTask {
    ///
    /// Instantiate a new Controller
    ///
    /// **Arguments**
    /// - device - the HID device to control
    fn new() -> Result<Self, Error>
    where
        Self: Sized;

    ///
    /// Set the State of an Button LED
    ///
    /// **Arguments**
    /// - button - Button associated with a LED
    /// - colour - Colour to apply
    fn set_button_led(&mut self, button: Button, colour: Colour);

    ///
    /// Set the State of an Pad LED
    ///
    /// **Arguments**
    /// - pad - Pad number
    /// - colour - Colour to apply
    fn set_pad_led(&mut self, pad: u8, colour: Colour);

    ///
    /// Get a specific display of the controller
    ///
    /// **Arguments**
    /// - display_idx - Display index
    fn get_display(&mut self, display_idx: u8) -> Result<Box<&mut dyn Canvas<P>>, Error>;
}
