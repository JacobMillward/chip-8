
pub fn convert_display_buffer(src: &[u8; SCREEN_WIDTH * SCREEN_HEIGHT], dest: &mut [u32]) {
    for (index, val) in src.iter().enumerate() {
        dest[index] = match val {
            0 => 0x0,
            1 => 0xffffff,
            _ => unreachable!(),
        };
    }
}

pub(crate) fn get_chip8_keys(keys: Vec<Key>) -> [u8; 16] {
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
