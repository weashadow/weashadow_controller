use evdev::Device;

pub fn test_touch_screen() {
    let mut d = Device::open("/dev/input/event0").unwrap();
    println!("{d}");
    loop {
        for ev in d.fetch_events().unwrap() {
            println!("{ev:?}");
        }
    }
}
