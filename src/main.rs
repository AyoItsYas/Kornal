#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, Level, Output},
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use esp_println::println;
use kornal::lcd1602a::Lcd1602a;

#[entry]
fn main() -> ! {
    println!("Hello world!");

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let delay = Delay::new(&clocks);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut gpio2 = Output::new(io.pins.gpio2, Level::Low);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100.kHz(),
        &clocks,
        None,
    );

    let mut lcd = Lcd1602a::new(i2c, 0x27, delay);

    lcd.init();
    lcd.clear();
    lcd.put_cursor(0, 0);
    lcd.send_string("Hello world!!!");

    loop {
        gpio2.toggle();
        delay.delay_millis(500u32);
    }
}
