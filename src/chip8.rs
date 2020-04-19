pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

struct Registers {
    v: [u8; 16],
    i: u16,
    pc: u16,
}

impl Registers {
    pub fn inc_pc(&mut self) {
        self.pc += 2;
    }
}

struct Timers {
    delay_timer: u8,
    sound_timer: u8,
}

struct ReturnStack {
    stack: [u16; 16],
    sp: u8,
}

impl ReturnStack {
    pub fn push(&mut self, val: u16) {
        if self.sp as usize == self.stack.len() - 1 {
            panic!("Chip8 Stack Overflow");
        }

        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Chip8 Stack Underflow");
        }

        let val = self.stack[self.sp as usize];
        self.sp -= 1;

        val
    }
}

pub struct Chip8 {
    memory: [u8; 4096],
    registers: Registers,
    return_stack: ReturnStack,
    timers: Timers,
    gfx: [u8; SCREEN_HEIGHT * SCREEN_WIDTH],
    keys: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            registers: Registers {
                v: [0; 16],
                i: 0,
                pc: 0x200,
            },
            return_stack: ReturnStack {
                stack: [0; 16],
                sp: 0,
            },
            timers: Timers {
                delay_timer: 0,
                sound_timer: 0,
            },
            gfx: [0; SCREEN_HEIGHT * SCREEN_WIDTH],
            keys: [0; 16],
        }
    }

    pub fn load_rom(&mut self, rom: &[u8; 3584]) {
        self.load_into_memory(rom, 0x200)
    }

    pub fn load_into_memory(&mut self, data: &[u8], offset_index: u16) {
        self.memory[offset_index as usize..].clone_from_slice(data)
    }

    pub fn update_timers(&mut self) {
        if self.timers.delay_timer > 0 {
            self.timers.delay_timer -= 1;
        }

        if self.timers.sound_timer > 0 {
            self.timers.delay_timer -= 1;
        }
    }

    pub fn set_keys(&mut self, keys: &[u8; 16]) {
        self.keys.copy_from_slice(keys)
    }

    pub fn get_display_buffer(&self) -> &[u8; 64 * 32] {
        &self.gfx
    }

    pub fn execute_cycle(&mut self) {
        // Fetch
        let opcode: u16 = self.read_word(self.registers.pc);

        // Decode
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.registers.v[x];
        let vy = self.registers.v[y];
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // Match and Execute
        match (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F),
        ) {
            // CLS
            (0, 0, 0xE, 0) => {
                self.gfx = [0; SCREEN_HEIGHT * SCREEN_WIDTH];
                self.registers.inc_pc();
            },
            // RET
            (0, 0, 0xE, 0xE) => self.registers.pc = self.return_stack.pop(),
            // JP addr
            (1, _, _, _) => self.registers.pc = nnn,
            // CALL addr
            (2, _, _, _) => {
                self.return_stack.push(self.registers.pc);
                self.registers.pc = nnn;
            },
            // SE Vx, byte
            (3, _, _, _) => {
                if vx == kk {
                    self.registers.inc_pc();
                    self.registers.inc_pc();
                } else {
                    self.registers.inc_pc();
                }
            },
            // SNE Vx, byte
            (4, _, _, _) => {
                if vx != kk {
                    self.registers.inc_pc();
                    self.registers.inc_pc();
                } else {
                    self.registers.inc_pc();
                }
            },
            // SE Vx, Vy
            (5, _, _, 0) => {
                if vx == vy {
                    self.registers.inc_pc();
                    self.registers.inc_pc();
                } else {
                    self.registers.inc_pc();
                }
            },
            // LD Vx, byte
            (6, _, _, _) => {
                self.registers.v[x] = kk;
                self.registers.inc_pc();
            },
            // ADD Vx, byte
            (7, _, _, _) => {
                self.registers.v[x] += kk;
                self.registers.inc_pc();
            },
            // LD Vx, Vy
            (8, _, _, 0) => {
                self.registers.v[x] = vy;
                self.registers.inc_pc();
            },
            // OR Vx, Vy
            (8, _, _, 1) => {
                self.registers.v[x] = vx | vy;
                self.registers.inc_pc();
            },
            // AND Vx, Vy
            (8, _, _, 2) => {
                self.registers.v[x] = vx & vy;
                self.registers.inc_pc();
            },
            // XOR Vx, Vy
            (8, _, _, 3) => {
                self.registers.v[x] = vx ^ vy;
                self.registers.inc_pc();
            },

            (_, _, _, _) => (),
        }
    }

    fn read_word(&self, index: u16) -> u16 {
        ((self.memory[index as usize] as u16) << 8) | (self.memory[(index + 1) as usize] as u16)
    }
}
