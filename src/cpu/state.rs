pub const SR_MASK_NEGATIVE: u8 = 1 << 7;
pub const SR_MASK_OVERFLOW: u8 = 1 << 6;
pub const SR_MASK_BREAK: u8 = 1 << 4;
pub const SR_MASK_DECIMAL: u8 = 1 << 3;
pub const SR_MASK_INTERRUPT: u8 = 1 << 2;
pub const SR_MASK_ZERO: u8 = 1 << 1;
pub const SR_MASK_CARRY: u8 = 1 << 0;

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

    fn set_status_field(&mut self, field: u8, value: bool) {
        if value {
            self.status |= field;
        } else {
            self.status &= !field;
        }
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

    #[test]
    fn test_set_status_field() {
        let mut state = State::new();

        let initial = super::SR_MASK_OVERFLOW | super::SR_MASK_CARRY | super::SR_MASK_ZERO;

        state.set_status_field(initial, true);
        assert_eq!(initial, state.status);
        state.set_status_field(super::SR_MASK_BREAK, true);
        assert_eq!(initial | super::SR_MASK_BREAK, state.status);
        state.set_status_field(super::SR_MASK_BREAK, false);
        assert_eq!(initial, state.status);
    }
}