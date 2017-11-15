#![allow(non_snake_case)]

use rand::Rng;
use rand::thread_rng;
use ram::Ram;
use utils::*;


pub const SCREEN_COLUMNS: usize = 64;
pub const SCREEN_ROWS: usize = 32;

pub struct Chip {
    pub I: u16,
    pub mem: Ram,
    pub V: [u8; 16],
    pub PC: usize,
    pub SP: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub vid_mem: [u8; SCREEN_COLUMNS * SCREEN_ROWS],
    pub stack: [u16; 16],
}

impl Chip {
    pub fn new() -> Chip {
        let chip = Chip {
            I: 0,
            mem: Ram::new(),
            V: [0; 16],
            PC: 0x200,
            delay_timer: 60,
            SP: 0,
            sound_timer: 60,
            vid_mem: [0; SCREEN_COLUMNS * SCREEN_ROWS],
            stack: [0; 16],
        };
        chip
    }

    pub fn print_mem(&self) {
        self.mem.print();
    }

    pub fn execute(&self, opcode: u16) {}

    fn decode_00E0(&mut self, opcode: u16) {
        for i in 0..self.vid_mem.len() {
            self.vid_mem[i] = 0;
        } 
        //redraw_screen();
    }

    //JUMP to NNN
    fn decode_1NNN(&mut self, opcode: u16) {
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
        self.V[get_X(opcode) as usize] = get_NN(opcode) as u8;
    } 

    fn decode_7XNN(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] += get_NN(opcode) as u8;
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
        let res = self.V[get_X(opcode) as usize] + self.V[get_Y(opcode) as usize] as u8;
        self.V[get_X(opcode) as usize] = res;
        self.V[0xf] = if res >= 0x100 { 1 } else { 0  };
    }

    fn decode_8XY5(&mut self, opcode: u16) {
        let res = self.V[get_X(opcode) as usize] as i8 - self.V[get_Y(opcode) as usize] as i8;
        self.V[get_X(opcode) as usize] = res as u8;
        self.V[0xf] = if res >= 0 { 1 } else { 0  };
    }

    fn decode_8XY6(&mut self, opcode: u16) {
        let lsb = self.V[get_X(opcode) as usize] & 1;
        let res = self.V[get_X(opcode) as usize] >> 1;
        self.V[get_X(opcode) as usize] = res;
        self.V[get_Y(opcode) as usize] = res;
        self.V[0xf] = lsb;
    }

    fn decode_8XY7(&mut self, opcode: u16) {
        let res = self.V[get_Y(opcode) as usize] as i8 - self.V[get_X(opcode) as usize] as i8;
        self.V[get_X(opcode) as usize] = res as u8;
        self.V[0xf] = if res >= 0 { 1 } else { 0  };
    }

    fn decode_8XY8(&mut self, opcode: u16) {
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
        self.V[get_X(opcode) as usize] = rng.next_u32() as u8  & get_NN(opcode) as u8;
    }

    fn decode_DXYN(&mut self, opcode: u16) {
        let mut x = (self.V[get_X(opcode) as usize] % 64) as usize; //Wrap around? 
        let mut y = (self.V[get_Y(opcode) as usize] % 32) as usize; 
        let n = get_N(opcode);
        let mut flipped = false;

        for j in 0..n {
            let mut px = self.mem.read(self.I as usize + j as usize) as u8;
            y += 1usize;  
            
            for k in 0..8 {
                let mut p_bit: u8 = (px >> k) & 1;
                if self.vid_mem[x + k + y * 64] == 1 && p_bit == 1 {
                    flipped |= true;
                }
                self.vid_mem[x + k + y * 64] ^= p_bit;
            }
        }
        self.V[0x0f] = if flipped { 1 } else { 0 };
        //redraw_screen();
    }

    fn decode_FX1E(&mut self, opcode: u16) {
        self.I = (self.I + self.V[get_X(opcode) as usize] as u16) & 0xfff;
    }

    fn decode_FX33(&mut self, opcode: u16) {
        let mut bcd: u8 = self.V[get_X(opcode) as usize] as u8;
        self.mem.write(self.I as usize + 0, bcd / 100);
        self.mem.write(self.I as usize + 1, (bcd / 10) & 10);
        self.mem.write(self.I as usize + 2, bcd % 10);
    }

    fn decode_FX55(&mut self, opcode: u16) {
        let last_reg = get_X(opcode) as usize; 
        for j in 0..last_reg {
            self.mem.write(self.I as usize + j, self.V[j as usize] as u8);
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
}
