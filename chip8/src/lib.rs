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
    static ref KEY_MEM: Mutex<Vec<u8>> = {
        let mut c = vec![0u8;16];
        Mutex::new(c)
    };
}

#[wasm_bindgen(module = "../www/index")]
extern "C" {
    fn setMainLoop(f: &Closure<FnMut()>);
    fn setVideoBuffer(vid_mem: *const u8);
    fn getKeyPtr(keys: *mut u8);
    fn getKeys() -> u16;
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
        chip.clear_keys();
        if getKeys() < 17 {
            chip.key[getKeys() as usize] = 1;
        }
        if chip.delay_timer > 0 {
            chip.delay_timer -= 1;
        }
        if chip.sound_timer > 0 {
            chip.sound_timer -= 1;
        }

        log(&format!("W: {}", chip.key[0x5]));

        /*chip.key[0x0] = keys[0x0]; //Key::X
        chip.key[0x1] = keys[0x1]; //Key::Num1
        chip.key[0x2] = keys[0x2]; //Key::Num2
        chip.key[0x3] = keys[0x3]; //Key::Num3

        chip.key[0x4] = keys[0x4]; //Key::Q
        chip.key[0x5] = keys[0x5]; //Key::W
        chip.key[0x6] = keys[0x6]; //Key::E
        chip.key[0x7] = keys[0x7]; //Key::A

        chip.key[0x8] = keys[0x8]; //Key::S
        chip.key[0x9] = keys[0x9]; //Key::D
        chip.key[0xa] = keys[0xa]; //Key::Z
        chip.key[0xb] = keys[0xb]; //Key::C

        chip.key[0xc] = keys[0xc]; //Key::Num4
        chip.key[0xd] = keys[0xd]; //Key::R
        chip.key[0xe] = keys[0xe]; //Key::F
        chip.key[0xf] = keys[0xf]; //Key::V */

        let vid_ptr = chip.get_vid_mem_ptr();
        setVideoBuffer(vid_ptr);
        getKeyPtr(chip.get_key_mem_ptr());
    }) as Box<FnMut()>);

    setMainLoop(&cb);
    ClosureHandle(cb)
}