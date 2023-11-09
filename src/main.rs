// insert its contents inside a module named `my` under this scope
mod hardware;
use sysfs_gpio::Direction;

fn main() {
    // let _ = hardware::gpio::set_gpio_direction(0x56, Direction::Out).unwrap();
    // let _ = hardware::spi::create_spi().unwrap();
    // let _ = hardware::fpga::fpga_lattice_init().unwrap();
    // let _ = hardware::lt9711::lt9711_init().unwrap();
    hardware::fpga::fpga_display_get_resolution_and_version().unwrap();
}
