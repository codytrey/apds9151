use crate::types::Error;
// use crate::apds9151::crc;
use embedded_hal::blocking::i2c;
// use embedded_hal::blocking::i2c::Transactional;

#[allow(dead_code)]
pub mod apds9151 {
    pub const DEV_ADDR: u8 = 0x52;
    pub struct Register {}
    impl Register {
        pub const MAIN_CTRL: u8 = 0x00;
        pub const PS_LED: u8 = 0x01;
        pub const PS_PULSES: u8 = 0x02;
        pub const PS_MEAS_RATE: u8 = 0x03;
        pub const LS_MEAS_RATE: u8 = 0x04;
        pub const LS_GAIN: u8 = 0x05;
        pub const PART_ID: u8 = 0x06;
        pub const MAIN_STATUS: u8 = 0x07;
        pub const PS_DATA: u8 = 0x08;
        pub const LS_DATA_IR: u8 = 0x0A;
        pub const LS_DATA_GREEN: u8 = 0x0D;
        pub const LS_DATA_BLUE: u8 = 0x10;
        pub const LS_DATA_RED: u8 = 0x13;
        pub const PS_CAN_0: u8 = 0x1F;
        pub const PS_CAN_1: u8 = 0x20;
    }
}

impl<I2C, E> crate::Apds9151<I2C>
where I2C: i2c::Read<Error = E> + i2c::Write<Error = E> {
    pub(crate) fn read_data(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c.write(self.address, &[address])
        .map_err(Error::I2C)?;
        self.i2c.read(self.address, buffer).map_err(Error::I2C).and(Ok(()))
    }

    pub(crate) fn write_register(&mut self, address: u8, data: u8) -> Result<(), Error<E>>{
        self.i2c.write(self.address, &[address, data]).map_err(Error::I2C).and(Ok(()))
    }

    #[allow(dead_code)]
    pub(crate) fn write_data(&mut self, buffer: &mut [u8]) -> Result<(), Error<E>>{
        if buffer.is_empty() {
            self.i2c.write(self.address, &buffer).map_err(Error::I2C)?;
            return Ok(());
        }

        self.i2c.write(self.address, &buffer).map_err(Error::I2C).and(Ok(()))
    }
}