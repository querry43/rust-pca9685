rust-pca9685
============

The Rust `pca9685` crate is for manipulating the [Adafruit 16-Channel servo driver](https://www.adafruit.com/product/815) and other boards based on the PCA9685.  I wrote this before I found i2c-pca9685 with nearly the identical interface.  You should probably use that instead.

Example
-------

```rust
extern crate pca9685;
extern crate i2cdev;

use pca9685::pwm::PWM;
use i2cdev::linux::{LinuxI2CError};

fn main() {
    match move_servo() {
        Ok(()) => println!("it worked!"),
        Err(err) => println!("uhoh: {}", err),
    }
}

fn move_servo() -> Result<(), LinuxI2CError> {
    let mut pwm = PWM::new("/dev/i2c-1", 0x40)?;
    pwm.set_pwm_freq(60.0)?;
    pwm.set_pwm(0, 0, 500)?;
    pwm.set_pwm(1, 0, 500)?;
    Ok(())
}
```

Features
--------

- [x] Works
- [ ] Stop it from compiling every time
- [ ] Documented
- [ ] Tested
