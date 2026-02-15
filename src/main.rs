mod chip;

use chip::Chip;
use minifb::{Key, Scale, Window, WindowOptions};
use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

fn main() {
    let mut chip = Chip::init();
    let mut buffer: [u32; 64 * 32] = [0; 64 * 32];
    let mut execs = 0;
    let mut window = Window::new(
        "chip-8",
        64,
        32,
        WindowOptions {
            scale: Scale::X16,
            resize: false,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let mut program_file = File::open("spaceinvaders.ch8").unwrap();
    let mut program = Vec::new();
    program_file.read_to_end(&mut program).unwrap();

    chip.load_program(&program);

    // let mut last_instant_exec = Instant::now();
    let mut last_time_since_updated_ticker = Instant::now();
    let mut last_time_since_cycle = Instant::now();
    loop {

        if last_time_since_cycle.elapsed() > Duration::from_nanos(2_000_000) { //execute at 500hz

            chip.cycle();

            //map display to window buffer
            let mut i = 0;
            for pix_on in chip.display {
                if pix_on {
                    buffer[i] = from_u8_rgb(255, 255, 255);
                } else {
                    buffer[i] = from_u8_rgb(0, 0, 0);
                }
                i += 1;
            }
            chip.keyboard = [false; 16];
            for key in window.get_keys() {
                match key {
                    Key::Key1 => chip.keyboard[0] = true,
                    Key::Key2 => chip.keyboard[1] = true,
                    Key::Key3 => chip.keyboard[2] = true,
                    Key::Key4 => chip.keyboard[3] = true,
                    Key::Q => chip.keyboard[4] = true,
                    Key::W => chip.keyboard[5] = true,
                    Key::E => chip.keyboard[6] = true,
                    Key::R => chip.keyboard[7] = true,
                    Key::A => chip.keyboard[8] = true,
                    Key::S => chip.keyboard[9] = true,
                    Key::D => chip.keyboard[10] = true,
                    Key::F => chip.keyboard[11] = true,
                    Key::Z => chip.keyboard[12] = true,
                    Key::X => chip.keyboard[13] = true,
                    Key::C => chip.keyboard[14] = true,
                    Key::V => chip.keyboard[15] = true,
                    _default => {}
                }
            }
            last_time_since_cycle = Instant::now();
        }

        // update timers
        if last_time_since_updated_ticker.elapsed() > Duration::from_nanos(16_666_666) { //keep screen refresh rate at 60hz
            chip.tick_timers();
            window.update_with_buffer(&buffer, 64, 32).unwrap();
            last_time_since_updated_ticker = Instant::now();
        }

        // execs += 1;
        // if last_instant_exec.elapsed() > Duration::from_secs(1) {
        //     println!("execs per sec: {}",execs);
        //     last_instant_exec = Instant::now();
        //     execs = 0;
        // }

    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
