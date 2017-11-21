#![allow(non_snake_case)]

use rand::Rng;
use rand::thread_rng;
use ram::Ram;
use utils::*;



pub struct Chip {
    pub I: u16,
    pub mem: Ram,
    pub V: [u8; 16],
    pub PC: usize,
    pub SP: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub vid_mem: [[u8; SCREEN_COLUMNS]; SCREEN_ROWS],
    pub stack: Vec<u16>,
    pub key: [bool; 16],
    pub draw: bool,
}

impl Chip {
    pub fn new() -> Chip {
        let chip = Chip {
            I: 0,
            mem: Ram::new(),
            V: [0; 16],
            PC: 0x200,
            delay_timer: 0,
            SP: 0,
            sound_timer: 0,
            vid_mem: [[0; SCREEN_COLUMNS]; SCREEN_ROWS],
            stack: Vec::with_capacity(16),
            key: [false; 16],
            draw: false,
        };
        chip
    }

    pub fn print_mem(&self, all: bool) {
        self.mem.print(all);
    }

    fn fetch(&mut self) -> u16 {
        let opcode = self.mem.read(self.PC);
        self.PC += 2;
        opcode
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    fn execute(&mut self, opcode: u16) {
        match (opcode & 0xf000) >> 12 {
            0x0 => {
                match opcode & 0x00ff {
                    0x00e0 => self.decode_00E0(opcode),
                    0x00ee => self.decode_00EE(opcode),
                    _ => {} //self.unimplemented(opcode),
                };
            }
            0x1 => self.decode_1NNN(opcode),
            0x2 => self.decode_2NNN(opcode),
            0x3 => self.decode_3XNN(opcode),
            0x4 => self.decode_4XNN(opcode),
            0x5 => self.decode_5XY0(opcode),
            0x6 => self.decode_6XNN(opcode),
            0x7 => self.decode_7XNN(opcode),
            0x8 => {
                match opcode & 0x000f {
                    0x0 => self.decode_8XY0(opcode),
                    0x1 => self.decode_8XY1(opcode),
                    0x2 => self.decode_8XY2(opcode),
                    0x3 => self.decode_8XY3(opcode),
                    0x4 => self.decode_8XY4(opcode),
                    0x5 => self.decode_8XY5(opcode),
                    0x6 => self.decode_8XY6(opcode),
                    0x7 => self.decode_8XY7(opcode),
                    0xe => self.decode_8XYE(opcode),
                    _ => {} //self.unimplemented(opcode),
                }
            }
            0x9 => self.decode_9XY0(opcode),
            0xa => self.decode_ANNN(opcode),
            0xb => self.decode_BNNN(opcode),
            0xc => self.decode_CXNN(opcode),
            0xd => self.decode_DXYN(opcode),
            0xe => {
                match opcode & 0x00ff {
                    0x9e => self.decode_EX9E(opcode),
                    0xa1 => self.decode_EXA1(opcode),
                    _ => {} //self.unimplemented(opcode),
                }
            }
            0xf => {
                match opcode & 0x00ff {
                    0x07 => self.decode_FX07(opcode),
                    0x0a => self.decode_FX0A(opcode),
                    0x15 => self.decode_FX15(opcode),
                    0x18 => self.decode_FX18(opcode),
                    0x1e => self.decode_FX1E(opcode),
                    0x29 => self.decode_FX29(opcode),
                    0x33 => self.decode_FX33(opcode),
                    0x55 => self.decode_FX55(opcode),
                    0x65 => self.decode_FX65(opcode),
                    _ => {} //self.unimplemented(opcode),
                }
            }
            _ => {}//self.unimplemented(opcode),
        };
    }

    fn decode_0NNN(&mut self, opcode: u16) {
        println!("opcode 0NNN not implemented but used");
    }

    fn decode_00E0(&mut self, opcode: u16) {
        for i in 0..(SCREEN_ROWS - 1) {
            for j in 0..(SCREEN_COLUMNS - 1) {
                self.vid_mem[i][j] = 0;
            }
        }
        //self.draw = true; //is this right?
    }

    fn decode_00EE(&mut self, opcode: u16) {
        self.PC = match self.stack.pop() {
            Some(x) => x as usize,
            None => self.PC,
        };
    }
    //JUMP to NNN
    fn decode_1NNN(&mut self, opcode: u16) {
        self.PC = get_NNN(opcode) as usize;
    }

    fn decode_2NNN(&mut self, opcode: u16) {
        self.stack.push(self.PC as u16);
        self.PC = get_NNN(opcode) as usize;

    }

    fn decode_3XNN(&mut self, opcode: u16) {
        if self.V[get_X(opcode) as usize] == get_NN(opcode) as u8 {
            self.PC += 2;
        }
    }

    fn decode_4XNN(&mut self, opcode: u16) {
        if self.V[get_X(opcode) as usize] != get_NN(opcode) as u8 {
            self.PC += 2;
        }
    }

    fn decode_5XY0(&mut self, opcode: u16) {
        if self.V[get_X(opcode) as usize] == self.V[get_Y(opcode) as usize] {
            self.PC += 2;
        }
    }

    fn decode_6XNN(&mut self, opcode: u16) {
        self.write_to_reg(get_X(opcode) as u8, get_NN(opcode) as u8);
    }

    fn decode_7XNN(&mut self, opcode: u16) {
        let vx = self.V[get_X(opcode) as usize];
        self.V[get_X(opcode) as usize] = vx.wrapping_add(get_NN(opcode) as u8);
    }

    fn decode_8XY0(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] = self.V[get_Y(opcode) as usize];
    }

    fn decode_8XY1(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] |= self.V[get_Y(opcode) as usize];
    }

    fn decode_8XY2(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] &= self.V[get_Y(opcode) as usize];
    }

    fn decode_8XY3(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] ^= self.V[get_Y(opcode) as usize];
    }

    fn decode_8XY4(&mut self, opcode: u16) {
        let res = self.V[get_X(opcode) as usize] as u16 + self.V[get_Y(opcode) as usize] as u16;
        self.V[get_X(opcode) as usize] = res as u8;
        self.V[0xf] = if res > 0x100 { 1 } else { 0 };
    }

    fn decode_8XY5(&mut self, opcode: u16) {
        //let res = self.V[get_X(opcode) as usize] as i8 - self.V[get_Y(opcode) as usize] as i8;
        //self.V[get_X(opcode) as usize] = res as u8;
        //self.V[0xf] = if res >= 0 { 1 } else { 0 };

        let x = self.V[get_X(opcode) as usize] as u8;
        let y = self.V[get_Y(opcode) as usize] as u8;

        if x > y {
            self.V[0xf] = 0;
        } else {
            self.V[0xf] = 1;
        }

        self.V[get_X(opcode) as usize].wrapping_sub(self.V[get_Y(opcode) as usize]);
    }

    fn decode_8XY6(&mut self, opcode: u16) {
        let lsb = self.V[get_X(opcode) as usize] & 1;
        let res = self.V[get_X(opcode) as usize] >> 1;
        self.V[get_X(opcode) as usize] = res;
        self.V[get_Y(opcode) as usize] = res;
        self.V[0xf] = lsb;
    }

    fn decode_8XY7(&mut self, opcode: u16) {
        let vx = self.read_reg(get_X(opcode) as u8);
        let vy = self.read_reg(get_Y(opcode) as u8);

        if vy > vx {
            self.write_to_reg(0xf, 0x1);
        } else {
            self.write_to_reg(0xf, 0x0);
        }

        self.write_to_reg(get_X(opcode) as u8, vy.wrapping_sub(vx));
    }

    fn decode_8XYE(&mut self, opcode: u16) {
        let msb = self.V[get_X(opcode) as usize] >> 7;
        let res = self.V[get_X(opcode) as usize] << 1;
        self.V[get_X(opcode) as usize] = res;
        self.V[get_Y(opcode) as usize] = res;
        self.V[0xf] = msb;
    }

    fn decode_9XY0(&mut self, opcode: u16) {
        if self.V[get_X(opcode) as usize] != self.V[get_Y(opcode) as usize] {
            self.PC += 2;
        }
    }

    fn decode_ANNN(&mut self, opcode: u16) {
        self.I = get_NNN(opcode);
    }

    fn decode_BNNN(&mut self, opcode: u16) {
        let val = (self.V[0] as u16 + get_NNN(opcode)) & 0xfff;
        self.PC = val as usize;
    }

    fn decode_CXNN(&mut self, opcode: u16) {
        let mut rng = thread_rng();
        self.V[get_X(opcode) as usize] = rng.next_u32() as u8 & get_NN(opcode) as u8;
    }

    pub fn decode_DXYN(&mut self, opcode: u16) {
        let col = (self.V[get_X(opcode) as usize] % 64) as usize;
        let row = (self.V[get_Y(opcode) as usize] % 32) as usize;
        let height = get_N(opcode) as usize;
        let mut pixel: u16;
        self.V[0xf] = 0;

        for row_off in 0..height {
            pixel = self.mem.read((self.I as usize + row_off) as usize);
            pixel >>= 8;
            for col_off in 0..8 {
                if pixel & (0x80 >> col_off) > 0 {
                    if self.vid_mem[(row + row_off) % 32][(col + col_off) % 64] == 1 {
                        self.V[0xf] = 1;
                    }
                    self.vid_mem[(row + row_off) % 32][(col + col_off) % 64] ^= 1
                }
            }
        }
        self.draw = true;
    }

    fn decode_EX9E(&mut self, opcode: u16) {
        //skip next instruction if key with value of VX is pressed
        if self.key[self.V[get_X(opcode) as usize] as usize] == true {
            self.PC += 2;
        }
    }

    fn decode_EXA1(&mut self, opcode: u16) {
        //skip next instruction if key with value of VX is NOT pressed
        if self.key[self.V[get_X(opcode) as usize] as usize] == false {
            self.PC += 2;
        }
    }

    fn decode_FX07(&mut self, opcode: u16) {
        //The value of delay timer is placed in VX
        self.V[get_X(opcode) as usize] = self.delay_timer;
    }

    //test fails
    fn decode_FX0A(&mut self, opcode: u16) {
        //wait for a keypress, store the value of the key in VX
        //all execution stops until a key is pressed

        let mut pressed = false;

        for i in 0..self.key.len() {
            if self.key[i] == true {
                print!("Inne");
                &mut self.write_to_reg(get_X(opcode) as u8, i as u8);
                pressed = true;
            }
        }
        if !pressed {
            self.PC -= 2;
        }
    }

    fn decode_FX15(&mut self, opcode: u16) {
        self.delay_timer = self.V[get_X(opcode) as usize] as u8;
    }

    fn decode_FX18(&mut self, opcode: u16) {
        //set sound timer = to VX
        self.sound_timer = self.V[get_X(opcode) as usize];
    }

    fn decode_FX1E(&mut self, opcode: u16) {
        self.I = (self.I + self.V[get_X(opcode) as usize] as u16) & 0xfff;
    }

    fn decode_FX29(&mut self, opcode: u16) {
        //might not be what the manual meant
        self.I = self.mem.read(self.V[get_X(opcode) as usize] as usize);
    }

    fn decode_FX33(&mut self, opcode: u16) {
        let mut bcd: u8 = self.V[get_X(opcode) as usize] as u8;
        self.mem.write(self.I as usize + 0, bcd / 100);
        self.mem.write(self.I as usize + 1, (bcd / 10) % 10);
        self.mem.write(self.I as usize + 2, bcd % 10);
    }

    fn decode_FX55(&mut self, opcode: u16) {
        let last_reg = get_X(opcode) as usize;
        for j in 0..last_reg {
            self.mem.write(
                self.I as usize + j,
                self.V[j as usize] as u8,
            );
            self.I += 1;
        }
    }

    fn decode_FX65(&mut self, opcode: u16) {
        let last_reg = get_X(opcode) as usize;
        for j in 0..last_reg {
            self.V[j] = self.mem.read(self.I as usize + j) as u8;
            self.I += 1;
        }
    }

    fn write_to_reg(&mut self, i: u8, val: u8) {
        self.V[i as usize] = val;
    }

    fn read_reg(&mut self, i: u8) -> u8 {
        self.V[i as usize]
    }

    fn unimplemented(&self, opcode: u16) {
        println!("Unimplemented opcode: {:#04X}", opcode);
    }
}

#[cfg(test)]
mod tests {
    use utils::{SCREEN_COLUMNS, SCREEN_ROWS};
    use chip::*;
    #[test]
    fn frame_buf() {
        let frm_buf: [[u8; SCREEN_COLUMNS]; SCREEN_ROWS] = [[0; SCREEN_COLUMNS]; SCREEN_ROWS];

        let mut c = Chip::new();

        let opcode: u16 = 0xd004;
        assert!(c.I == 0);
        c.mem.write((c.I + 0) as usize, 0b00011000u8);
        c.mem.write((c.I + 1) as usize, 0b00100100u8);
        c.mem.write((c.I + 2) as usize, 0b00100100u8);
        c.mem.write((c.I + 3) as usize, 0b00011000u8);

        c.decode_DXYN(opcode);

        print_buf(&c.vid_mem);

    }

    #[test]
    fn test_FX0A() {
        let mut c = Chip::new();
        let opcode: u16 = 0xF30A;
        c.key[5] = 1;
        c.PC = 2;
        c.decode_FX0A(opcode);
        assert!(c.PC == 2 as usize);
        assert!(c.read_reg(get_X(opcode) as u8) == c.key[5]);

    }

    fn print_buf(buf: &[[u8; SCREEN_COLUMNS]; SCREEN_ROWS]) {
        for k in 0..SCREEN_COLUMNS + 2 {
            print!{"-"};
        }
        println!();
        for i in 0..SCREEN_ROWS {
            print!("|");
            for j in 0..SCREEN_COLUMNS {
                print!("{}", buf[i][j]);
            }
            println!("|");
        }
        for k in 0..SCREEN_COLUMNS + 2 {
            print!{"-"};
        }
    }
}
