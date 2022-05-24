use rand::rngs::ThreadRng;
use rand::Rng;
use std::num::Wrapping;

use super::return_stack::ReturnStack;

use super::SCREEN_HEIGHT;
use super::SCREEN_WIDTH;

pub(crate) struct Registers {
    pub(crate) v: [u8; 16],
    pub(crate) i: u16,
    pub(crate) pc: u16,
}

impl Registers {
    pub fn inc_pc(&mut self) {
        self.pc += 2;
    }
}

pub(crate) struct Timers {
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,
}

pub struct CPU {
    memory: [u8; 4096],
    registers: Registers,
    return_stack: ReturnStack,
    timers: Timers,
    gfx: [u8; SCREEN_HEIGHT * SCREEN_WIDTH],
    keys: [u8; 16],
    rng: ThreadRng,
}

impl CPU {
    pub fn new() -> Self {
        let mut chip8 = Self {
            memory: [0; 4096],
            registers: Registers {
                v: [0; 16],
                i: 0,
                pc: 0x200,
            },
            return_stack: ReturnStack::new(),
            timers: Timers {
                delay_timer: 0,
                sound_timer: 0,
            },
            gfx: [0; SCREEN_HEIGHT * SCREEN_WIDTH],
            keys: [0; 16],
            rng: rand::thread_rng(),
        };

        chip8.load_sprite_font();

        chip8
    }

    pub(crate) fn load_sprite_font(&mut self) {
        pub(crate) const SPRITE_FONT: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        self.memory[0..80].copy_from_slice(&SPRITE_FONT);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        if rom.len() > 3584 {
            panic!("Cannot load rom with size greater than 3584 bytes");
        }

        self.memory[0x200..(rom.len() + 0x200)].copy_from_slice(rom);
    }

    pub fn update_timers(&mut self) {
        if self.timers.delay_timer > 0 {
            self.timers.delay_timer -= 1;
        }

        if self.timers.sound_timer > 0 {
            self.timers.sound_timer -= 1;
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
            }
            // RET
            (0, 0, 0xE, 0xE) => {
                self.registers.pc = self.return_stack.pop();
            }
            // JP addr
            (1, _, _, _) => {
                self.registers.pc = nnn;
            }
            // CALL addr
            (2, _, _, _) => {
                self.registers.inc_pc();
                self.return_stack.push(self.registers.pc);
                self.registers.pc = nnn;
            }
            // SE Vx, byte
            (3, _, _, _) => {
                if vx == kk {
                    self.registers.inc_pc();
                }
                self.registers.inc_pc();
            }
            // SNE Vx, byte
            (4, _, _, _) => {
                if vx != kk {
                    self.registers.inc_pc();
                }
                self.registers.inc_pc();
            }
            // SE Vx, Vy
            (5, _, _, 0) => {
                if vx == vy {
                    self.registers.inc_pc();
                }
                self.registers.inc_pc();
            }
            // LD Vx, byte
            (6, _, _, _) => {
                self.registers.v[x] = kk;
                self.registers.inc_pc();
            }
            // ADD Vx, byte
            (7, _, _, _) => {
                self.registers.v[x] = (Wrapping(vx) + Wrapping(kk)).0;
                self.registers.inc_pc();
            }
            // LD Vx, Vy
            (8, _, _, 0) => {
                self.registers.v[x] = vy;
                self.registers.inc_pc();
            }
            // OR Vx, Vy
            (8, _, _, 1) => {
                self.registers.v[x] = vx | vy;
                self.registers.inc_pc();
            }
            // AND Vx, Vy
            (8, _, _, 2) => {
                self.registers.v[x] = vx & vy;
                self.registers.inc_pc();
            }
            // XOR Vx, Vy
            (8, _, _, 3) => {
                self.registers.v[x] = vx ^ vy;
                self.registers.inc_pc();
            }
            // ADD Vx, Vy
            (8, _, _, 4) => {
                let sum: u16 = vx as u16 + vy as u16;
                self.registers.v[x] = sum as u8;
                self.registers.v[0xF] = if sum > 0xFF { 1 } else { 0 };
                self.registers.inc_pc();
            }
            // SUB Vx, Vy
            (8, _, _, 5) => {
                self.registers.v[0xF] = if vx > vy { 1 } else { 0 };
                self.registers.v[x] = (Wrapping(vx) - Wrapping(vy)).0;
                self.registers.inc_pc();
            }
            // SHR Vx
            (8, _, _, 6) => {
                self.registers.v[0xF] = vx & 0x1;
                self.registers.v[x] = vx >> 1;
                self.registers.inc_pc();
            }
            // SUBN Vx, Vy
            (8, _, _, 7) => {
                self.registers.v[0xF] = if vy > vx { 1 } else { 0 };
                self.registers.v[x] = (vy as i8 - vx as i8) as u8;
                self.registers.inc_pc();
            }
            // SHR Vx
            (8, _, _, 0xE) => {
                self.registers.v[0xF] = vx & 0x80;
                self.registers.v[x] = vx << 1;
                self.registers.inc_pc();
            }
            // SNE Vx, Vy
            (9, _, _, 0) => {
                if vx != vy {
                    self.registers.inc_pc();
                }
                self.registers.inc_pc();
            }
            // LD I, addr
            (0xA, _, _, _) => {
                self.registers.i = nnn;
                self.registers.inc_pc();
            }
            // JP V0, addr
            (0xB, _, _, _) => {
                self.registers.pc = self.registers.v[0] as u16 + nnn;
            }
            // RND Vx, byte
            (0xC, _, _, _) => {
                let random_number = self.rng.gen_range(0, 256) as u8;

                self.registers.v[x] = random_number & kk;
                self.registers.inc_pc();
            }
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                let (start, end) = (
                    self.registers.i as usize,
                    (self.registers.i + (n as u16)) as usize,
                );
                let sprite = &self.memory[start..end];

                let mut was_erased = false;

                for (sprite_y, pixel_row) in sprite.iter().enumerate() {
                    let pixel_y = (vy as usize + sprite_y) % SCREEN_HEIGHT;

                    for sprite_x in 0..8_usize {
                        let pixel_x = (vx as usize + sprite_x) % SCREEN_WIDTH;
                        let gfx_idx = (pixel_y * SCREEN_WIDTH) + pixel_x;

                        let pixel_val = (pixel_row & (0x80 >> sprite_x)) > 0;
                        let old_val = self.gfx[gfx_idx] > 0;

                        self.gfx[gfx_idx] = if old_val ^ pixel_val { 1 } else { 0 };

                        if old_val && pixel_val {
                            was_erased = true;
                        }
                    }
                }

                self.registers.v[0xF] = if was_erased { 1 } else { 0 };
                self.registers.inc_pc();
            }
            // SKP Vx
            (0xE, _, 9, 0xE) => {
                if self.keys.contains(&vx) {
                    self.registers.inc_pc();
                }
                self.registers.inc_pc();
            }
            // SKNP Vx
            (0xE, _, 0xA, 1) => {
                if !self.keys.contains(&vx) {
                    self.registers.inc_pc();
                }
                self.registers.inc_pc();
            }
            // LD Vx, DT
            (0xF, _, 0, 7) => {
                self.registers.v[x] = self.timers.delay_timer;
                self.registers.inc_pc();
            }
            // LD Vx, K
            (0xF, _, 0, 0xA) => {
                let pressed_key = self.keys.iter().find(|&k| *k != 0);
                if let Some(key_value) = pressed_key {
                    self.registers.v[x] = *key_value;
                    self.registers.inc_pc();
                }
            }
            // LD DT, Vx
            (0xF, _, 1, 5) => {
                self.timers.delay_timer = vx;
                self.registers.inc_pc();
            }
            // LD ST, Vx
            (0xF, _, 1, 8) => {
                self.timers.sound_timer = vx;
                self.registers.inc_pc();
            }
            // ADD I, Vx
            (0xF, _, 1, 0xE) => {
                self.registers.i += vx as u16;
                self.registers.inc_pc();
            }
            // LD F, Vx
            (0xF, _, 2, 9) => {
                self.registers.i = vx as u16 * 5;
                self.registers.inc_pc();
            }
            // LD B, Vx
            (0xF, _, 3, 3) => {
                self.memory[self.registers.i as usize] = vx / 100;
                self.memory[(self.registers.i + 1) as usize] = (vx / 10) % 10;
                self.memory[(self.registers.i + 2) as usize] = (vx % 100) % 10;
                self.registers.inc_pc();
            }
            // LD [I], Vx
            (0xF, _, 5, 5) => {
                for idx in 0..x {
                    self.memory[(self.registers.i + idx as u16) as usize] = self.registers.v[idx];
                }
                self.registers.inc_pc();
            }
            // LD Vx, [I]
            (0xF, _, 6, 5) => {
                for idx in 0..x {
                    self.registers.v[idx] = self.memory[(self.registers.i + idx as u16) as usize];
                }
                self.registers.inc_pc();
            }

            (_, _, _, _) => (),
        }
    }

    pub(crate) fn read_word(&self, index: u16) -> u16 {
        ((self.memory[index as usize] as u16) << 8) | (self.memory[(index + 1) as usize] as u16)
    }
}
