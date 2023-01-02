use hidapi::HidError;

///
/// Common controller errors
///
#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    HidAPI(HidError),

    /// Input buffer does not container the expected amount of data.
    InvalidReport,

    /// Unexpected control returned from hardware device
    UnknownControl,

    /// Tried to write to non-existant display
    InvalidDisplay
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::HidAPI(e) => e.fmt(fmt), // Pass on to HIDAPI interface
            Error::InvalidReport => {
                write!(fmt, "Report is either two small or not parsable")
            }
            Error::UnknownControl => {
                write!(fmt, "Unexpected control returned from hardware device")
            }
            Error::InvalidDisplay => {
                write!(fmt, "Attempted to write to invalid display")
            }
        }
    }
}

impl From<HidError> for Error {
    fn from(err: HidError) -> Error {
        Error::HidAPI(err)
    }
}
