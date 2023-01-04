mod maschine_mikro_mk2;
mod maschine_mk2;

use crate::Color;
pub use maschine_mikro_mk2::MaschineMikroMk2;
pub use maschine_mk2::MaschineMk2;
use raqote::DrawTarget;

use crate::{events::Button, Error, EventTask};

///
/// Common device behaviours
///
pub trait Device: EventTask {
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
    /// - color - Color to apply
    fn set_button_led(&mut self, button: Button, color: Color);

    ///
    /// Set the State of a Pad LED
    ///
    /// **Arguments**
    /// - pad - Pad number
    /// - color - Color to apply
    fn set_pad_led(&mut self, pad: u8, color: Color);

    ///
    /// Get a specific display of the Device
    ///
    /// **Arguments**
    /// - display_idx - Display index
    fn get_display(&mut self, display_idx: u8) -> Result<&mut DrawTarget, Error>;
}
