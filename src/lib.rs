#![cfg_attr(not(feature = "use_std"), no_std)]

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

pub enum InputMode {
    Default,
    Floating,
    PulledUp,
    PulledDown,
    Unknow,
}

pub enum OutputMode {
    Default,
    PushPull,
    OpenDrain,
    OpenSource,
    Unknow,
}

/// Generic Digital Input trait
pub trait DigitalIn {
    /// Read the digital Input
    ///
    /// # Error
    ///
    /// On some system, reading a pin can fail (Eg. on Linux)
    fn read(&self) -> Result<bool>;
    fn set_input_mode(&mut self) -> Result<()>;
    fn set_to_input(&mut self) -> Result<()>;
}

pub trait DigitalOut {
    fn write(&mut self, value: bool) -> Result<bool>;
    fn set_output_mode(&mut self) -> Result<()>;
    fn set_low(&mut self) -> Result<()>;
    fn set_high(&mut self) -> Result<()>;
    fn set_to_output(&mut self) -> Result<()>;
}

pub trait DigitalIO: DigitalIn + DigitalOut {
    fn set_direction(&mut self, value: bool) -> Result<()> {
        if value {
            self.set_to_input()
        } else {
            self.set_to_output()
        }
    }
}

pub trait AnalogIn {}
pub trait AnalogOut {}
pub trait AnalogIO {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
