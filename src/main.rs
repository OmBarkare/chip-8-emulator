mod chip;

use minifb::{Scale, Window, WindowOptions};
use chip::Chip;


fn main() {
    let mut chip = Chip::init();
    let mut buffer: [u32; 64*32] = [0; 64*32];
    let mut window = Window::new(
        "chip-8", 
        64,
        32,
        WindowOptions {
            scale: Scale::X16,
            resize: false,
            ..WindowOptions::default()
        }
    ).unwrap();

    let program: [u8; 10] = [
        0x60, 0x00, // LD V0, 0
        0x61, 0x00, // LD V1, 0
        0xA0, 0x27, // LD I, 0x22 (Address of your Font '0')
        0xD0, 0x15, // DRW V0, V1, 5 bytes
        0x12, 0x08, // JUMP to 0x208 (Infinite loop)
    ];

    chip.load_program(&program);

    while(true) {
        chip.cycle();
        let mut i = 0;
        for pix_on in chip.display {
            if pix_on {
                buffer[i] = from_u8_rgb(255, 255, 255);                
            } else {
                buffer[i] = from_u8_rgb(0, 0, 0);
            }
            i += 1;
        }
        
        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}