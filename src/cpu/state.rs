
pub struct State {
    pub acc: u8,
    pub x: u8,
    pub y: u8,

    program_counter: u16,
    stack_pointer: u8,
    status: u8,

    cycles: u64,
}

impl State {
    pub fn new() -> State {
        State {
            acc: 0,
            x: 0,
            y: 0,
            program_counter: 0,
            stack_pointer: 0,
            status: 0,
            cycles: 0,
        }
    }

    pub fn offset_pc(&mut self, offset: i8) {
        if offset < 0 {
            self.program_counter -= (-offset) as u16;
        } else {
            self.program_counter += offset as u16;
        }
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.program_counter = pc;
    }
    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }
}


#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn test_offset_pc() {
        let mut state = State::new();

        state.offset_pc(100);
        assert_eq!(100, state.program_counter);

        state.offset_pc(-50);
        assert_eq!(50, state.program_counter);
    }
}

/*
SR Flags (bit 7 to bit 0):

N	....	Negative
V	....	Overflow
-	....	ignored
B	....	Break
D	....	Decimal (use BCD for arithmetics)
I	....	Interrupt (IRQ disable)
Z	....	Zero
C	....	Carry
*/