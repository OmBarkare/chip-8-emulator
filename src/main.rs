
mod chip;
use chip::{Chip};
fn main() {
    let mut chip = Chip::init();

    while(true) {
        chip.cycle();
        draw(&chip);
    }
}

fn draw(chip: &Chip) {
    print!("\x1B[2J\x1B[1;1H");
    for i in 0..64*32 {
        if i%64 == 0 {
            println!("");
        }
        if chip.display[i] {
            print!("*");
        }
    }
}