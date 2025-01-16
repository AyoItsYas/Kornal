use core::panic;

use embedded_hal::i2c::I2c;
use esp_hal::delay::Delay;

pub struct Lcd1602a<I2C> {
    i2c: I2C,
    address: u8,
    delay: Delay,
}

impl<I2C: I2c> Lcd1602a<I2C> {
    pub fn new(i2c: I2C, address: u8, delay: Delay) -> Self {
        Self {
            i2c,
            address,
            delay,
        }
    }

    pub fn send_command(&mut self, cmd: u8) {
        let data_u = cmd & 0xF0;
        let data_l = (cmd << 4) & 0xF0;
        let data_t = [
            data_u | 0x0C, // en=1, rs=0
            data_u | 0x08, // en=0, rs=0
            data_l | 0x0C, // en=1, rs=0
            data_l | 0x08, // en=0, rs=0
        ];
        self.i2c.write(self.address, &data_t).unwrap();

        return;
    }

    pub fn send_data(&mut self, data: u8) {
        let data_u = data & 0xF0;
        let data_l = (data << 4) & 0xF0;
        let data_t = [
            data_u | 0x0D, // en=1, rs=0
            data_u | 0x09, // en=0, rs=0
            data_l | 0x0D, // en=1, rs=0
            data_l | 0x09, // en=0, rs=0
        ];
        self.i2c.write(self.address, &data_t).unwrap();

        return;
    }

    pub fn init(&mut self) {
        // 4 bit mode

        // FreeRtos.delay_us(50_000); // wait for > 40ms
        self.delay.delay_millis(50u32);

        self.send_command(0x30);
        // FreeRtos.delay_us(5_000); // wait for > 4.1ms
        self.delay.delay_millis(5u32);

        self.send_command(0x30);
        // FreeRtos.delay_us(200); // wait for > 100us
        self.delay.delay_millis(2u32);

        self.send_command(0x30);
        // FreeRtos.delay_us(10_000);
        self.delay.delay_millis(10u32);

        self.send_command(0x20); // 4 bit mode
                                 //FreeRtos.delay_us(10_000);
        self.delay.delay_millis(10u32);

        // display initialization

        self.send_command(0x28); // 4 bit mode, 2 lines, 5x8 characters
                                 // FreeRtos.delay_us(1_000);
        self.delay.delay_millis(1u32);

        self.send_command(0x08); // display off
                                 // FreeRtos.delay_us(1_000);
        self.delay.delay_millis(1u32);

        self.send_command(0x01); // clear display
                                 // FreeRtos.delay_us(2_000);
        self.delay.delay_millis(2u32);

        self.send_command(0x06); // entry mode set
                                 // FreeRtos.delay_us(1_000);
        self.delay.delay_millis(1u32);

        self.send_command(0x0C); // display on
                                 // FreeRtos.delay_us(1_000);
        self.delay.delay_millis(1u32);

        return;
    }

    pub fn clear(&mut self) {
        self.send_command(0x01);
        self.delay.delay_millis(5u32);

        return;
    }

    pub fn put_cursor(&mut self, row: u8, col: u8) {
        if (col > 15) || (row > 1) {
            panic!("Invalid row or column number!");
        }

        match row {
            0 => self.send_command(0x80 | col),
            1 => self.send_command(0xC0 | col),
            _ => panic!("Invalid row number!"),
        }

        return;
    }

    pub fn send_string(&mut self, s: &str) {
        for c in s.chars() {
            self.send_data(c as u8);
        }

        return;
    }

    pub fn send_number(&mut self, n: u32) {
        let mut num = n;
        let mut digits = [0u8; 10];
        let mut i = 0;

        while num > 0 {
            digits[i] = (num % 10) as u8;
            num /= 10;
            i += 1;
        }

        while i > 0 {
            i -= 1;
            self.send_data((digits[i] + 48) as u8);
        }

        return;
    }
}
