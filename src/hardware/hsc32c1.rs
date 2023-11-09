extern crate i2cdev;

use std::{thread, io};
use std::time::Duration;

use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;

pub fn hsc32c1_init() -> Result<(), io::Error> {
    let mut dev = LinuxI2CDevice::new("/dev/i2c-0", 0x100)?;

    thread::sleep(Duration::from_millis(100));

    

    Ok(())
}