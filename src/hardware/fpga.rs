extern crate spidev;
use std::io;
use std::io::prelude::*;

use crate::hardware::gpio;
use sysfs_gpio::Direction;

use super::spi::get_spi;

pub fn fpga_lattice_init() -> Result<(), io::Error> {
    let pin_55 = gpio::set_gpio_direction(0x55, Direction::In)?;
    let pin_0d = gpio::set_gpio_direction(0x0d, Direction::In)?;

    let pin_55_value = pin_55.get_value().unwrap();
    let pin_0d_value = pin_0d.get_value().unwrap();

    println!("Pin 0x55 value: {}", pin_55_value);
    println!("Pin 0x0d value: {}", pin_0d_value);

    let pin_0e = gpio::set_gpio_direction(0x0e, Direction::Out)?;
    let pin_02 = gpio::set_gpio_direction(0x02, Direction::In)?;
    let pin_03 = gpio::set_gpio_direction(0x03, Direction::In)?;
    let pin_01 = gpio::set_gpio_direction(0x01, Direction::In)?;
    let pin_57 = gpio::set_gpio_direction(0x57, Direction::In)?;

    let mut var_1 = 0xb;

    while (pin_55.get_value().unwrap() == 0 || pin_0d.get_value().unwrap() == 0) && var_1 != 0 {
        var_1 -= 1;
        pin_0e.set_value(0).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        pin_0e.set_value(1).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let pin_55_value = pin_55.get_value().unwrap();
    let pin_0d_value = pin_0d.get_value().unwrap();

    println!("Pin 0x55 value: {}", pin_55_value);
    println!("Pin 0x0d value: {}", pin_0d_value);

    if pin_55_value == 0 || pin_0d_value == 0 {
        println!("fpga_lattice_init failed");
        return Err(io::Error::new(io::ErrorKind::Other, "fpga_lattice_init failed"));
    } else {
        println!("fpga_lattice_init ok");
        return Ok(());
    }
}

pub fn fpga_display_get_resolution_and_version() -> Result<(), io::Error> {
    let mut buffer = [0u8; 20];
    fpga_set_command_mode(0)?;
    let command: [u8; 2] = [0xfe, 0x04];
    fpga_spi_write(&command).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(200));
    fpga_set_command_mode(0)?;
    let mut buffer = [0u8; 17];
    let len = fpga_spi_read(&mut buffer[..]).unwrap();
    println!("The bytes: {:?}", &buffer[..len]);
    Ok(())
}

pub fn fpga_pwn_set_duty() -> Result<(), io::Error> {
    let mut buffer = [0u8; 20];
    fpga_set_command_mode(0)?;
    let command: [u8; 2] = [0xfe, 0x04];
    fpga_spi_write(&command).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(200));
    fpga_set_command_mode(0)?;
    let mut buffer = [0u8; 17];
    let len = fpga_spi_read(&mut buffer[..]).unwrap();
    println!("The bytes: {:?}", &buffer[..len]);
    Ok(())
}

pub fn fpga_set_command_mode(i: u8) -> Result<(), io::Error> {
    let pin_56 = gpio::set_gpio_direction(0x56, Direction::Out)?;
    pin_56.set_value(i).unwrap();
    return Ok(());
}

pub fn fpga_spi_write(i: &[u8]) -> io::Result<usize> {
    let mut spi = get_spi().unwrap();
    spi.write(i)
}

fn fpga_spi_read(i: &mut [u8]) -> io::Result<usize> {
    let mut spi = get_spi().unwrap();
    spi.read(i)
}