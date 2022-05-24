extern crate minifb;

use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};
mod chip8;
mod window;

use window::*;
use chip8::{CPU, SCREEN_HEIGHT, SCREEN_WIDTH};
use minifb::{Key, Window, WindowOptions};

const CPU_CLOCK_SPEED_HZ: f64 = 500.0;
const FRAMERATE_TARGET_HZ: f64 = 60.0;

fn main() {
    let mut file = File::open("data/Pong.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");
    let mut chip8 = CPU::new();

    chip8.load_rom(&data);

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
            chip8.set_keys(&keys);

            chip8.execute_cycle();

            last_cpu_update = now;
        }

        if (Instant::now() - last_timer_update) > target_frame_duration {
            chip8.update_timers();

            convert_display_buffer(chip8.get_display_buffer(), &mut buffer);

            window
                .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
                .unwrap();

            last_timer_update = now;
        }
    }
}
