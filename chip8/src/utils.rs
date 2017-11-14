#![allow(dead_code, unused_variables, non_snake_case)]
pub fn get_NNN(opcode: u16) -> u16 {
    opcode & 0x0fff
}

pub fn get_NN(opcode: u16) -> u16 {
    opcode & 0x00ff
}

pub fn get_N(opcode: u16) -> u16 {
    opcode & 0x000f
}

pub fn get_X(opcode: u16) -> u16 {
    (opcode & 0x0f00) >> 8
}

pub fn get_Y(opcode: u16) -> u16 {
    (opcode & 0x00f0) >> 4
}
