use std::io;

use sysfs_gpio::Pin;

use super::gpio::set_gpio_direction;

static COMMAND_PIN:u64 = 0x56;

#[derive(Debug)]
pub struct CommandMode {
  pub command_mode: bool,
  command_gpio: Pin
}

impl CommandMode {
  pub fn new() -> CommandMode {
    let command_pin = set_gpio_direction(COMMAND_PIN, sysfs_gpio::Direction::Out).unwrap();
    CommandMode {
      command_mode: false,
      command_gpio: command_pin
    }
  }

  pub fn enable(&mut self) -> io::Result<()> {
    self.command_gpio.set_value(1).unwrap();
    self.command_mode = true;
    Ok(())
  }

  pub fn disable(&mut self) -> io::Result<()> {
    self.command_gpio.set_value(0).unwrap();
    self.command_mode = false;
    Ok(())
  }
}