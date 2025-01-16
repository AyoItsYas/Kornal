#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    i2c::master::I2c,
    prelude::*,
};
use esp_println::println;
use fugit::HertzU32;
use kornal::aht20::AHT20;
use kornal::lcd1602a::Lcd1602a;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    let delay = Delay::new();

    let mut i2c = I2c::new(peripherals.I2C0, {
        let mut config = esp_hal::i2c::master::Config::default();
        config.frequency = HertzU32::kHz(200);
        config
    })
    .with_sda(peripherals.GPIO21)
    .with_scl(peripherals.GPIO22);

    loop {
        let mut aht20 = AHT20::new(&mut i2c, 0x38, delay);
        aht20.init().unwrap();
        let (temp, hum, _, _) = aht20.read_values().unwrap();

        let mut lcd = Lcd1602a::new(&mut i2c, 0x27, delay);

        lcd.init();

        lcd.put_cursor(0, 0);
        lcd.send_string("Temperature : ");
        lcd.send_number(temp as u32);

        lcd.put_cursor(1, 0);
        lcd.send_string("Humidity    : ");
        lcd.send_number(hum as u32);

        println!("{{\"temp\": {:.20}, \"hum\": {:.20}}}", temp, hum);

        delay.delay_millis(950);
        led.toggle();
        delay.delay_millis(50);
        led.toggle();
    }
}
