use super::vec::UVec2;

#[derive(Debug)]
pub enum Error {
    OutOfBounds { p: UVec2, bounds: UVec2 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::OutOfBounds { p, bounds } => write!(
                fmt,
                "Attempted to access point {p:?}, which is out of bounds {bounds:?}"
            ),
        }
    }
}
