use std::{fs::File, io, os::fd::AsRawFd};
use derivative::Derivative;


use ioctl_sys::ioctl;

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct FbBitfield {
    #[derivative(Default(value = "0"))]
    offset: u32, /* beginning of bitfield */
    #[derivative(Default(value = "0"))]
    length: u32,    /* length of bitfield */
    #[derivative(Default(value = "0"))]
    msb_right: u32, /* != 0 : Most significant bit is right */
}

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct FbVarScreeninfo {
    #[derivative(Default(value = "0"))]
    xres: u32,
    #[derivative(Default(value = "0"))]
    yres: u32,
    #[derivative(Default(value = "0"))]
    xres_virtual: u32,
    #[derivative(Default(value = "0"))]
    yres_virtual: u32,
    #[derivative(Default(value = "0"))]
    xoffset: u32,
    #[derivative(Default(value = "0"))]
    yoffset: u32,
    #[derivative(Default(value = "0"))]
    bits_per_pixel: u32,
    #[derivative(Default(value = "0"))]
    grayscale: u32,
    red: FbBitfield,
    green: FbBitfield,
    blue: FbBitfield,
    transp: FbBitfield,
    #[derivative(Default(value = "0"))]
    nonstd: u32,
    #[derivative(Default(value = "0"))]
    activate: u32,
    #[derivative(Default(value = "0"))]
    height: u32,
    #[derivative(Default(value = "0"))]
    width: u32,
    #[derivative(Default(value = "0"))]
    accel_flags: u32,
    #[derivative(Default(value = "0"))]
    pixclock: u32,
    #[derivative(Default(value = "0"))]
    left_margin: u32,
    #[derivative(Default(value = "0"))]
    right_margin: u32,
    #[derivative(Default(value = "0"))]
    upper_margin: u32,
    #[derivative(Default(value = "0"))]
    lower_margin: u32,
    #[derivative(Default(value = "0"))]
    hsync_len: u32,
    #[derivative(Default(value = "0"))]
    vsync_len: u32,
    #[derivative(Default(value = "0"))]
    sync: u32,
    #[derivative(Default(value = "0"))]
    vmode: u32,
    #[derivative(Default(value = "0"))]
    rotate: u32,
    reserved: [u32; 5],
}

ioctl!(bad read fbioget_vscreeninfo with 0x4600; FbVarScreeninfo);

pub fn command() -> io::Result<()> {
    let file = File::open("/dev/fb0")?;
    let fd = file.as_raw_fd();
    let mut fb_var_screeninfo = FbVarScreeninfo::default();
    fb_var_screeninfo.reserved = [0; 5];
    fb_var_screeninfo.red = FbBitfield::default();
    fb_var_screeninfo.green = FbBitfield::default();
    fb_var_screeninfo.blue = FbBitfield::default();
    fb_var_screeninfo.transp = FbBitfield::default();

    let ret = unsafe { fbioget_vscreeninfo(fd, &mut fb_var_screeninfo) };
    println!("returned {}, x = {:?}", ret, fb_var_screeninfo);
    Ok(())
}
