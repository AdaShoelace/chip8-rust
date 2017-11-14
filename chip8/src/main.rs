#![allow(dead_code, unused)]
extern crate sdl2;
extern crate rand;

mod chip;
mod ram;
mod utils;


use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut ram = ram::Ram::new();
    let PC = ram::MEM_START;
    
    let arg1:String = env::args().nth(1).expect("No arguments given!");

    let mut f = File::open(arg1).unwrap();
    f.read(&mut ram.mem);

    for i in 0..10 {
        println!("{:4X}", ram.mem[i]); 
    }

}
