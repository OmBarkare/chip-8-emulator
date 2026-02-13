mod chip;

use minifb::{Window, WindowOptions};
use chip::Chip;


fn main() {
    let mut chip = Chip::init();
    while(true) {
        chip.cycle();
        draw(&chip);
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