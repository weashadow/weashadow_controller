// insert its contents inside a module named `my` under this scope
mod hardware;
use std::io::{self, Write};

use sysfs_gpio::Direction;

use crate::hardware::fpga::{fpga_set_command_mode, fpga_spi_write};

fn main() {
    // let _ = hardware::gpio::set_gpio_direction(0x56, Direction::Out).unwrap();
    // let _ = hardware::spi::create_spi().unwrap();
    // let _ = hardware::fpga::fpga_lattice_init().unwrap();
    // let _ = hardware::lt9711::lt9711_init().unwrap();
    // hardware::fpga::fpga_display_get_resolution_and_version().unwrap();
    let mut input = String::new();

    print!("Please enter a hexadecimal value: ");
    io::stdout().flush().unwrap();  // flush the output buffer to ensure the prompt is displayed

    io::stdin().read_line(&mut input).unwrap();

    // Remove trailing newline
    let input = input.trim_end();

    // Convert the hex string to a Vec<u8>
    let buffer: Vec<u8> = hex::decode(input).unwrap_or_else(|e| {
        eprintln!("Error decoding hex string: {}", e);
        std::process::exit(1);
    });

    fpga_set_command_mode(1).unwrap();
    
    fpga_spi_write(&buffer).unwrap();
}
