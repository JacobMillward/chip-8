mod chip8;

use chip8::{SCREEN_HEIGHT, SCREEN_WIDTH};
use clap::Parser;
use ferro_app::{
    winit::{
        event::{Event, WindowEvent},
        keyboard::KeyCode,
    },
    AppBuilder, InputManager,
};
use pixels::{PixelsBuilder, SurfaceTexture};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

const CPU_CLOCK_SPEED_HZ: f64 = 500.0;
const FRAMERATE_TARGET_HZ: f64 = 60.0;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// ROM file to play
    input_rom: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut file = File::open(args.input_rom).unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");

    let mut app = AppBuilder::new()
        .with_window_title("Ferro8")
        .with_window_size(640, 320)
        .build()?;

    let mut pixels = {
        let window_size = app.window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &app.window);

        PixelsBuilder::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)
            .texture_format(pixels::wgpu::TextureFormat::Rgba8UnormSrgb)
            .build()?
    };

    pixels.render_texture_format();

    let mut chip8 = chip8::Chip8::new();

    chip8.load_rom(&data);

    let target_cpu_update_duration = Duration::from_secs_f64(1_f64 / CPU_CLOCK_SPEED_HZ);
    let target_frame_duration = Duration::from_secs_f64(1_f64 / FRAMERATE_TARGET_HZ);
    let mut last_cpu_update = Instant::now();
    let mut last_timer_update = Instant::now();

    app.run(|event, event_loop, input| {
        if let Event::WindowEvent {
            event: WindowEvent::Resized(size),
            ..
        } = event
        {
            if let Err(err) = pixels.resize_surface(size.width, size.height) {
                eprintln!("pixels.resize_surface() failed: {}", err);
                event_loop.exit();
            }
        }

        let now = Instant::now();

        let keys = get_chip8_keys(input);
        chip8.set_keys(&keys);

        if (now - last_cpu_update) > target_cpu_update_duration {
            chip8.execute_cycle();

            last_cpu_update = now;
        }

        if (Instant::now() - last_timer_update) > target_frame_duration {
            chip8.update_timers();

            let frame = pixels.frame_mut();
            draw_chip8(chip8.get_display_buffer(), frame);

            if let Err(err) = pixels.render() {
                eprintln!("pixels.render() failed: {}", err);
                event_loop.exit();
                return;
            }

            last_timer_update = now;
        }
    })
    .expect("Failed to run event loop");

    Ok(())
}

/// Returns a 16 element array of keys that are currently pressed, with the corresponding Chip8 key value.
fn get_chip8_keys(input: &InputManager) -> [u8; 16] {
    let mut result: [u8; 16] = [0; 16];
    let mut last_set_index = 0;
    let key_map: HashMap<KeyCode, u8> = HashMap::from_iter([
        // Row 1
        (KeyCode::Digit1, 0x1),
        (KeyCode::Digit2, 0x2),
        (KeyCode::Digit3, 0x3),
        (KeyCode::Digit4, 0xC),
        // Row 2
        (KeyCode::KeyQ, 0x4),
        (KeyCode::KeyW, 0x5),
        (KeyCode::KeyE, 0x6),
        (KeyCode::KeyR, 0xD),
        // Row 3
        (KeyCode::KeyA, 0x7),
        (KeyCode::KeyS, 0x8),
        (KeyCode::KeyD, 0x9),
        (KeyCode::KeyF, 0xE),
        // Row 4
        (KeyCode::KeyZ, 0xA),
        (KeyCode::KeyX, 0x0),
        (KeyCode::KeyC, 0xB),
        (KeyCode::KeyV, 0xF),
    ]);

    for key in key_map.keys() {
        if input.key_down(*key) {
            result[last_set_index] = *key_map.get(key).unwrap();
            last_set_index += 1;
        }
    }

    result
}

// Converts the Chip8 display buffer to Rgba8UnormSrgb format.
// Chip8 display buffer is a 1D array of 2048 (64*32) pixels, each pixel is either 0 or 1.
// Rgba8UnormSrgb format is a 1D array of 8192 (64*32*4) bytes, each byte is a color component (R, G, B, A).
fn draw_chip8(chip8_display_buffer: &[u8; SCREEN_WIDTH * SCREEN_HEIGHT], frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let color = if chip8_display_buffer[i] == 0 { 0 } else { 255 };

        pixel.copy_from_slice(&[color, color, color, 255]);
    }
}
