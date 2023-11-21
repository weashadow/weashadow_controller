extern crate alloc;
use std::time::{self, Duration, Instant};

use alloc::rc::Rc;
use framebuffer::Framebuffer;
use i_slint_core::software_renderer::WindowRotation;
use slint::{
    platform::{software_renderer::MinimalSoftwareWindow, Platform},
    PlatformError,
};

use super::pixel::BGRAPixel;

slint::include_modules!();

const DISPLAY_WIDTH: u32 = 800;
const DISPLAY_HEIGHT: u32 = 480;

struct TouchScreen {
    window: Rc<MinimalSoftwareWindow>,
    // framebuffer: Framebuffer,
    // pixels: &'a mut [BGRAPixel],
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
        let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();
        let (_, pixels, _) = unsafe { framebuffer.frame.align_to_mut::<BGRAPixel>() };
        let half_pixels: &mut [BGRAPixel] = &mut pixels[0..384000];
        loop {
            self.window.draw_if_needed(|renderer| {
                // if let Some(event) = check_for_touch_event(/*...*/) {
                //     window.dispatch_event(event);
                // }
                renderer.set_window_rotation(WindowRotation::Rotate90);
                renderer.render(half_pixels, DISPLAY_HEIGHT as usize);

                if !self.window.has_active_animations() {
                    std::thread::sleep(
                        slint::platform::duration_until_next_timer_update()
                            .unwrap_or(Duration::from_secs(1)),
                    );
                }
            });
        }
    }
}

pub fn display() -> Result<(), PlatformError> {

    let window = MinimalSoftwareWindow::new(
        slint::platform::software_renderer::RepaintBufferType::ReusedBuffer,
    );

    window.set_size(slint::PhysicalSize::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));

    slint::platform::set_platform(Box::new(TouchScreen {
        window: window.clone(),
        // framebuffer,
        timer: Instant::now(),
        // pixels,
    }))
    .unwrap();

    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    ui.run()
}
