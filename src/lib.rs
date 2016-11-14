 
pub enum Error {
    Unavailable,
    Invalid,
    Unexpected,
    IOError,
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
