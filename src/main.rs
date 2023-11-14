// insert its contents inside a module named `my` under this scope
mod hardware;
use std::io::{self, Write};

use crate::hardware::fpga::{fpga_set_command_mode, fpga_spi_write, StepperDirection, fpga_stepper_control};

fn main() {
    io::stdout().flush().unwrap();  // flush the output buffer to ensure the prompt is displayed

    let mut distance = String::new();
    let mut direction = String::new();

    println!("Please enter a distance in mm:");
    io::stdin().read_line(&mut distance).unwrap();
    let distance: u32 = distance.trim().parse().expect("Please enter a valid number!");

    println!("Please enter a direction (up/down):");
    io::stdin().read_line(&mut direction).unwrap();
    let direction: StepperDirection = direction.trim().parse().expect("Please enter a valid direction!");

    match direction {
        StepperDirection::UP => println!("Moving {}mm up", distance),
        StepperDirection::DOWN => println!("Moving {}mm down", distance),
    }

    let mut command = fpga_stepper_control(distance, 8000, direction).unwrap().to_be_bytes();
    let hex_string: String = command.iter()
                                   .map(|b| format!("{:02x}", b))
                                   .collect::<Vec<String>>()
                                   .join("");
    println!("The bytes: {}", hex_string);
    command.reverse();

    print!("Are you sure you want to proceed? (y/n): ");
    io::stdout().flush().unwrap(); // Flush stdout to display the prompt immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Trim newline and check response
    match input.trim() {
        "y" | "Y" | "yes" | "Yes" => {
            // User confirmed, proceed
            fpga_spi_write(&command).unwrap();
            // Add the code you want to execute here
        },
        _ => {
            // User did not confirm, do not proceed
            println!("Exiting");
        },
    }

    
    // let _ = hardware::gpio::set_gpio_direction(0x56, Direction::Out).unwrap();
    // let _ = hardware::spi::create_spi().unwrap();
    // let _ = hardware::fpga::fpga_lattice_init().unwrap();
    // let _ = hardware::lt9711::lt9711_init().unwrap();
    // hardware::fpga::fpga_display_get_resolution_and_version().unwrap();
}
