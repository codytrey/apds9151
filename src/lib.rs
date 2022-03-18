mod register_access;
mod apds9151;
pub mod types;

pub use apds9151::*;
pub struct Apds9151<I2C> {
    i2c: I2C,
    address: u8,
}


