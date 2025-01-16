use embedded_hal::i2c::I2c;
use esp_hal::delay::Delay;

pub struct AHT20<I2C> {
    i2c: I2C,
    address: u8,
    delay: Delay,
}

impl<I2C, E> AHT20<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C, address: u8, delay: Delay) -> Self {
        Self {
            i2c,
            address,
            delay,
        }
    }

    pub fn init(&mut self) -> Result<(), E> {
        let cmd_init = [0xBE, 0x08, 0x00];
        self.i2c.write(self.address, &cmd_init)?;
        self.delay.delay_nanos(350_000);

        return Ok(());
    }

    pub fn read_values(&mut self) -> Result<(f32, f32, u32, u32), E> {
        let cmd_meas = [0xAC, 0x33, 0x00];
        self.i2c.write(self.address, &cmd_meas)?;
        self.delay.delay_nanos(80_000);

        let mut buf = [0u8; 7];
        self.i2c.read(self.address, &mut buf)?;

        let hum_raw = ((buf[1] as u32) << 12) | ((buf[2] as u32) << 4) | ((buf[3] as u32) >> 4);
        let temp_raw = ((buf[3] as u32 & 0x0F) << 16) | ((buf[4] as u32) << 8) | (buf[5] as u32);

        let humidity = (hum_raw as f32) * 100.0 / 1048576.0;
        let temperature = (temp_raw as f32) * 200.0 / 1048576.0 - 50.0;

        return Ok((temperature, humidity, temp_raw, hum_raw));
    }
}
