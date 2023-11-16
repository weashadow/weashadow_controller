use std::{cell::RefCell, io, rc::Rc};

use super::{command_mode::CommandMode, spi::Spi};

#[derive(Debug)]
pub struct BuildScreen {
    spi: Rc<RefCell<Spi>>,
    command_mode: Rc<RefCell<CommandMode>>,
}

impl BuildScreen {
    pub fn new(
        spi: Rc<RefCell<Spi>>,
        command_mode: Rc<RefCell<CommandMode>>,
    ) -> io::Result<BuildScreen> {
        Ok(BuildScreen { spi, command_mode })
    }

    pub fn display_image_data(&mut self, data: &[u8]) {
        self.command_mode.borrow_mut().disable().unwrap();
        let command = [0, 0xfe];
        self.spi.borrow_mut().send(&command).unwrap();
        let command = [0x10, 0xfe];
        self.spi.borrow_mut().send(&command).unwrap();
        let command = [0x20, 0xfe];
        self.spi.borrow_mut().send(&command).unwrap();
        let command = [0xfd, 0xfe];
        self.spi.borrow_mut().send(&command).unwrap();

        self.spi.borrow_mut().send_image_data(data).unwrap();

        let command = [0x8, 0xfe];
        self.spi.borrow_mut().send(&command).unwrap();
    }
}
