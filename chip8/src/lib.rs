use rand::Rng;
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
    pub gfx: [u8; w * h],
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

        // Decode Opcode
        // https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x000F {
                    0x0000 => {
                        // 00E0 Clear screen
                        for i in 0..self.gfx.len() {
                            self.gfx[i] = 0;
                        }
                        self.pc += 2;
                        self.draw_flag = true;
                    }
                    0x000E => {
                        // 00EE Returns from a subroutine
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                        self.pc += 2;
                    }
                    _ => println!("unk {:x}", self.opcode),
                }
            }
            0x1000 => {
                // 1NNN Jumps to address NNN
                self.pc = nnn;
            }
            0x2000 => {
                // 2NNN Calls subroutine at NNN
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            0x3000 => {
                // 3XNN Skips the next instruction if VX equals NN. (Usually
                // the next instruction is a jump to skip a code block)
                if self.v[x] == nn {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x4000 => {
                // 4XNN Skips the next instruction if VX doesn't equal NN.
                // (Usually the next instruction is a jump to skip a code
                // block)
                if self.v[x] != nn {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x5000 => {
                // 5XY0 Skips the next instruction if VX equals VY. (Usually
                // the next instruction is a jump to skip a code block)
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x6000 => {
                // 6XNN Sets VX to NN
                self.v[x] = nn;
                self.pc += 2;
            }
            0x7000 => {
                // 7XNN Adds NN to VX. (Carry flag is not changed)
                self.v[x] += nn;
                self.pc += 2;
            }
            0x8000 => {
                match self.opcode & 0x000F {
                    0x0000 => {
                        // 0x8XY0 Sets VX to the value of VY
                        self.v[x] = self.v[y];
                        self.pc += 2;
                    }
                    0x0001 => {
                        // 0x8XY1 Sets VX to VX or VY. (Bitwise OR operation)
                        self.v[x] = (self.v[x] | self.v[y]);
                        self.pc += 2;
                    }
                    0x0002 => {
                        // 0x8XY2 Sets VX to VX and VY. (Bitwise AND operation)
                        self.v[x] = (self.v[x] & self.v[y]);
                        self.pc += 2;
                    }
                    0x0003 => {
                        // 0x8XY3 Sets VX to VX xor VY
                        self.v[x] = (self.v[x] ^ self.v[y]);
                        self.pc += 2;
                    }
                    0x0004 => {
                        // 0x8XY4 Adds VY to VX. VF is set to 1 when there's a
                        // carry, and to 0 when there isn't
                        if self.v[y] > (0xFF - self.v[x]) {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[x] += self.v[y];
                        self.pc += 2;
                    }
                    0x0005 => {
                        // 0x8XY5 VY is subtracted from VX. VF is set to 0 when
                        // there's a borrow, and 1 when there isn't
                        if self.v[x] > self.v[y] {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[x] -= self.v[y];
                        self.pc += 2;
                    }
                    0x0006 => {
                        // 0x8XY6 Stores the least significant bit of VX in VF
                        // and then shifts VX to the right by 1
                        if self.opcode & 0x1 >= 1 {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[x] = self.v[x] >> 1;
                        self.pc += 2;
                    }
                    0x0007 => {
                        // 0x8XY7 Sets VX to VY minus VX. VF is set to 0 when
                        // there's a borrow, and 1 when there isn't
                        if self.v[y] > self.v[x] {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[x] = self.v[y] - self.v[x];
                        self.pc += 2;
                    }
                    0x000E => {
                        // 0x8XYE Stores the most significant bit of VX in VF
                        // and then shifts VX to the left by 1
                        if self.opcode & 0x80 == 0x80 {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[x] = self.v[x] << 1;
                        self.pc += 2;
                    }
                    _ => println!("unk {:x}", self.opcode),
                }
            }
            0x9000 => {
                // 9XY0 Skips the next instruction if VX doesn't equal VY.
                // (Usually the next instruction is a jump to skip a code
                // block)
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0xA000 => {
                // ANNN set index to NNN position
                self.index = nnn;
                self.pc += 2;
            }
            0xB000 => {
                // BNNN Jumps to the address NNN plus V0
                self.pc = nnn + self.v[0] as u16;
                self.pc += 2;
            }
            0xC000 => {
                // CXNN Sets VX to the result of a bitwise and operation on a
                // random number (Typically: 0 to 255) and NN
                let mut rng = rand::thread_rng();
                let r: u8 = rng.gen_range(0, 255);
                self.v[x] = r & nn;
                self.pc += 2;
            }
            0xD000 => {
                // DXYN Draws a sprite at coordinate (VX, VY) that has a width
                // of 8 pixels and a height of N+1 pixels. Each row of 8 pixels
                // is read as bit-coded starting from memory location I; I
                // value doesn’t change after the execution of this
                // instruction. As described above, VF is set to 1 if any
                // screen pixels are flipped from set to unset when the sprite
                // is drawn, and to 0 if that doesn’t happen
                let heigh = self.opcode & 0x000F;
                let mut pixel: u8;
                self.v[0xF] = 0;
                for yline in 0..heigh {
                    pixel = self.memory[(self.index + yline) as usize];
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            let pos = (self.v[x] as u16 + xline) as usize
                                + (self.v[y] as u16 + yline) as usize * w;
                            if pos >= 2048 {
                                break;
                            }
                            if self.gfx[pos] == 1 {
                                self.v[0xF] = 1;
                            } else {
                                self.v[0xF] ^= 1;
                            }
                        }
                    }
                }

                self.draw_flag = true;
                self.pc += 2;
            }
            0xE000 => {
                match self.opcode & 0x00FF {
                    0x009E => {
                        // EX9E Skips the next instruction if the key stored in
                        // VX is pressed. (Usually the next instruction is a
                        // jump to skip a code block)
                        if self.key[self.v[x] as usize] != 0 {
                            self.pc += 2;
                        }
                        self.pc += 2;
                    }
                    0x00A1 => {
                        // EXA1 Skips the next instruction if the key stored in
                        // VX isn't pressed. (Usually the next instruction is a
                        // jump to skip a code block)
                        if self.key[self.v[x] as usize] != 1 {
                            self.pc += 2;
                        }
                        self.pc += 2;
                    }
                    _ => println!("unk {:x}", self.opcode),
                }
            }
            0xF000 => {
                match self.opcode & 0x00FF {
                    0x0007 => {
                        // FX07 Sets VX to the value of the delay timer
                        self.v[x] = self.delay_timer;
                        self.pc += 2;
                    }
                    0x000A => {
                        // FX0A A key press is awaited, and then stored in VX.
                        // (Blocking Operation. All instruction halted until
                        // next key event)
                        let mut pressed: bool = false;
                        for i in 0..16 {
                            if self.key[i] == 1 {
                                self.v[x] = i as u8;
                                pressed = true;
                            }
                        }
                        if pressed {
                            self.pc += 2;
                        }
                    }
                    0x0015 => {
                        // FX15 Sets the delay timer to VX
                        self.delay_timer = self.v[x];
                        self.pc += 2;
                    }
                    0x0018 => {
                        // FX18 Sets the sound timer to VX
                        self.sound_timer = self.v[x];
                        self.pc += 2;
                    }
                    0x001E => {
                        // FX1E Adds VX to I. VF is not affected
                        self.index += self.v[x] as u16;
                        self.pc += 2;
                    }
                    0x0029 => {
                        // FX29 Sets I to the location of the sprite for the
                        // character in VX. Characters 0-F (in hexadecimal) are
                        // represented by a 4x5 font
                        self.index = self.v[x] as u16 * 5;
                        self.pc += 2;
                    }
                    0x0033 => {
                        self.memory[self.index as usize] = self.v[x] / 100;
                        self.memory[self.index as usize + 1] = (self.v[x] / 10) % 10;
                        self.memory[self.index as usize + 2] = (self.v[x] / 100) % 10;
                        self.pc += 2;
                    }
                    0x0055 => {
                        // FX55 Stores V0 to VX (including VX) in memory
                        // starting at address I. The offset from I is
                        // increased by 1 for each value written, but I itself
                        // is left unmodified
                        for i in 0..(x + 1) {
                            self.memory[self.index as usize + i] = self.v[i];
                        }
                        self.pc += 2;
                    }
                    0x0064 => {
                        // 0xFX65 Fills V0 to VX (including VX) with values
                        // from memory starting at address I. The offset from I
                        // is increased by 1 for each value written, but I
                        // itself is left unmodified
                        for i in 0..(x + 1) {
                            self.v[i] = self.memory[self.index as usize + i];
                        }
                        self.pc += 2;
                    }
                    _ => println!("unk {:x}", self.opcode),
                }
            }
            _ => println!("opc {:x}", self.opcode),
        }
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
    }
}
