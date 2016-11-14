#[derive(Debug)] 
pub enum Error {
    Unavailable,
    Invalid,
    Unexpected,
    IOError,
}

use std::error;
use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Descr"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[cfg(feature = "use_std")]
pub type Result<T> = ::std::result::Result<T, Error>;

#[cfg(not(feature = "use_std"))]
pub type Result<T> = ::core::result::Result<T, Error>;

pub mod digital_io;

pub mod prelude {
    pub use digital_io::*;
}

pub trait AnalogIn {}
pub trait AnalogOut {}
pub trait AnalogIO {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
