struct chip8 {
    /// Current Opcode
    opcode: u16,
    /// Device Memory
    memory: [u8; 4096],
    /// Device Registers
    v: [u8; 16],
    /// Delay Timer
    delay_timer: u8,
    /// Sound Timer
    sound_timer: u8,
    /// Index
    i: u16,
    /// Program Counter
    pc: u16,
    /// Graphics Buffer
    gfx: [u8; 64*32],
    /// Stack
    stack: [u8; 16],
    /// Stack Pointer
    sp: u8,
    /// Current Keycodes pressed
    key: [u8; 16]
}



fn main() {
    println!("Hello, world!");
}
