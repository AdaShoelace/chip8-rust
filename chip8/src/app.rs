#![feature(nll)]
use gui::*;
use engine::chip::Chip;
use debugger::Debugger;

use std::collections::btree_set::*;
use getopts::{Options, Matches};

use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::time::{Instant, Duration};

pub struct App<'a> {
    gui: Gui<'a>,
    chip: Chip,
    debugger: Option<Debugger>,
    break_points: Option<BTreeSet<u16>>,
    pub running: bool,
    pub step: bool,
    delay_duration: Instant,
}

impl<'a> App<'a> {
    pub fn new(args: Matches) -> Self {
        App {
            gui: Gui::new(),
            chip: Chip::new(),
            debugger: if args.opt_present("d") { Some(Debugger::new()) } else { None },
            break_points: Self::get_break_points(args),
            running: true,
            step: true,
            delay_duration: Instant::now(),
        }
    }

    pub fn run(&mut self, rom: String) {
        self.load_rom(rom);
        let mut now = Instant::now();
        let mut last_instruction = now.clone();
        let mut last_screen = now.clone();

        while self.running {
            let ret = self.gui.handle_events();
            self.running = ret.0;
            self.step = ret.1;
            now = Instant::now();
            if now - last_instruction > Duration::from_millis(2) {
                if self.step {
                    self.chip.emulate_cycle();
                    match self.debugger {
                        Some(ref mut ok) => {
                            ok.update(self.chip.clone());
                            match self.break_points {
                                Some(ref mut addr) => {
                                    if addr.contains(&self.chip.PC) {
                                        self.step = false;
                                    }
                                }
                                None => {}
                            };
                        }
                        None => {}
                    }
                    last_instruction = now.clone();
                }
                self.decr_timers(now.clone());

                if now - last_screen > Duration::from_millis(10) {
                    self.gui.paint(&self.chip.vid_mem);
                    last_screen = now.clone();
                }
                self.gui.read_keys(&mut self.chip);
            }
        }
    }

    fn load_rom(&mut self, filename: String) {
        let mut f = File::open(&filename).unwrap_or_else(|e| {
            println!("Filename: {}", filename);
            panic!(e);
        });
        let meta = fs::metadata(filename).unwrap();
        let file_length = meta.len();
        let mut buf: Vec<u8> = Vec::with_capacity(file_length as usize);
        f.read_to_end(&mut buf).expect("File not found");
        self.chip.mem.write_rom(&buf);
    }

    fn decr_timers(&mut self, now: Instant) {
        if now - self.delay_duration > Duration::from_millis(16) {
            if self.chip.delay_timer > 0 {
                self.chip.delay_timer -= 1;
            }
            if self.chip.sound_timer > 0 {
                self.chip.sound_timer -= 1;
            }
            self.delay_duration = Instant::now();
        }
    }

    fn get_break_points(args: Matches) -> Option<BTreeSet<u16>> {
        let mut break_point: Option<BTreeSet<u16>> = if args.opt_present("b") {
            Some(
                args
                .opt_strs("b")
                .into_iter()
                .map(|x| (u16::from_str_radix(x.as_str(), 16).unwrap() + 0x200))
                .collect::<BTreeSet<u16>>(),
                )
        } else {
            None
        };
        break_point
    }
}
