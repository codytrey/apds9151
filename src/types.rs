#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2C(E),
    /// CRC checksum mismatch
    ChecksumMismatch,
}

// Main Control
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum MainControl {
    RGBMode = 0x04,
    LightSensorEnable = 0x02,
    ProximitySensorEnable = 0x01,
    OFF = 0x00,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ProximitySensorResolution {
    Resolution8Bits = 0x00,
    Resolution9Bits = 0x01,
    Resolution10Bits = 0x10,
    Resolution11Bits = 0x11,
}

impl Default for ProximitySensorResolution {
    fn default() -> Self {
        ProximitySensorResolution::Resolution11Bits
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ProximitySensorRate {
    Rate6_25Ms = 0x001,
    Rate12_5Ms = 0x010,
    Rate25Ms = 0x011,
    Rate50Ms = 0x100,
    Rate100Ms = 0x101,
    Rate200Ms = 0x110,
    Rate400Ms = 0x111,
}

impl Default for ProximitySensorRate {
    fn default() -> Self {
        ProximitySensorRate::Rate50Ms
    }
}

#[derive(Debug)]
pub enum LEDPulseFreq {
    Rate60kHz = 0x18,
    Rate70kHz = 0x40,
    Rate80kHz = 0x28,
    Rate90kHz = 0x30,
    Rate100kHz = 0x38,
}

impl Default for LEDPulseFreq {
    fn default() -> Self {
        LEDPulseFreq::Rate60kHz
    }
}

#[derive(Debug)]
pub enum LEDCurrent {
    Pulse2mA = 0,
    Pulse5mA = 1,
    Pulse10mA = 2,
    Pulse25mA = 3,
    Pulse50mA = 4,
    Pulse75mA = 5,
    Pulse100mA = 6,
    Pulse125mA = 7,
}

impl Default for LEDCurrent {
    fn default() -> Self {
        LEDCurrent::Pulse100mA
    }
}

#[derive(Debug)]
pub enum ColorResolution {
    Resolution20bit = 0x00,
    Resolution19bit = 0x10,
    Resolution18bit = 0x20,
    Resolution17bit = 0x30,
    Resolution16bit = 0x40,
    Resolution13bit = 0x50,
}

impl Default for ColorResolution {
    fn default() -> Self {
        ColorResolution::Resolution18bit
    }
}

#[derive(Debug)]
pub enum ColorMeaseurementRate {
    Rate25ms = 0,
    Rate50ms = 1,
    Rate100ms = 2,
    Rate200ms = 3,
    Rate500ms = 4,
    Rate1000ms = 5,
    Rate2000ms = 7,
}

impl Default for ColorMeaseurementRate {
    fn default() -> Self {
        ColorMeaseurementRate::Rate100ms
    }
}

#[derive(Debug)]
pub enum GainFactor {
    Gain1x = 0,
    Gain3x = 1,
    Gain6x = 2,
    Gain9x = 3,
    Gain18x = 4,
}

impl Default for GainFactor {
    fn default() -> Self {
        GainFactor::Gain3x
    }
}