extern crate alloc;
use std::{
    io::{self, Write},
    time::{self, Instant},
};

use alloc::rc::Rc;
use framebuffer::Framebuffer;
use slint::platform::{software_renderer::MinimalSoftwareWindow, Platform };

slint::include_modules!();

const DISPLAY_WIDTH: usize = 800;
const DISPLAY_HEIGHT: usize = 480;

struct TouchScreen {
    window: Rc<MinimalSoftwareWindow>,
    framebuffer: Framebuffer,
    timer: time::Instant,
}

impl Platform for TouchScreen {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }
    fn duration_since_start(&self) -> core::time::Duration {
        self.timer.elapsed()
    }
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        loop {
        }
    }
}

pub fn display() -> io::Result<()> {
    let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();
    let h = framebuffer.var_screen_info.yres;
    let line_length = framebuffer.fix_screen_info.line_length;
    let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

    println!(
        "h: {}, line_length: {}, bytespp: {}",
        h, line_length, bytespp
    );

    // let mut frame = vec![0u8; 3072000 as usize];
    // frame.fill(0xff);
    // let _ = framebuffer.write_frame(&frame);

    let window = MinimalSoftwareWindow::new(
        slint::platform::software_renderer::RepaintBufferType::ReusedBuffer,
    );

    slint::platform::set_platform(Box::new(TouchScreen {
        window: window.clone(),
        framebuffer,
        timer: Instant::now(),
    }))
    .unwrap();

    let ui = AppWindow::new();
    // ... setup callback and properties on `ui` ...

    // Make sure the window covers our entire screen.
    window.set_size(slint::PhysicalSize::new(320, 240));

    Ok(())
}
