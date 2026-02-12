struct Chip {
    mem: [u8; 4096], //4kB ram
    rv: [u8; 16], //16 8-bit v registers
    ri: u16, //one 16 bit register
    dt: u8, //delay timer
    st: u8, //sound timer
    pc: u16, //program counter
    sp: u8, //stack pointer
    stack: [u16; 16],
    keyboard: [bool; 16], //16 keys, either pressed or not pressed
    display: [bool; 64*32], //pixels either on or off
}

impl Chip {
    pub fn init() -> Self {
        Chip {
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
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let mut instruction: u16 = self.mem[self.pc as usize] as u16;
        instruction = instruction << 8;
        instruction |= self.mem[(self.pc + 1) as usize] as u16;

        self.pc += 2;
        instruction
    }
}


fn main() {

}