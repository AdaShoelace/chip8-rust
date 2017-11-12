use std::process;
use std::env;
use std::fs::*;
use std::io::prelude::*;

fn disassemble(code_buffer: Vec<u8>, pc: usize) {
    let ref mut code: &[u8] = &code_buffer[pc..code_buffer.len()];
    //let ref firstnib: u8 = (code[0] >> 4);

    print!("{:04x} {:02x} {:02x}", pc, code[0], code[1]);
    match code[0] >> 4 {
        0x00 => println!("0 not handled yet"),
        0x01 => println!("1 not handled yet"),
        0x02 => println!("2 not handled yet"),
        0x03 => println!("3 not handled yet"),
        0x04 => println!("4 not handled yet"),
        0x05 => println!("5 not handled yet"),
        0x06 => {
            let reg: u8 = code[0] & 0x0f;
            print!("{:<10} V{:01X},{:#02x}", "MVI", reg, code[1]);
        },
        0x07 => println!("7 not handled yet"),
        0x08 => println!("8 not handled yet"),
        0x09 => println!("9 not handled yet"),
        0x0a => {
            let adresshi:u8 = code[0] & 0x0f;
            print!("{:<10} I,{:#01x}{:02x}", "MVI", adresshi, code[1]);
        },
        0x0b => println!("b not handled yet"),
        0x0c => println!("c not handled yet"),
        0x0d => println!("d not handled yet"),
        0x0e => println!("e not handled yet"),
        0x0f => println!("f not handled yet"),
        _ => println!("Not implemented"),
    };

}

fn main() {
    let args: Vec<String> = env::args().collect();              //Store command line arguments in vector
    let mut f = File::open(&args[0]).expect("File not found");  //Open file of the given path
    let meta_data = f.metadata();                               //Get metadata for file
    let fsize:u64 = meta_data.unwrap().len();                   //Get file size
    
    let mut buffer:Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer).err().expect("Error");

    let pc:usize = 0x200;
    disassemble(buffer, pc);

}
