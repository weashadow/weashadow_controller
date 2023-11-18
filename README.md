# Roadmap

## Get control of the 3d printer hardware
- [x] 12K sceren
- [x] UV light
- [x] Z axis
- [x] Touch screen framebuffer
- [ ] Touch screen tslib
- [ ] 24V usb power

## Custom controller software
- [ ] Read and parse goo file format
- [ ] Http server to provide a web interface
- [ ] A set of UI components to control the printer


# Elegoo Saturn 3 Ultra Custom Firmware 

This repository contains the custom firmware for the Elegoo Saturn 3 Ultra. This firmware is currently under heavy development, and it's designed to provide more control and functionality to your 3D printer. 

Please note that this firmware is not officially supported by Elegoo. Use it at your own risk. I am not responsible for any damages that may occur to your 3D printer as a result of using this custom firmware.

## Command Line Arguments

The firmware can be controlled using the following command line arguments:

- **Move Axis (`--move_axis` or `-m`)**: This argument allows you to move the axis of the printer. The value is multiplied by 0.1mm. For example, a value of 10 would move the axis by 1mm. The default value is 0.

- **Get Current Position (`--position` or `-p`)**: If this argument is used, the current position of the printer's axis is returned.

- **UV Light (`--uv_light` or `-u`)**: This argument allows you to control the UV light of the printer. The value should be between 0-255. The default value is 0.

- **Show Image (`--show_image` or `-s`)**: This argument allows you to display an image located in the `/customer/resource/` directory. You should provide the file path of the `.bin` image file. The default value is an empty string, which means no image will be shown.

## Usage

Here is an example of how you can use these command line arguments:

```
./firmware --move_axis 10 --position --uv_light 128 --show_image "/customer/resource/example.bin"
```

This command will move the axis by 1mm, return the current position, set the UV light to half intensity, and display the image located at `/customer/resource/example.bin`.

## Contributing

As this project is in heavy development, contributions are always welcome. Please feel free to open issues or submit pull requests.

## Disclaimer

Again, please note that I take no responsibility for any damages that might occur to your 3D printer as a result of using this custom firmware. Always use caution when modifying your 3D printer's firmware.