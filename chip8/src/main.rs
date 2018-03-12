#![allow(dead_code, unused)]
#![feature(nll)]

extern crate sfml;
extern crate rand;
extern crate nfd;
extern crate getopts;

mod engine;
mod utils;
mod debugger;
mod gui;
mod app;

use app::App;
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
use std::collections::BTreeSet;

use engine::chip::Chip;
use utils::{SCREEN_COLUMNS, SCREEN_ROWS, SCALE};

use sfml::window::{VideoMode, ContextSettings, Event, Key, Style};
use sfml::system::{Time, Clock, Vector2f};
use sfml::graphics::{RenderTarget, RectangleShape, Transformable, Drawable, RenderWindow, Shape,
Color};

use nfd::Response;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("r", "rom", "path to rom", "NAME");
    opts.optopt(
        "b",
        "break-point",
        "adress to break on (in hexadecimal)",
        "ADRESS",
        );
    opts.optflag("d", "debug", "run in debug mode");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("s", "supermode", "run in super chip8 mode");

    let program = args[0].clone();

    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let rom: String = if !matches.opt_strs("r").is_empty() {
        matches.opt_strs("r")[0].clone()
    } else {
        let file_result =
            nfd::open_file_dialog(Some("ch8"), env::current_dir().unwrap().as_path().to_str())
            .unwrap_or_else(|e| {
                panic!(e);
            });

        match file_result {
            Response::Okay(file_path) => file_path,
            Response::OkayMultiple(files) => {
                println!("Choose one file only");
                panic!();
            }
            _ => panic!(),
        }
    };
    
    App::new(matches).run(rom);

}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
