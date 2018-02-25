#![allow(dead_code, unused)]

use chip::*;

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
        let mut t = Text::new("test", &self.font, 12);
        t.set_position((0f32, 0f32));
        t.set_fill_color(&Color::WHITE);
        self.window.clear(&Color::BLACK);
        self.window.draw(&t);
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
