#![no_std]
use embedded_hal::{digital::PinState, i2c::I2c};

/// Pins address
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Pin {
    IO0 = 0x01,
    IO1 = 0x02,
    IO2 = 0x04,
    IO3 = 0x08,
    IO4 = 0x10,
    IO5 = 0x20,
    IO6 = 0x40,
    IO7 = 0x80,
}

/// Register address
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Register {
    Input = 0x00,
    Output = 0x01,
    Polarity = 0x02,
    Config = 0x03,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    OUTPUT = 0,
    INPUT = 1,
}

impl From<Pin> for u8 {
    fn from(value: Pin) -> Self {
        value as u8
    }
}

impl From<Mode> for bool {
    fn from(value: Mode) -> Self {
        match value {
            Mode::OUTPUT => false,
            Mode::INPUT => true,
        }
    }
}

impl From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        value as u8
    }
}

pub struct Pca9557<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Pca9557<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    // ************** Basic layer manipulation *****************
    /// Return u8 value from register
    fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        let mut buf: [u8; _] = [0u8; 1];
        self.i2c.write_read(self.address, &[reg as u8], &mut buf)?;
        Ok(buf[0])
    }

    /// Write u8 to register
    fn write_register(&mut self, reg: Register, value: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[reg as u8, value])
    }

    fn mask_it(value: u8, mask: u8, op: bool) -> u8 {
        match op {
            true => value | mask,
            false => value & !mask,
        }
    }
    // *************** Public layer ******************************

    /// Set pin mode input/output
    pub fn set_mode(&mut self, pin: Pin, mode: Mode) -> Result<(), E> {
        let val: u8 = self.read_register(Register::Config)?;
        let value: u8 = Self::mask_it(val, pin as u8, mode.into());
        self.write_register(Register::Config, value)
    }

    /// Read pin state
    pub fn read_pin(&mut self, pin: Pin) -> Result<PinState, E> {
        let regres = self.read_register(Register::Input)?;
        // let shift = u8::from(pin).trailing_zeros();
        // let res = (regres >> shift) & 1;
        // if res > 0 {
        if (regres & u8::from(pin)) != 0 {
            Ok(PinState::High)
        } else {
            Ok(PinState::Low)
        }
    }

    /// Write pin state
    pub fn write_pin(&mut self, pin: Pin, state: PinState) -> Result<(), E> {
        let regres = self.read_register(Register::Output)?;
        let value: u8 = Self::mask_it(regres, pin as u8, state.into());
        self.write_register(Register::Output, value)
    }

    /// Read input register
    pub fn read_input(&mut self) -> Result<u8, E> {
        self.read_register(Register::Input)
    }

    /// Read real input without polarity
    pub fn read_input_raw(&mut self) -> Result<u8, E> {
        let input = self.read_register(Register::Input)?;
        let polarity = self.read_register(Register::Polarity)?;
        Ok(input ^ polarity)
    }

    /// Read output register
    pub fn read_output(&mut self) -> Result<u8, E> {
        self.read_register(Register::Output)
    }

    /// Read config register
    pub fn read_config(&mut self) -> Result<u8, E> {
        self.read_register(Register::Config)
    }

    /// Read polarity register
    pub fn read_polarity(&mut self) -> Result<u8, E> {
        self.read_register(Register::Polarity)
    }

    /// Write output register
    pub fn write_output(&mut self, value: u8) -> Result<(), E> {
        self.write_register(Register::Output, value)
    }

    /// Write config register
    pub fn write_config(&mut self, value: u8) -> Result<(), E> {
        self.write_register(Register::Config, value)
    }

    /// Write polarity register
    pub fn write_polarity(&mut self, value: u8) -> Result<(), E> {
        self.write_register(Register::Polarity, value)
    }

    /// Wake up device
    pub fn reset(&mut self) -> Result<(), E> {
        self.write_config(0xff)?;
        self.write_output(0xff)?;
        self.write_polarity(0x00)
    }
}
