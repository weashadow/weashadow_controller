use std::io;

use spidev::SpiModeFlags;
use spidev::SpidevOptions;

use spidev::Spidev;

pub fn get_spi() -> Result<Spidev, io::Error> {
  let mut spi = Spidev::open("/dev/spidev0.0")?;
  // let options = SpidevOptions::new()
  //      .bits_per_word(8)
  //      .max_speed_hz(30000000)
  //      .mode(SpiModeFlags::SPI_MODE_3)
  //      .build();
  // spi.configure(&options)?;
  Ok(spi)
}