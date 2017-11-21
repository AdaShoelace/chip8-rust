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
use sfml::system::{Time, Clock, Vector2f};
use sfml::graphics::{RenderTarget, RectangleShape, Transformable, Drawable, RenderWindow, Shape,
                     Color};

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const PIXEL: u32 = 20;

fn main() {
    let arg1: String = env::args().nth(1).expect("No arguments given!");

    let mut chip = Chip::new();
    load_rom(arg1, &mut chip);
    chip.print_mem(true);
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

    let mut clock: Clock = Clock::start();
    let mut current_time: Time;
    while window.is_open() {
        while clock.elapsed_time().as_milliseconds() < 100 / 60 {}

        current_time = clock.restart();
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }
        read_keys(&mut chip, &window);
        chip.emulate_cycle();
        let ticks = (current_time.as_milliseconds() / 16) as u8;
        let temp_del = chip.delay_timer as i16;
        let temp_sound = chip.sound_timer as i16;
        let delay_val = (temp_del - ticks as i16) as i16;
        let sound_val = (temp_sound - ticks as i16) as i16;
        chip.delay_timer = if delay_val > 0 { delay_val } else { 0 } as u8;
        chip.delay_timer = if sound_val > 0 { sound_val } else { 0 } as u8;

        if chip.draw {
            chip.draw = false;
            window.clear(&Color::BLACK);
            //casuses index out of bounds error -> panic
            for x in 0..SCREEN_COLUMNS {
                for y in 0..SCREEN_ROWS {
                    if chip.vid_mem[y][x] == 1 {
                        let x_pos = (x * SCALE) as f32;
                        let y_pos = (y * SCALE) as f32;
                        &mut rect.set_position((x_pos, y_pos));
                        window.draw(&rect);
                    }
                }
                window.display();
            }
        }
    }
}

fn read_keys(chip: &mut Chip, window: &RenderWindow) {
    chip.key[0] = Key::Num1.is_pressed();
    chip.key[1] = Key::Num2.is_pressed();
    chip.key[2] = Key::Num3.is_pressed();
    chip.key[3] = Key::Num4.is_pressed();
    chip.key[4] = Key::Q.is_pressed();
    chip.key[5] = Key::W.is_pressed();
    chip.key[6] = Key::E.is_pressed();
    chip.key[7] = Key::R.is_pressed();
    chip.key[8] = Key::A.is_pressed();
    chip.key[9] = Key::S.is_pressed();
    chip.key[10] = Key::D.is_pressed();
    chip.key[11] = Key::F.is_pressed();
    chip.key[12] = Key::Z.is_pressed();
    chip.key[13] = Key::X.is_pressed();
    chip.key[14] = Key::C.is_pressed();
    chip.key[15] = Key::V.is_pressed();
}

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

//wanna figure this out for stopping execution when waiting for keypress
fn delay<T, F>(args: &[T], func: F)
where
    F: Fn(&[T]),
{
    func(args);
}
