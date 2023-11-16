use std::{cell::RefCell, io, rc::Rc};


pub mod axis;
pub mod build_screen;
pub mod command_mode;
pub mod gpio;
pub mod spi;
pub mod uv_light;

pub struct Hardware {
    pub axis: axis::Axis,
    pub uv_light: uv_light::UvLight,
    pub spi: Rc<RefCell<spi::Spi>>,
    pub command_mode: Rc<RefCell<command_mode::CommandMode>>,
    pub build_screen: Rc<RefCell<build_screen::BuildScreen>>
}

impl Hardware {
    pub fn new() -> io::Result<Hardware> {
        let spi = spi::Spi::init()?;
        let spi_rc = Rc::new(RefCell::new(spi));
        let uv = uv_light::UvLight::new()?;

        let cm = command_mode::CommandMode::new();
        let cm_rc = Rc::new(RefCell::new(cm));

        let axis = axis::Axis::new(Rc::clone(&spi_rc), Rc::clone(&cm_rc))?;

        let build_screen = build_screen::BuildScreen::new(Rc::clone(&spi_rc), Rc::clone(&cm_rc))?;
        let build_screen_rc = Rc::new(RefCell::new(build_screen));

        let hardware = Hardware {
            axis,
            spi: Rc::clone(&spi_rc),
            uv_light: uv,
            command_mode: Rc::clone(&cm_rc),
            build_screen: Rc::clone(&build_screen_rc)
        };
        Ok(hardware)
    }
}
