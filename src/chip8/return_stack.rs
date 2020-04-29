pub(super) struct ReturnStack {
    stack: [u16; 16],
    sp: u8,
}

impl ReturnStack {
    pub fn new() -> Self {
        Self {
            stack: [0; 16],
            sp: 0,
        }
    }
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