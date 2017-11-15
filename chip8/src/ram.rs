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

    pub fn print(&self) {
        let mut i = 0x200;
        while i < self.get_length() - 2000 {
            println!(
                "Addr: {:4X} opcode: {:2X} {:2X}",
                i,
                self.mem[i],
                self.mem[i + 1]
            );
            i += 2;
        }
    }

    pub fn write_rom(&mut self, rom: &Vec<u8>) {
        let mut j = 0x200;
        for i in rom.iter() {
            self.mem[j] = *i;
            j += 1;
        }

        for i in 0x200..0x20a {
            println!("{:02X}", self.mem[i]);
        }
    }

    pub fn read(&self, addr: usize) -> u16 {
        let mut ret: u16 = self.mem[addr] as u16;
        ret = (ret << 8) | self.mem[addr + 1] as u16;
        ret as u16
    }

    pub fn write(&mut self, addr: usize, data: u8) {
        self.mem[addr] = data;
    }
}
