#![crate_type = "lib"]
#![crate_name = "pca9685"]

extern crate i2cdev;

pub mod pwm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
