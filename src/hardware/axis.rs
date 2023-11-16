use std::{cell::RefCell, io, rc::Rc, thread, time::Duration};

use byteorder::{BigEndian, ByteOrder};

use super::{command_mode::CommandMode, spi::Spi};

const F0: u16 = 671;
const DF: u16 = 400;

static GET_POSITION_COMMAND: u32 = 0x10fe;
static SET_POSITION_COMMAND: u32 = 0x01fe;

pub enum AxisDirection {
    UP,
    DOWN,
}

#[derive(Debug)]
pub struct Axis {
    spi: Rc<RefCell<Spi>>,
    command_mode: Rc<RefCell<CommandMode>>,

    halt: bool,
}

impl Axis {
    pub fn move_axis(
        &mut self,
        distance: u32,
        speed: u16,
        direction: AxisDirection,
    ) -> io::Result<()> {
        let s = (speed - F0) / DF;
        let spped_result =
            s.wrapping_add(2).wrapping_shl(8) | s.wrapping_add(2).wrapping_shr(8) & 0xff;

        let base = 800; // 1mm
        let base = 80; // 0.1mm
                       // let base = 8; // 0.01mm
        let d: u32 = distance * base;
        let distance_result = (d & 0xff) << 8 | (d >> 8) & 0xff;

        let direction_result: u16 = match direction {
            AxisDirection::UP => 0x0000,
            AxisDirection::DOWN => 0x8000,
        };

        let result: u64 = u64::from(spped_result).wrapping_shl(48)
            + u64::from(distance_result).wrapping_shl(32)
            + u64::from(direction_result).wrapping_shl(16)
            + u64::from(SET_POSITION_COMMAND);

        let command = result.to_be_bytes();

        self.command_mode.borrow_mut().enable()?;
        self.spi.borrow_mut().send(&command)?;

        return Ok(());
    }

    pub fn get_position_in_micrometer(&mut self) -> io::Result<u32> {
        self.command_mode.borrow_mut().enable()?;
        self.spi
            .borrow_mut()
            .send(&GET_POSITION_COMMAND.to_be_bytes())?;
        let mut buffer = [0u8; 8];
        self.spi.borrow_mut().read(&mut buffer)?;
        println!("{:X?}", buffer);

        thread::sleep(Duration::from_millis(100));
        // [TODO] need to figure out why this command need to be send twice to get data
        self.spi
            .borrow_mut()
            .send(&GET_POSITION_COMMAND.to_be_bytes())?;
        thread::sleep(Duration::from_millis(100));

        let mut buffer = [0u8; 8];
        self.spi.borrow_mut().read(&mut buffer)?;
        println!("{:2X?}", buffer);

        let mut padded_buffer = [0; 4];
        let slice_len = buffer[3..6].len();
        padded_buffer[(4 - slice_len)..].copy_from_slice(&buffer[3..6]);
        println!("{:2X?}", padded_buffer);

        // I'm not sure, the system is actually little endian, but this works.
        let num = BigEndian::read_u32(&padded_buffer);

        // 00 00 00 50 preset 0.1mm
        let position = num / 8;

        Ok(position)
    }

    pub fn move_to_home(&mut self) -> io::Result<()> {
        // get current position
        // self.get_position()?;

        // let distance = 0;
        // // caculate the distance to home
        // self.move_axis(0, 8000, AxisDirection::UP)?;
        Ok(())
    }

    pub fn new(spi: Rc<RefCell<Spi>>, command_mode: Rc<RefCell<CommandMode>>) -> io::Result<Axis> {
        let axis = Axis {
            spi,
            command_mode,
            halt: false,
        };
        Ok(axis)
    }
}
