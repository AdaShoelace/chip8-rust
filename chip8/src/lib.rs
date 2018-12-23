#![feature(custom_attribute, /* wasm_custom_section, wasm_import_module*/)]
#![allow(unused_mut)]
extern crate wasm_bindgen;
extern crate rand;

#[macro_use]
extern crate lazy_static;
extern crate js_sys;

mod engine;

use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use engine::chip::Chip;
use js_sys::*;

lazy_static! {
    static ref CHIP: Mutex<Chip> = {
        let mut c = Chip::new();
        Mutex::new(c)
    };
}

#[wasm_bindgen(module = "../www/index")]
extern "C" {
    fn setMainLoop(f: &Closure<FnMut()>);
    fn setVideoBuffer(vid_mem: *const u8);
    fn init();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct ClosureHandle(Closure<FnMut()>);

#[wasm_bindgen]
pub fn run(rom: Uint8Array) -> ClosureHandle {
    let mut chip = Chip::new();

    rom.for_each(&mut |current, index, _array| {
        chip.mem.write(0x200 + index as usize, current)
    });

    init();
    let cb = Closure::wrap(Box::new(move || {
        log("Running!");
        chip.emulate_cycle();
        let ptr = chip.get_vid_mem_ptr();
        setVideoBuffer(ptr);
    }) as Box<FnMut()>);

    setMainLoop(&cb);
    ClosureHandle(cb)
}