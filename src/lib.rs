mod register_access;
pub mod apds9151;
pub mod types;
pub struct Apds9151<I2C> {
    i2c: I2C,
    address: u8,
}


