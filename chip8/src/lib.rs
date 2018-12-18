#![feature(custom_attribute, /* wasm_custom_section, wasm_import_module*/)]
#![allow(unused_mut)]
extern crate wasm_bindgen;
extern crate rand;

#[macro_use]
extern crate lazy_static;

mod engine;

use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use engine::chip::Chip;
use engine::utils::*;

lazy_static! {
    static ref CHIP: Mutex<Chip> = {
        let mut c = Chip::new();
        Mutex::new(c)
    };
}

#[wasm_bindgen]
pub fn execute_cycle() {
    CHIP.lock().unwrap().emulate_cycle();
}

#[wasm_bindgen]
pub fn get_mem() -> *const [u8; 4096] {
    CHIP.lock().unwrap().mem.get_meta_address()
}

#[wasm_bindgen]
pub fn get_vid_mem() -> *const [[u8; SCREEN_COLUMNS]; SCREEN_ROWS]  {
    &(CHIP.lock().unwrap().vid_mem)
}

#[wasm_bindgen]
pub fn key_pressed(key: u8) {
    CHIP.lock().unwrap().key_pressed(key);
}

#[wasm_bindgen]
pub fn dump_registers() -> String {
    format!{"I: {}", CHIP.lock().unwrap().I}
}

#[wasm_bindgen]
pub fn dump_key_mem() -> String {
    let mut res = String::new(); 
    let len = CHIP.lock().unwrap().key.len();
    for i in 0..len {
        if CHIP.lock().unwrap().key[i] {
            res.push_str(&format!("{},", i));
        }
    }
    res
}

