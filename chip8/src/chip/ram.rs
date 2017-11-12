#![allow(dead_code, unused_variables)]
pub const MEM_START: u16 = 0x200;

pub struct Ram {
    pub mem: [u8; 4096],
}

impl Ram {
    pub fn new() -> Ram {
        let ram = Ram { mem: [0; 4096] };
        ram
    }

    pub fn get_length(&self) -> usize {
        self.mem.len()
    }

    pub fn read(&self, addr: usize) -> u16 {
        let mut ret: u16 = self.mem[addr] as u16;
        ret = (ret << 8) | self.mem[addr + 1] as u16;
        ret as u16
    }

    pub fn write(&mut self, addr: usize, data: u16) {
        let hi_byte: u8 = (data & 0xff00) as u8;
        let low_byte: u8 = (data & 0x00ff) as u8;
        self.mem[addr] = hi_byte;
        self.mem[addr + 1] = low_byte;
    }
}
