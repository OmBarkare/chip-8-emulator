use std::{mem, time::{SystemTime, UNIX_EPOCH}};


pub struct Chip {
    mem: [u8; 4096], //4kB ram
    rv: [u8; 16], //16 8-bit v registers
    ri: u16, //one 16 bit register
    dt: u8, //delay timer
    st: u8, //sound timer
    pc: u16, //program counter
    sp: u8, //stack pointer
    stack: [u16; 16],
    keyboard: [bool; 16], //16 keys, either pressed or not pressed
    pub display: [bool; 64*32], //pixels either on or off
    rand_seed: u32,
}
const hex_sprites: [u8; 80] = [
        0xF0,0x90,0x90,0x90,0xF0,
        0x20,0x60,0x20,0x20,0x70,
        0xF0,0x10,0xF0,0x80,0xF0,
        0xF0,0x10,0xF0,0x10,0xF0,
        0x90,0x90,0xF0,0x10,0x10,
        0xF0,0x80,0xF0,0x10,0xF0,
        0xF0,0x80,0xF0,0x90,0xF0,
        0xF0,0x10,0x20,0x40,0x40,
        0xF0,0x90,0xF0,0x90,0xF0,
        0xF0,0x90,0xF0,0x10,0xF0,
        0xF0,0x90,0xF0,0x90,0x90,
        0xE0,0x90,0xE0,0x90,0xE0,
        0xF0,0x80,0x80,0x80,0xF0,
        0xE0,0x90,0x90,0x90,0xE0,
        0xF0,0x80,0xF0,0x80,0xF0,
        0xF0,0x80,0xF0,0x80,0x80
    ];
impl Chip {
    pub fn init() -> Self {
        let mut chip = Chip {
            mem: [0; 4096],
            rv: [0; 16],
            ri: 0,
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
            keyboard: [false; 16],
            display: [false; 64*32],
            rand_seed: {
                let start = SystemTime::now();
                let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
                since_epoch.as_millis() as u32
            },
        };
        let mut i = 0x22;
        for l in hex_sprites {
            chip.mem[i] = l;
            i += 1;
        }

        chip
    }

    pub fn fetch(&mut self) -> u16 {
        let mut opcode: u16 = self.mem[self.pc as usize] as u16;
        opcode = opcode << 8;
        opcode |= self.mem[(self.pc + 1) as usize] as u16;

        self.pc += 2;
        opcode
    }

    pub fn random_byte(&mut self) -> u8 {
        self.rand_seed = self.rand_seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.rand_seed >> 24) as u8
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, inst) in program.iter().enumerate() {
            self.mem[0x200 + i] = *inst;
        }
    }


    pub fn cycle(&mut self) {
        let opcode = self.fetch();

        let m = (opcode & 0xF000) >> 12; // highest 4 bits
        let x = (opcode & 0x0F00) >> 8; // lower 4 bits of higher byte
        let y = (opcode & 0x00F0) >> 4; // higher 4 bits of lower byte
        let n = (opcode & 0x000F); // lowest 4 bits
        let kk = (opcode & 0x00FF); // lower 8 bits
        let nnn = (opcode & 0x0FFF); // lower 12 bits

        match (m, x, y, n) {
            
            (0, 0, 0xE, 0) => { //CLS clear display
                self.display = [false; 64*32];
            }

            (0, 0, 0xE, 0xE) => { //RET return from subroutine
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }

            (0x1, _, _, _) => { // JP addr jump pc to address nnn
                self.pc = nnn;
            }

            (0x2, _, _, _) => { // CALL addr call subroutine at nnn
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn;
            }

            (0x3, _, _, _) => { // SE Vx, kk skip next instruction of Vx == kk
                if(self.rv[x as usize] == kk as u8) {
                    self.pc += 2;
                }
            }

            (0x4, _, _, _) => { // SNE Vx, kk skip next instruction if Vx != kk
                if(self.rv[x as usize] != kk as u8) {
                    self.pc += 2;
                }
            }

            (0x5, _, _, 0x0) => { // SE Vx, Vy
                if(self.rv[x as usize] != kk as u8) {
                    self.pc += 2;
                }
            }

            (0x6, _, _, _) => { // LD Vx, kk load kk to Vx
                self.rv[x as usize] = kk as u8;
            }

            (0x7, _, _, _) => { // ADD Vx, kk set Vx = Vx + kk
                self.rv[x as usize] = (kk as u8) + self.rv[x as usize];
            }

            (0x8, _, _, 0x0) => { // LD Vx, Vy  Vx = Vy
                self.rv[x as usize] = self.rv[y as usize];
            }

            (0x8, _, _, 0x1) => { // OR Vx, Vy  Vx = Vx OR Vy
                self.rv[x as usize] = self.rv[x as usize] | self.rv[y as usize];
            }

            (0x8, _, _, 0x2) => { // AND Vx, Vy  Vx = Vx AND Vy
                self.rv[x as usize] = self.rv[x as usize] & self.rv[y as usize];
            }

            (0x8, _, _, 0x3) => { // XOR Vx, Vy  Vx = Vx XOR Vy
                self.rv[x as usize] = self.rv[x as usize] ^ self.rv[y as usize];
            }

            (0x8, _, _, 0x4) => { // ADD Vx, Vy  Vx = Vx + Vy, set VF = carry
                let p = x as u16;
                let q = y as u16;
                if p+q > 255 {
                    self.rv[0xF] = 1;
                } else {
                    self.rv[0xF] = 0;                    
                }
                self.rv[x as usize] = ((p+q) & 0x00FF) as u8;
            }

            (0x8, _, _, 0x5) => { // SUB Vx, Vy  Vx = Vx - Vy, VF = NOT borrow
                let vx = self.rv[x as usize];
                let vy = self.rv[y as usize];

                if(vx > vy) {
                    self.rv[0xF] = 1;
                } else {
                    self.rv[0xF] = 0;
                }

                self.rv[x as usize] = self.rv[x as usize].wrapping_sub(self.rv[y as usize])
            }
            
            (0x8, _, _, 0x6) => { // SHR Vx shift right Vx, store LSb in VF

                if (self.rv[x as usize] & 0x01) == 1 {
                    self.rv[0xF] = 1;
                } else {
                    self.rv[0xF] = 0;
                }

                self.rv[x as usize] = self.rv[x as usize] >> 1;
            }

            (0x8, _, _, 0x7) => { // SUBN Vx, Vy  Vx = Vy - Vx, VF = NOT borrow
                let vx = self.rv[x as usize];
                let vy = self.rv[y as usize];

                if(vy > vx) {
                    self.rv[0xF] = 1;
                } else {
                    self.rv[0xF] = 0;
                }

                self.rv[x as usize] = self.rv[y as usize].wrapping_sub(self.rv[x as usize])
            }

            (0x8, _, _, 0xE) => { // SHL Vx shilft left Vx, store MSb in VF

               if (self.rv[x as usize] & 0x10) >> 7 == 1 {
                    self.rv[0xF] = 1;
                } else {
                    self.rv[0xF] = 0;
                }

                self.rv[x as usize] = self.rv[x as usize] << 1;
            }

            (9, _, _, 0x0) => { // skip next if Vx != Vy
                if self.rv[x as usize] != self.rv[y as usize] {
                    self.pc += 2;
                }
            }

            (0xA, _, _, _) => { // LD I, nnn set ri to nnn
                self.ri = nnn;
            }

            (0xB, _, _, _) => { // JP V0, nnn jump to location nnn + V0
                self.pc = self.rv[0] as u16 + nnn;
            }

            (0xC, _, _, _) => { // random byte AND kk
                let rnd = self.random_byte();
                self.rv[x as usize] = kk as u8 & rnd;
            }

            (0xD, _, _, _) => { // DRW Vx, Vy, n display n-byte sprite starting at location in ri, to coordinates Vx, Vy
                let vx = self.rv[x as usize] as usize;
                let vy = self.rv[y as usize] as usize;
                let sprite_hieght = n;
                
                for h in 0..sprite_hieght { // reads n bytes, one byte is one line
                    let line = self.mem[(self.ri + h) as usize];
                    for p in 0..8 { // writes each pixel of byte individually
                        let pix = ((line << p) & 0x80) >> 7;
                        let mut currVx = vx;
                        let mut currVy = vy;
                        currVx = (currVx + p) % 64; // %64 wraps the sprite aruond the width
                        currVy = (currVy + h as usize) % 32; // %32 wraps sprite around the height

                        let curr_coor_linear = (currVy * 64) + currVx;

                        if pix == 1 && self.display[curr_coor_linear] { // checking collision
                            self.rv[0xF] = 1;
                        }

                        let pix_bool = if pix == 1 {true} else {false};

                        self.display[curr_coor_linear] ^= pix_bool;
                    }
                }
            }

            (0xE, _, 0x9, 0xE) => { // SKP Vx, skip if key with value of Vx is pressed
                if self.keyboard[self.rv[x as usize] as usize] {
                    self.pc += 2;
                }
            }

            (0xE, _, 0xA, 0x1) => { // SKNP Vx, skip if key with value of Vx is not pressed
                if !self.keyboard[self.rv[x as usize] as usize] {
                    self.pc += 2;
                }
            }

            (0xF, _, 0x0, 0x7) => { // LD Vx, DT load dt into Vx
                self.rv[x as usize] = self.dt;
            }

            (0xF, _, 0x0, 0xA) => { // LD Vx, K wait for keypress, load that value into Vx
                let mut key_pressed = false;

                for i in 0..16 {
                    if self.keyboard[i] {
                        key_pressed = true;
                        self.rv[x as usize] = i as u8;
                        break;
                    }
                }

                if !key_pressed {
                    self.pc -= 2;
                }
            }

            (0xF, _, 0x1, 0x5) => { // LD DT, Vx load dt into Vx
                self.dt = self.rv[x as usize];
            }

            (0xF, _, 0x1, 0x8) => { // LD Vx, ST load st into Vx
                self.st = self.rv[x as usize];
            }

            (0xF, _, 0x1, 0xE) => { // ADD I, Vx, add I = I + Vx
                self.ri = self.ri + self.rv[x as usize] as u16;
            }

            (0xF, _, 0x2, 0x9) => { // set I = location of sprite for digit in Vx
                self.ri = (0x22 + self.rv[x as usize] * 5) as u16;
            }

            (0xF, _, 0x3, 0x3) => { // set I = location of sprite for digit in Vx
                let mut num = self.rv[x as usize];
                self.mem[self.ri as usize + 2] = num % 10;
                num /= 10;
                self.mem[self.ri as usize + 1] = num % 10;
                num /= 10;
                self.mem[self.ri as usize] = num % 10;
            }

            (0xF, _, 0x5, 0x5) => {
                for i in 0..=x {
                    self.mem[(self.ri + i) as usize] = self.rv[i as usize];
                }
            }

            (0xF, _, 0x6, 0x5) => {
                for i in 0..=x {
                    self.rv[i as usize] = self.mem[(self.ri + i) as usize];
                }
            }

            default => {

            }
        }

    }
}

