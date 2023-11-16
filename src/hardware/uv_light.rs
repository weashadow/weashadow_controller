use std::io;

use sysfs_pwm::Pwm;

pub struct UvLight {
    pwm: Pwm,
}

impl UvLight {
  /*
  luminance: 0-255
   */
    pub fn enable(&mut self, luminace: u8) -> io::Result<()> {
        // normalize luminance to 0-1
        let luminance = luminace as f32 / 255.0;
        self.pwm.set_duty_cycle(luminance).unwrap();
        Ok(())
    }
    pub fn disable(&mut self) {
        self.pwm.set_duty_cycle(0.0).unwrap();
    }
    pub fn new() -> io::Result<UvLight> {
        let pwm = Pwm::new(0, 0).unwrap();
        // export is unnecessary because chitu firmweware has already exported it
        // pwm.export().unwrap();
        Ok(UvLight { pwm })
    }
}
