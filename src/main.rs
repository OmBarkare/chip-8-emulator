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

fn draw(chip: &Chip) {
    print!("\x1B[2J\x1B[1;1H");
    for i in 0..64*32 {
        if i%64 == 0 {
            println!("");
        }
        if chip.display[i] {
            print!("*");
        } else {
            print!(" ");
        }
    }
}