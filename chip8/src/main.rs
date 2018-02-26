#![allow(dead_code, unused)]
extern crate sfml;
extern crate rand;
extern crate nfd;
extern crate getopts;

mod chip;
mod ram;
mod utils;
mod debugger;

use debugger::Debugger;
use std::path::Path;
use getopts::{Options, Matches};

use std::env;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::io;
use std::time::{Duration, Instant};
use std::thread;
use std::u16;

use chip::Chip;
use utils::{SCREEN_COLUMNS, SCREEN_ROWS, SCALE};

use sfml::window::{VideoMode, ContextSettings, Event, Key, Style};
use sfml::system::{Time, Clock, Vector2f};
use sfml::graphics::{RenderTarget, RectangleShape, Transformable, Drawable, RenderWindow, Shape,
Color};

use nfd::Response;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const PIXEL: u32 = 20;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("r", "rom", "path to rom", "NAME");
    opts.optopt("b", "break-point", "adress to break on (in hexadecimal)", "ADRESS");
    opts.optflag("d", "debug", "run in debug mode");
    opts.optflag("h", "help", "print this help menu");

    let program = args[0].clone();

    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut step: bool = true;
    let mut debugger: Option<Debugger> = None;

    if matches.opt_present("d") {
        //step = false;
        debugger = Some(Debugger::new());
    }

    let rom: String = if !matches.opt_strs("r").is_empty() { matches.opt_strs("r")[0].clone() } else {
        let file_result = nfd::open_file_dialog(Some("ch8"), env::current_dir().unwrap().as_path().to_str()).unwrap_or_else(|e| {
            panic!(e);
        });

        match file_result {
            Response::Okay(file_path) => file_path,
            Response::OkayMultiple(files) => {
                println!("Choose one file only");
                panic!();
            },
            _ => panic!(),
        }
    };

    let break_point: Option<String> = if matches.opt_defined("b") { Some(matches.opt_strs("b")[0].clone()) } else { None };

    let mut chip = Chip::new();
    load_rom(rom, &mut chip);
    //chip.print_mem(true);

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

    let mut last_instruction = Instant::now();
    let mut last_screen = Instant::now();
    let mut delay_duration = Instant::now();

    while window.is_open() {

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code: Key::Escape, .. } => {
                    return
                },
                Event::KeyPressed { code: Key::F5, .. } => step = true,
                _ => {}
            };
        }


        if Instant::now() - last_instruction > Duration::from_millis(2) {
            if step {
                chip.emulate_cycle();
                match debugger {
                    Some(ref mut ok) => {
                        ok.update(chip.clone());
                        //step = false;
                        match break_point {
                            Some(ref addr) => {
                                if chip.PC == u16::from_str_radix(addr.as_str(), 16).unwrap() + 0x200 {
                                    println!("PC: {} break_point: {}", &chip.PC, i64::from_str_radix(addr.as_str(), 16).unwrap() as u16);
                                    step = false;
                                }
                            },
                            None => {}
                        };
                    },
                    None => {}
                }
            }
            last_instruction = Instant::now();
        }

        if Instant::now() - delay_duration > Duration::from_millis(16) {
            if chip.delay_timer > 0 {
                chip.delay_timer -= 1;
            }
            if chip.sound_timer > 0 {
                chip.sound_timer -= 1;
            }
            delay_duration = Instant::now();
        }

        if Instant::now() - last_screen > Duration::from_millis(10) {
            window.clear(&Color::BLACK);
            for x in 0..SCREEN_COLUMNS {
                for y in 0..SCREEN_ROWS {
                    if chip.vid_mem[y][x] == 1 {
                        let x_pos = (x * SCALE) as f32;
                        let y_pos = (y * SCALE) as f32;
                        &mut rect.set_position((x_pos, y_pos));
                        window.draw(&rect);
                    }
                }
            }
            window.display();
            last_screen = Instant::now();
        }
        read_keys(&mut chip, &window);
    }
}


fn read_keys(chip: &mut Chip, window: &RenderWindow) {
    chip.key[0x0] = Key::X.is_pressed();
    chip.key[0x1] = Key::Num1.is_pressed();
    chip.key[0x2] = Key::Num2.is_pressed();
    chip.key[0x3] = Key::Num3.is_pressed();

    chip.key[0x4] = Key::Q.is_pressed();
    chip.key[0x5] = Key::W.is_pressed();
    chip.key[0x6] = Key::E.is_pressed();
    chip.key[0x7] = Key::A.is_pressed();

    chip.key[0x8] = Key::S.is_pressed();
    chip.key[0x9] = Key::D.is_pressed();
    chip.key[0xa] = Key::Z.is_pressed();
    chip.key[0xb] = Key::C.is_pressed();

    chip.key[0xc] = Key::Num4.is_pressed();
    chip.key[0xd] = Key::R.is_pressed();
    chip.key[0xe] = Key::F.is_pressed();
    chip.key[0xf] = Key::V.is_pressed();
}

fn load_rom(filename: String, chip: &mut Chip) {
    let mut f = File::open(&filename).unwrap_or_else(|e| {
        println!("Filename: {}", filename);
        panic!(e);
    });
    let meta = fs::metadata(filename).unwrap();
    let file_length = meta.len();
    let mut buf: Vec<u8> = Vec::with_capacity(file_length as usize);
    f.read_to_end(&mut buf).expect("File not found");
    chip.mem.write_rom(&buf);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
