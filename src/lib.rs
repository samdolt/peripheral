#![no_std]

pub trait DigitalIn {
    fn read(&self) -> Option<bool>;
    fn set_input_mode(&mut self);
    fn set_to_input(&mut self) -> Option<()>;
}

pub trait DigitalOut {
    fn write(&mut self, value: bool) -> Option<bool>;
    fn set_output_mode(&mut self);
    fn set_low(&mut self);
    fn set_high(&mut self);
    fn set_to_output(&mut self) -> Option<()>;
}

pub trait DigitalIO: DigitalIn + DigitalOut {
    fn set_direction(&mut self, value: bool) -> Option<()> {
        if value {
            self.set_to_input()
        } else {
            self.set_to_output()
        }

    }
}

pub trait AnalogIn{}
pub trait AnalogOut{}
pub trait AnalogIO{}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
