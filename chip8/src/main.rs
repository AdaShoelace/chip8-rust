#![allow(dead_code, unused)]
extern crate sfml;
extern crate rand;

mod chip;
mod ram;
mod utils;

use std::env;
use std::fs::File;
use std::fs;
use std::io::Read;

use chip::Chip;
use utils::*;

use sfml::window::{VideoMode, ContextSettings, Event, Key, Style};
use sfml::system::{Clock, Vector2f};
use sfml::graphics::{RenderTarget, RectangleShape, Transformable, Drawable, RenderWindow, Shape,
                     Color};

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const PIXEL: u32 = 20;

fn main() {
    let arg1: String = env::args().nth(1).expect("No arguments given!");

    let mut chip = Chip::new();
    load_rom(arg1, &mut chip);
    //chip.print_mem();

    //Window etc
    let width = (SCREEN_COLUMNS * SCALE) as u32;
    let height = (SCREEN_ROWS * SCALE) as u32;
    let mut window = RenderWindow::new(
        (width, height),
        "Chip8 Emulator",
        Style::CLOSE,
        &Default::default(),
    );

    let mut rect = RectangleShape::new();
    rect.set_size((SCALE as f32, SCALE as f32));
    rect.set_fill_color(&Color::WHITE);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }
        chip.execute();
        window.clear(&Color::BLACK);
        //casuses index out of bounds error -> panic
        for x in 0..SCREEN_COLUMNS {
            for y in 0..SCREEN_ROWS {
                if chip.vid_mem[y * SCREEN_COLUMNS + x] == 1 {
                    let x_pos = (x * SCALE) as f32;
                    let y_pos = (y * SCALE) as f32;
                    &mut rect.set_position((x_pos, y_pos));
                    window.draw(&rect);
                }
            }
        }
        window.display();
    }
}

/*
fn draw_buffer(buf: &[u8;64*32], win: &mut RenderWindow, rect: &mut RectangleShape<'static>) {
    //take in renderwindow and buffer

    for x in 0..SCREEN_COLUMNS {
        for y in 0..SCREEN_ROWS {
            if buf[x * SCREEN_COLUMNS + y] == 1 {
                let x_pos = (x*SCALE) as f32;
                let y_pos = (y*SCALE) as f32;
                &mut rect.set_position((x_pos, y_pos));
                win.draw(&rect);
            }
        }
    }
}
*/

fn load_rom(filename: String, chip: &mut Chip) {
    let mut f = File::open(&filename).unwrap();
    let meta = fs::metadata(filename).unwrap();
    let file_length = meta.len();
    let mut buf: Vec<u8> = Vec::with_capacity(file_length as usize);
    f.read_to_end(&mut buf).expect("File not found");

    println!("Debug print of buffer:");
    for i in 0..10 {
        println!("{:#04X}", buf[i]);
    }
    chip.mem.write_rom(&buf);
}
