use core::result::Result;

use crate::{
    register_access::apds9151::{DEV_ADDR, Register},
    Apds9151,
    types::*,
};
use embedded_hal::blocking::i2c;

impl<I2C, E> Apds9151<I2C>
where I2C: i2c::Read<Error = E> + i2c::Write<Error = E> {
    /// Creates a new instance of the device
    pub fn new_apda9151(i2c: I2C) -> Self {
        Apds9151 {
            i2c,
            address: DEV_ADDR,
        }
    }

    /// Desstroy driver instance, returning the I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write the default initiliation data to the device
    pub fn initialize(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::MAIN_CTRL, MainControl::RGBMode as u8 | MainControl::LightSensorEnable as u8 | MainControl::ProximitySensorEnable as u8)?;
        self.write_register(Register::PS_MEAS_RATE, ProximitySensorResolution::Resolution11Bits as u8 | ProximitySensorRate::Rate100Ms as u8)?;
        self.write_register(Register::PS_PULSES, 8_u8)?;

        Ok(())
    }

    pub fn get_ir(&mut self) -> Result<u32, Error<E>> {
        let mut buffer: [u8; 3] = [0; 3];
        self.read_data(Register::LS_DATA_IR, &mut buffer)?;
        let c = to_20_bit(buffer);
        Ok(c)
    }

    /// Get the Red color channel as a u32
    pub fn get_red(&mut self) -> Result<u32, Error<E>> {
        let mut buffer: [u8; 3] = [0; 3];
        self.read_data(Register::LS_DATA_RED, &mut buffer)?;
        let c = to_20_bit(buffer);
        Ok(c)
    }

    /// Get the Blue color channel as a u32
    pub fn get_blue(&mut self) -> Result<u32, Error<E>> {
        let mut buffer: [u8; 3] = [0; 3];
        self.read_data(Register::LS_DATA_BLUE, &mut buffer)?;
        let c = to_20_bit(buffer);
        Ok(c)
    }

    /// Get the Green color channel as a u32
    pub fn get_green(&mut self) -> Result<u32, Error<E>> {
        let mut buffer: [u8; 3] = [0; 3];
        self.read_data(Register::LS_DATA_GREEN, &mut buffer)?;
        let c = to_20_bit(buffer);
        Ok(c)
    }

    /// Gets the Proximity sensor data as u16
    pub fn get_proximity(&mut self) -> Result<u16, Error<E>> {
        let mut buffer: [u8; 2] = [0; 2];
        self.read_data(Register::PS_DATA, &mut buffer)?;
        let d = to_11_bit(buffer);
        Ok(d)
    }

    /// Configures Proximity Sensor LED
    pub fn config_proximity_led(&mut self, freq: LEDPulseFreq, curr: LEDCurrent, pulses: u8) -> Result<(), Error<E>> {
        self.write_register(Register::PS_LED, freq as u8 | curr as u8)?;
        self.write_register(Register::PS_PULSES, pulses)?;
        Ok(())
    }

    /// Configures Proximity Sensor
    pub fn config_proximity(&mut self, res: ProximitySensorResolution, rate: ProximitySensorResolution) -> Result<(), Error<E>> {
        self.write_register(Register::PS_MEAS_RATE, res as u8 | rate as u8)?;
        Ok(())
    }

    /// Configures Color Sensor
    pub fn config_color(&mut self, res: ColorResolution, rate: ColorMeaseurementRate) -> Result<(), Error<E>> {
        self.write_register(Register::LS_MEAS_RATE, res as u8 | rate as u8)?;
        Ok(())
    }

    /// Configure Color Sensor Gain
    pub fn set_gain(&mut self, gain: GainFactor) -> Result<(), Error<E>> {
        self.write_register(Register::LS_GAIN, gain as u8)?;
        Ok(())
    }

    // pub fn get_colors(&mut self) -> Result<[u32; 3], Error<E>> {
    //     // let mut data: [u8; 1] = [Register::LS_DATA_GREEN_0];
    //     // self.write_data(&mut data)?;

    //     let mut buffer: [u8; 9] = [0; 9];
    //     match self.read_data(Register::LS_DATA_GREEN_0, &mut buffer) {
    //         Ok(()) => (),
    //         // Err(Error::I2C(_)) => (),
    //         Err(e) => return Err(e),
    //     };
    //     let colors: [u32; 3] = [
    //         to_20_bit(&buffer[0..2]),
    //         to_20_bit(&buffer[3..5]),
    //         to_20_bit(&buffer[6..8]),
    //     ];
    //     Ok(colors)
    // }
}

fn to_20_bit(bytes: [u8; 3]) -> u32 {
    return (u32::from(bytes[0]) | (u32::from(bytes[1]) << 8) | (u32::from(bytes[2]) << 16)) & 0x03FFFF;
}

fn to_11_bit(bytes: [u8; 2]) -> u16 {
    return (u16::from(bytes[0]) | u16::from(bytes[1]) << 8) & 0x7FF
}