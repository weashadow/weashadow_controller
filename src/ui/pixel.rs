use slint::platform::software_renderer::{PremultipliedRgbaColor, TargetPixel};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BGRAPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}

impl BGRAPixel {}

impl TargetPixel for BGRAPixel {
    fn background() -> Self {
      Self { blue: 0, green: 0, red: 0, alpha: 0x0 }
    }

    fn blend(&mut self, color: PremultipliedRgbaColor) {
        let a = (u8::MAX - color.alpha) as u16;

        self.red = (self.red as u16 * a / 255) as u8 + color.red;
        self.green = (self.green as u16 * a / 255) as u8 + color.green;
        self.blue = (self.blue as u16 * a / 255) as u8 + color.blue;
        self.alpha = (self.alpha as u16 + color.alpha as u16
            - (self.alpha as u16 * color.alpha as u16) / 255) as u8;
    }

    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
      Self { blue, green, red, alpha: 0xff }
    }
}
