#![feature(custom_attribute, /* wasm_custom_section, wasm_import_module*/)]
extern crate wasm_bindgen;
extern crate rand;

pub mod engine;

use wasm_bindgen::prelude::*;
use engine::chip::RunMode;

pub use engine::chip::Chip; // <--Re-export

static mut chip: Chip = Chip::new();

#[wasm_bindgen]
pub fn execute_cycle() {
    unsafe { chip.emulate_cycle(); }
}