use core::panic;

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;
use esp_idf_svc::hal::delay::FreeRtos;

pub struct Lcd1602a<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C: I2c> Lcd1602a<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    pub fn send_command(&mut self, cmd: u8) -> Result<(), I2C::Error> {
        let data_u = cmd & 0xF0;
        let data_l = (cmd << 4) & 0xF0;
        let data_t = [
            data_u | 0x0C, // en=1, rs=0
            data_u | 0x08, // en=0, rs=0
            data_l | 0x0C, // en=1, rs=0
            data_l | 0x08, // en=0, rs=0
        ];
        self.i2c.write(self.address, &data_t)
    }

    pub fn send_data(&mut self, data: u8) -> Result<(), I2C::Error> {
        let data_u = data & 0xF0;
        let data_l = (data << 4) & 0xF0;
        let data_t = [
            data_u | 0x0D, // en=1, rs=0
            data_u | 0x09, // en=0, rs=0
            data_l | 0x0D, // en=1, rs=0
            data_l | 0x09, // en=0, rs=0
        ];
        self.i2c.write(self.address, &data_t)
    }

    pub fn init(&mut self) -> Result<(), I2C::Error> {
        // 4 bit mode

        FreeRtos.delay_us(50_000); // wait for > 40ms

        self.send_command(0x30)?;
        FreeRtos.delay_us(5_000); // wait for > 4.1ms

        self.send_command(0x30)?;
        FreeRtos.delay_us(200); // wait for > 100us

        self.send_command(0x30)?;
        FreeRtos.delay_us(10_000);

        self.send_command(0x20)?; // 4 bit mode
        FreeRtos.delay_us(10_000);

        // display initialization

        self.send_command(0x28)?; // 4 bit mode, 2 lines, 5x8 characters
        FreeRtos.delay_us(1_000);

        self.send_command(0x08)?; // display off
        FreeRtos.delay_us(1_000);

        self.send_command(0x01)?; // clear display
        FreeRtos.delay_us(2_000);

        self.send_command(0x06)?; // entry mode set
        FreeRtos.delay_us(1_000);

        self.send_command(0x0C)?; // display on
        FreeRtos.delay_us(1_000);

        return Ok(());
    }

    pub fn clear(&mut self) -> Result<(), I2C::Error> {
        self.send_command(0x01)?;
        FreeRtos.delay_us(5_000);

        return Ok(());
    }

    pub fn put_cursor(&mut self, row: u8, col: u8) -> Result<(), I2C::Error> {
        if (col > 15) || (row > 1) {
            panic!("Invalid row or column number!");
        }

        match row {
            0 => self.send_command(0x80 | col),
            1 => self.send_command(0xC0 | col),
            _ => panic!("Invalid row number!"),
        }
    }

    pub fn send_string(&mut self, s: &str) -> Result<(), I2C::Error> {
        for c in s.chars() {
            self.send_data(c as u8)?;
        }
        return Ok(());
    }
}
