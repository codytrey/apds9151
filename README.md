# Rust APDS9151 Digital Proximity and RGB Color I2C Sensor

This is a platform agnostic Rust driver for the APDS9151 Digital Proximity and RGB Color I2C Sensor, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Initialize the device. See `initialize()`
- Get color sensor IR channel data. See `get_ir()`
- Get color sensor red channel data. See `get_red()`
- Get color sensor blue channel data. See `get_blue()`
- Get color sensor green channel data. See `get_green()`
- Get proximity sensor data. See `get_proximity()`
- Configure proximity sensor LED fequency, current, and number of pulses. See `config_proximity_led()`
- Configure proximity sensor measurement rate and resolution. See `config_proximity()`
- Configure color sensor measurement rate and resolution. See `config_color()`
- Configre color sensor gain. See `set_gain()`

## The device

This device offers both RGB+IR color sensing controlled via I2C.

An example of the APDS9151 used in a product is the Rev Robotics v3 color sensor. [link](https://www.revrobotics.com/rev-31-1557/)

Datasheet: [APDS9151](https://docs.broadcom.com/doc/APDS-9151-DS)

## Usage

To use the driver, import the crate and the `embedded_hal` i2c interface for your platform.

The below example uses `stm32f1xx_hal` and gets the colors in a loop.
```rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::{rtt_init_print, rprintln};
use stm32f1xx_hal::{
    pac,
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
};
use cortex_m::asm::delay;
use apds9151::Apds9151;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);    
    
    let mut afio = dp.AFIO.constrain();
    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1, (scl, sda), 
        &mut afio.mapr, 
        Mode::Fast { 
            frequency: 400_000.Hz(),
            duty_cycle: DutyCycle::Ratio2to1 
        }, 
        clocks, 
        1000, 
        10,
        1000,
        1000,
    );

    let mut color_sensor = Apds9151::new_apda9151(i2c);
    
    color_sensor.initialize().unwrap();

    loop {
        let red = color_sensor.get_red().unwrap();
        let green = color_sensor.get_green().unwrap();
        let blue = color_sensor.get_blue().unwrap();
        rprintln!("R: {:#?}", red);
        rprintln!("G: {:#?}", green);
        rprintln!("B: {:#?}", blue);

        let ir = color_sensor.get_ir().unwrap();
        rprintln!("IR: {:#?}", ir);

        delay(clocks.sysclk().raw() / 100);

    }
    
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/codytrey/apds9151/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.