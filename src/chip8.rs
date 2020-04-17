struct Registers {
    v: [u8; 16],
    i: u16,
    pc: u16,
}

struct Timers {
    delay_timer: u8,
    sound_timer: u8,
}

struct ReturnStack {
    stack: [u8; 16],
    sp: u8,
}

pub struct Chip8 {
    memory: [u8; 4096],
    registers: Registers,
    return_stack: ReturnStack,
    timers: Timers,
    gfx: [u8; 64 * 32],
    keys: [u8; 16]
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            registers: Registers {
                v: [0; 16],
                i: 0,
                pc: 0x200
            },
            return_stack: ReturnStack {
                stack: [0; 16],
                sp: 0
            },
            timers: Timers {
                delay_timer: 0,
                sound_timer: 0
            },
            gfx: [0; 64 * 32],
            keys: [0; 16]
        }
    }
}