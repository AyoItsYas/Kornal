use anyhow::Result;
use esp_idf_svc::hal::{
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

use kornal::lcd1602a::Lcd1602a;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;

    let mut lcd = Lcd1602a::new(i2c, 0x27);

    lcd.init()?;
    lcd.clear()?;
    lcd.put_cursor(0, 0)?;
    lcd.send_string("Hello, world!")?;

    return Result::Ok(());
}
