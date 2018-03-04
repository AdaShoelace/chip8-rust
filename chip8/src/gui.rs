use utils::*;
use engine::chip::Chip;

use sfml::window::{VideoMode, ContextSettings, Event, Key, Style};
use sfml::system::{Time, Clock, Vector2f};
use sfml::graphics::{RenderTarget, RectangleShape, Transformable, Drawable, RenderWindow, Shape, Color};

use app::App;

pub struct Gui<'a> {
    window: RenderWindow,
    rect: RectangleShape<'a>,
}
impl <'a> Gui<'a> {
    pub fn new() -> Self {
        let mut ret = Gui {
            window: RenderWindow::new(
                        ((SCREEN_COLUMNS * SCALE) as u32, (SCREEN_ROWS * SCALE) as u32),
                        "Chip8 emulator",
                        Style::CLOSE,
                        &Default::default(),
                        ),
                        rect: RectangleShape::new(),
        };
        ret.rect.set_size((SCALE as f32, SCALE as f32));
        ret
    }

    pub fn paint(&mut self, vid_buffer: &[[u8; SCREEN_COLUMNS]; SCREEN_ROWS]) {
        self.window.clear(&Color::BLACK);
        for x in 0..SCREEN_COLUMNS {
            for y in 0..SCREEN_ROWS {
                if vid_buffer[y][x] == 1 {
                    let x_pos = (x * SCALE) as f32;
                    let y_pos = (y * SCALE) as f32;
                    &mut self.rect.set_position((x_pos, y_pos));
                    self.window.draw(&self.rect);
                }
            }
        }
        self.window.display();
    }

    pub fn read_keys(&mut self, chip:&mut Chip) {
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

    pub fn handle_events(&mut self) -> (bool, bool) {
        let mut ret: (bool, bool) = (true, true);
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => ret.0 = false,
                Event::KeyPressed { code: Key::F5, .. } => ret.1 = true,
                _ => {}
            };
        }
        ret
    }
}
