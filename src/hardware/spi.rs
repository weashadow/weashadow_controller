extern crate spidev;
use spidev::Spidev;
use std::io;
use std::io::Read;
use std::io::Write;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Spi {
    spi: Spidev,
}

impl Spi {
    pub fn init() -> io::Result<Spi> {
        let spi = Spidev::open("/dev/spidev0.0")?;
        Ok(Spi { spi })
    }

    pub fn send(&mut self, data: &[u8]) -> io::Result<usize> {
        // reverse data for little endian
        let mut data = data.to_vec();
        let chunks = data.chunks_mut(8);
        for chunk in chunks {
            chunk.reverse();
        }
        let size = self.spi.write(&data[..]).unwrap();
        // delay 10ms, not necessary, just for test
        thread::sleep(Duration::from_millis(10));
        Ok(size)
    }

    pub fn read(&mut self, data: &mut [u8]) -> io::Result<usize> {
        let size = self.spi.read(data).unwrap();
        Ok(size)
    }
}
