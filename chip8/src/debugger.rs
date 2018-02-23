use sfml::window::*;
use sfml::graphics::*;

const TEXT_DISTANCE_X: usize = 10;
const TEXT_DISTANCE_Y: usize = 14;

pub struct Debugger {
    window: RenderWindow,
    font: Font,
    reg_vec: String,
}

impl Debugger {
    pub fn run(&mut self) {
        while self.window.is_open() {
            self.print();
            self.window.clear(&Color::BLACK);
            self.window.display();   
        } 
    }

    pub fn new() -> Debugger {
        let ret = Debugger {
            window: RenderWindow::new(
                        (200, 400),
                        "Debug window",
                        Style::CLOSE,
                        &Default::default(),
                        ),
            font: Font::from_file("../resources/SpaceMono-Regular.ttf").unwrap(),
            reg_vec: String::from("Register: "),
        }; 
        ret
    }

   
    fn print(&mut self) {
        let mut output = self.reg_vec.clone();
        output.push_str("Test");
        let t = Text::new(output.as_str(), &self.font, 12);
        for i in 0..self.reg_vec.len() {
            self.window.draw(&t);
        }
    }

}
