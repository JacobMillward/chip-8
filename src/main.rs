extern crate minifb;

use std::{
    fs::File,
    io::Read
};
mod chip8;
mod window;

use window::run_chip8_program;

fn main() {
    let mut file = File::open("data/Pong.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");
   
    run_chip8_program(&data);
}

