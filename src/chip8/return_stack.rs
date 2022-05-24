#[derive(Debug, PartialEq)]
pub(super) enum StackError {
    Overflow,
    Underflow
}



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
    pub fn push(&mut self, val: u16) -> Result<(), StackError> {
        if self.sp as usize == self.stack.len() - 1 {
            return Err(StackError::Overflow);
        }

        self.stack[self.sp as usize] = val;
        self.sp += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16, StackError> {
        if self.sp == 0 {
            return Err(StackError::Underflow);

        }

        self.sp -= 1;
        Ok(self.stack[self.sp as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_push_and_retrieve_value() {
        let mut stack = ReturnStack::new();
        stack.push(2).expect("Unable to push value to stack");
        let val = stack.pop().unwrap();
        assert_eq!(val, 2);
    }

    #[test]
    fn does_overflow_after_16_items() {
        let mut stack = ReturnStack::new();
        for i in 1..16 {
            let result = stack.push(i);
            assert!(result.is_ok());
        }

        let result = stack.push(1);
        assert!(result.is_err());

        assert_eq!( match result {
            Ok(_) => unreachable!(),
            Err(err) => err
        }, StackError::Overflow);
    }

    #[test]
    fn does_underflow_when_empty() {
        let mut stack = ReturnStack::new();

        let result = stack.pop();
        assert!(result.is_err());

        assert_eq!( match result {
            Ok(_) => unreachable!(),
            Err(err) => err
        }, StackError::Underflow);
    }
}