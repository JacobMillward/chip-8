extern crate minifb;

use std::{
    fs::File,
    io::Read
};
use clap::Parser;

mod chip8;
mod window;

use window::run_chip8_program;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// ROM file to play
    input_rom: String,
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.input_rom).unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");
   
    run_chip8_program(&data);
}

