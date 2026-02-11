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



fn main() {

}