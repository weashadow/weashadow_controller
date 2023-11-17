extern crate alloc;
use alloc::{rc::Rc, boxed::Box};
use slint::platform::{Platform, software_renderer::MinimalSoftwareWindow};

slint::include_modules!();

struct MyPlatform {
    window: Rc<MinimalSoftwareWindow>,
    // optional: some timer device from your device's HAL crate
}

impl Platform for MyPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        // Since on MCUs, there can be only one window, just return a clone of self.window.
        // We'll also use the same window in the event loop.
        Ok(self.window.clone())
    }
    // optional: You can put the event loop there, or in the main function, see later
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        todo!();
    }
}

// #[hal::entry]
fn display() {
    // Initialize the heap allocator, peripheral devices and other things.
    // ...

    // Initialize a window (we'll need it later).
    let window = MinimalSoftwareWindow::new(Default::default());
    slint::platform::set_platform(Box::new(MyPlatform {
        window: window.clone(),
        //...
    }))
    .unwrap();

    // Setup the UI.
    // let ui = MyUI::new();
    // ... setup callback and properties on `ui` ...

    // Make sure the window covers our entire screen.
    window.set_size(slint::PhysicalSize::new(800, 640));

    // ... start event loop (see later) ...
}