use std::fs;
use std::io::Read;

const w: usize = 64;
const h: usize = 32;

pub struct Chip8 {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    index: u16,
    pc: u16,
    gfx: [u8; w * h],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: isize,
    key: [u8; 16],
    pub draw_flag: bool,
}

const font_set: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut c = Chip8 {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            index: 0,
            pc: 0x200,
            gfx: [0; w * h],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: false,
        };

        for i in 0..font_set.len() {
            c.memory[i] = font_set[i];
        }
        c
    }
    pub fn load_game(&mut self, filepath: &str) {
        let mut f = fs::File::open(filepath).expect("can not load rom file");
        let metadata = fs::metadata(filepath).expect("unable to read metadata");
        let mut b = vec![0; metadata.len() as usize];
        f.read(&mut b).expect("buffer overflow");
        for i in 0..b.len() {
            self.memory[512 + i] = b[i];
        }
    }
    pub fn emulate_cycle(&mut self) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8
            | self.memory[(self.pc + 1) as usize] as u16;
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
        let nn: u8 = (self.opcode & 0x00FF) as u8;
        let nnn: u16 = (self.opcode & 0x0FFF) as u16;
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_game() {
        let mut c = Chip8::new();
        c.load_game("Cargo.toml");
        c.emulate_cycle();
        // println!("{:?}", c.memory);
    }
}
