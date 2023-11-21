mod hardware;
mod ui;
mod printer;
use std::{thread, time::Duration, fs};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Move axis: multiple by 0.1 mm
    #[arg(short, long, default_value_t = 0, allow_hyphen_values = true)]
    move_axis: i8,

    /// Get current position
    #[arg(short, long)]
    position: bool,

    /// UV Light: 0-255
    #[arg(short, long, default_value_t = 0 )]
    uv_light: u8,

    /// Show image located in /customer/resource/***.bin  argurment shoud be file path
    #[arg(short, long, default_value = "")]
    show_image: String,

    #[arg(short, long)]
    display: bool,
}

fn main() {
    let mut hardware = hardware::Hardware::new().unwrap();

    let args = Args::parse();

    if args.move_axis != 0 {
        if args.move_axis > 0 {
            println!("Moving Up {:?} * 0.1 mm ", args.move_axis);
            hardware.axis.move_axis(args.move_axis as u32, 5400, hardware::axis::AxisDirection::UP).unwrap();
        }
        else {
            println!("Moving Down {:?} * 0.1 mm", -args.move_axis);
            hardware.axis.move_axis((-args.move_axis) as u32, 5400, hardware::axis::AxisDirection::DOWN).unwrap();
        }
    }
    // delay 1000ms
    thread::sleep(Duration::from_millis(1000));
    if args.position {
        let pos = hardware.axis.get_position_in_micrometer().unwrap();
        println!("Position: {:?} micro meter", pos);
    } 
    if args.uv_light != 0 {
        println!("UV Light: {:?} ", args.uv_light);
        hardware.uv_light.enable(args.uv_light).unwrap();
    } else {
        hardware.uv_light.disable();
    }
    if args.show_image != "" {
        println!("Show Image: {:?}", args.show_image);
        let data = fs::read(args.show_image).unwrap();
        let data = &data[0x30..];
        hardware.build_screen.borrow_mut().display_image_data(data);
    }
    if args.display {
        ui::ui::display().unwrap();
    }

}
