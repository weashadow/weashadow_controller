use std::io;

use sysfs_gpio::{Direction, Pin};

pub fn set_gpio_direction(pin: u64, dir: Direction) ->Result<Pin, io::Error> {
    let p = Pin::new(pin); // number depends on chip, etc.
    p.export().unwrap();
    p.set_direction(dir).unwrap();
    Ok(p)
}
