pub mod ram;

pub const SCREEN_COLUMNS: usize = 64;
pub const SCREEN_ROWS: usize = 32;

pub struct Chip {
    pub I: u16,
    pub mem: ram::Ram,
    pub PC: usize,
    pub SP: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub vid_mem: [u8; SCREEN_COLUMNS * SCREEN_ROWS],
    pub stack: [u16; 16],
}

impl Chip {
    pub fn new() -> Chip {
        let chip = Chip {
            I: 0,
            mem: ram::Ram::new(),
            PC: 0x200,
            delay_timer: 60,
            SP: 0,
            sound_timer: 60,
            vid_mem: [0; SCREEN_COLUMNS * SCREEN_ROWS],
            stack: [0; 16],
        };
        chip
    }
}
