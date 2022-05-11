extern crate minifb;

use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};
mod chip8;
use chip8::{Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};
use minifb::{Key, Window, WindowOptions};

const CPU_CLOCK_SPEED_HZ: f64 = 500.0;
const FRAMERATE_TARGET_HZ: f64 = 60.0;

fn main() {
    let mut file = File::open("data/Pong.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");
    let mut chip = Chip8::new();

    chip.load_rom(&data);

    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new("Chip8", 640, 320, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let target_cpu_update_duration: Duration = Duration::from_secs_f64(1_f64 / CPU_CLOCK_SPEED_HZ);
    let target_frame_duration: Duration = Duration::from_secs_f64(1_f64 / FRAMERATE_TARGET_HZ);
    let mut last_cpu_update = Instant::now();
    let mut last_timer_update = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        if (now - last_cpu_update) > target_cpu_update_duration {
            let raw_keys = window.get_keys();
            let keys = get_chip8_keys(raw_keys);
            chip.set_keys(&keys);

            chip.execute_cycle();

            last_cpu_update = now;
        }

        if (Instant::now() - last_timer_update) > target_frame_duration {
            chip.update_timers();

            convert_display_buffer(chip.get_display_buffer(), &mut buffer);

            window
                .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
                .unwrap();

            last_timer_update = now;
        }
    }
}

fn convert_display_buffer(src: &[u8; SCREEN_WIDTH * SCREEN_HEIGHT], dest: &mut [u32]) {
    for (index, val) in src.iter().enumerate() {
        dest[index] = match val {
            0 => 0x0,
            1 => 0xffffff,
            _ => unreachable!(),
        };
    }
}

fn get_chip8_keys(keys: Vec<Key>) -> [u8; 16] {
    let mut result: [u8; 16] = [0; 16];
    let mut last_set_index = 0;
    for key in keys.iter() {
        let converted = match key {
            Key::Key1 => Some(0x1),
            Key::Key2 => Some(0x2),
            Key::Key3 => Some(0x3),
            Key::Key4 => Some(0xC),

            Key::Q => Some(0x4),
            Key::W => Some(0x5),
            Key::E => Some(0x6),
            Key::R => Some(0xD),

            Key::A => Some(0x7),
            Key::S => Some(0x8),
            Key::D => Some(0x9),
            Key::F => Some(0xE),

            Key::Z => Some(0xA),
            Key::X => Some(0x0),
            Key::C => Some(0xB),
            Key::V => Some(0xF),
            _ => None,
        };

        if let Some(k) = converted {
            result[last_set_index] = k;
            last_set_index += 1;
        }
    }

    result
}
