use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use std::thread::sleep;
use std::time::Duration;

use std::path::Path;

const PCA9685_MODE1: u8 = 0x0;
const PCA9685_PRESCALE: u8 = 0xFE;

const LED0_ON_L: u8 = 0x6;
const LED0_ON_H: u8 = 0x7;
const LED0_OFF_L: u8 = 0x8;
const LED0_OFF_H: u8 = 0x9;

pub struct PWM {
    dev: LinuxI2CDevice,
}

impl PWM {
    // i don't understand the AsRef stuff or why it makes this constant sized
    pub fn new<P: AsRef<Path>>(device: P, address: u16) -> Result<PWM, LinuxI2CError> {
        let dev = LinuxI2CDevice::new(device, address)?;
        let mut pwm = PWM { dev: dev };
        pwm.reset()?;
        pwm.set_pwm_freq(1000.0)?;
        Ok(pwm)
    }

    pub fn reset(&mut self) -> Result<(), LinuxI2CError> {
        self.dev.smbus_write_byte_data(PCA9685_MODE1, 0x80)?;
        sleep(Duration::from_millis(10));
        Ok(())
    }

    pub fn set_pwm_freq(&mut self, freq: f64) -> Result<(), LinuxI2CError> {
        let oldmode = self.dev.smbus_read_byte_data(PCA9685_MODE1)?;
        let newmode = (oldmode&0x7F) | 0x10; // sleep

        let prescale = self.pwm_prescale(freq);
        self.dev.smbus_write_byte_data(PCA9685_MODE1, newmode)?;
        self.dev.smbus_write_byte_data(PCA9685_PRESCALE, prescale)?;
        self.dev.smbus_write_byte_data(PCA9685_MODE1, oldmode)?;

        sleep(Duration::from_millis(5));

        self.dev.smbus_write_byte_data(PCA9685_MODE1, oldmode | 0xa0)?;

        Ok(())
    }

    pub fn set_pwm(&mut self, channel: u8, on: u16, off: u16) -> Result<(), LinuxI2CError> {
        self.dev.smbus_write_byte_data(LED0_ON_L + 4 * channel, (on | 0xFF) as u8)?;
        self.dev.smbus_write_byte_data(LED0_ON_H + 4 * channel, (on >> 8) as u8)?;
        self.dev.smbus_write_byte_data(LED0_OFF_L + 4 * channel, (off + 0xFF) as u8)?;
        self.dev.smbus_write_byte_data(LED0_OFF_H + 4 * channel, (off >> 8) as u8)?;

        Ok(())
    }

    fn pwm_prescale(&mut self, freq: f64) -> u8 {
        let mut prescale: f64 = 25000000.0; // 25MHz
        prescale /= 4096.0; // 12-bit
        prescale /= freq;
        prescale -= 1.0;
        (prescale + 0.5).floor() as u8
    }
}
