use super::Error;
use super::Result;

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
    fn set_input_mode(&mut self, mode: InputMode) -> Result<()>{
        match mode {
            InputMode::Default => Ok(()),
            InputMode::Unknow => Err(Error::Unexpected),
            _ => Err(Error::Unavailable)
        }
    }

    fn get_input_mode(&self) -> InputMode {
        InputMode::Unknow
    }
    fn set_to_input(&mut self) -> Result<()>;
}

pub trait DigitalOut {
    fn write(&mut self, value: bool) -> Result<()>{
        match value {
            true => self.set_high(),
            false => self.set_low(),
        }
    }
    fn set_output_mode(&mut self, mode: OutputMode) -> Result<()> {
        match mode {
            OutputMode::Default => Ok(()),
            OutputMode::Unknow => Err(Error::Unexpected),
            _ => Err(Error::Unavailable),
        }
    }

    fn get_output_mode(&self) -> OutputMode {
        OutputMode::Unknow
    }
    
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
