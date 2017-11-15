#![allow(dead_code, unused)]
extern crate sdl2;
extern crate rand;

mod chip;
mod ram;
mod utils;


use std::env;
use std::fs::File;
use std::fs;
use std::io::Read;

use chip::Chip;

fn main() {
    let arg1:String = env::args().nth(1).expect("No arguments given!");

    let mut chip = Chip::new();
    load_rom(arg1, &mut chip);
    //chip.print_mem();
}

fn load_rom(filename: String, chip: &mut Chip) {
    let mut f = File::open(&filename).unwrap();
    let meta = fs::metadata(filename).unwrap();
    let file_length = meta.len();
    let mut buf: Vec<u8> =  Vec::with_capacity(file_length as usize);
    f.read_to_end(&mut buf).expect("File not found");
    for i in 0..10 {
        println!("{:2X}", buf[i]);
    }
    chip.mem.write_rom(&buf);
}
