#![allow(dead_code, unused)]

use engine::chip::Chip;

use sfml::window::*;
use sfml::system::*;
use sfml::graphics::*;

use std::fmt::*;
use std::env;

struct TextObject (String, String);

impl TextObject {
    fn new(first: &str, second: u32) -> TextObject {
        TextObject(String::from(first), format!("{:X}", second))
    }
}

pub struct Debugger {
    window: RenderWindow,
    font: Font,
    register_arr: Vec<TextObject>,
    pc: TextObject,
    I: TextObject,
}

impl Debugger {

    pub fn new() -> Self {
        let mut current_path = env::current_dir().unwrap();
        current_path.push("resources/SpaceMono-Regular.ttf");
        Debugger {
            window: RenderWindow::new(
                        (600, 400),
                        "Debug window",
                        Style::CLOSE,
                        &Default::default(),
                        ),
                        font: Font::from_file(current_path.as_path().to_str().unwrap()).unwrap(),
                        register_arr: Self::init_reg(),
                        pc: TextObject::new("PC", 0),
                        I: TextObject::new("I", 0)
        }
    }

    pub fn update(&mut self, chip: Chip) {
        let y_offset: f32 = 16f32;
        let initial_y: f32 = 0f32;
        let initial_x: f32 = 0f32;
        let mut pc = Text::new(format!("PC: {:X}", chip.PC).as_str(), &self.font, 14);
        pc.set_position((initial_x, initial_y));

        let mut sp = Text::new(format!("SP: {:X}", chip.SP).as_str(), &self.font, 14);
        sp.set_position((initial_x, initial_y + y_offset));
        
        let mut i = Text::new(format!("I: {:X}", chip.I).as_str(), &self.font, 14);
        i.set_position((initial_x, initial_y + (y_offset*2f32)));
        self.window.clear(&Color::BLACK);
        self.window.draw(&pc);
        self.window.draw(&sp);
        self.window.draw(&i);
        let y = initial_y + (y_offset * 3f32);
        for i in 0..16 {
            let mut current_reg = Text::new(format!("V{:X}: {:X}", i, chip.V[i]).as_str(), &self.font, 14);
            current_reg.set_position((initial_x, y + (y_offset * i as f32) as f32));
            self.window.draw(&current_reg);

        }
        self.window.display();
    }

    fn init_reg() -> Vec<TextObject> {
        let mut ret = Vec::<TextObject>::new();
        for i in 0..16 {
            ret.push(TextObject::new("V", i as u32));
        }
        ret
    }

    fn print_reg(&self) {

    }
}
