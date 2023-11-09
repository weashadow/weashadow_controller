extern crate i2cdev;

use std::{thread, io};
use std::time::Duration;

use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;

pub fn lt9711_init() -> Result<(), io::Error> {
    let mut dev = LinuxI2CDevice::new("/dev/i2c-0", 0x41)?;

    thread::sleep(Duration::from_millis(100));

    dev.smbus_write_byte_data(0xff, 0x80)?;
    dev.smbus_write_byte_data(0xee, 0x01)?;
    dev.smbus_write_byte_data(0xff, 0x81)?;

    thread::sleep(Duration::from_millis(100));

    let reg_0 = dev.smbus_read_byte_data(0x00)?;
    let reg_1 = dev.smbus_read_byte_data(0x01)?;
    let reg_2 = dev.smbus_read_byte_data(0x02)?;

    println!("reg1: {}", reg_0);
    println!("reg2: {}", reg_1);
    println!("reg3: {}", reg_2);

    Ok(())



    // loop {
    //     let mut buf: [u8; 6] = [0; 6];
    //     dev.smbus_write_byte(0x00).unwrap();
    //     thread::sleep(Duration::from_millis(10));
    //     dev.read(&mut buf).unwrap();
    //     println!("Reading: {:?}", buf);
    // }
}