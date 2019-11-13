pub const SR_MASK_NEGATIVE: u8 = 1 << 7;
pub const SR_MASK_OVERFLOW: u8 = 1 << 6;
pub const SR_MASK_BREAK: u8 = 1 << 4;
pub const SR_MASK_DECIMAL: u8 = 1 << 3;
pub const SR_MASK_INTERRUPT: u8 = 1 << 2;
pub const SR_MASK_ZERO: u8 = 1 << 1;
pub const SR_MASK_CARRY: u8 = 1 << 0;

const PC_START: u16 = 0x4020;

pub struct State {
    pub acc: u8,
    pub x: u8,
    pub y: u8,

    program_counter: u16,
    next_pc: u16,

    stack_pointer: u8,
    status: u8,

    cycles: u64,
}

impl State {
    pub fn new() -> State {
        let mut state = State {
            acc: 0,
            x: 0,
            y: 0,
            program_counter: 0,
            next_pc: 0,
            stack_pointer: 0,
            status: 0,
            cycles: 0,
        };
        state.program_counter = PC_START;
        state.next_pc = PC_START;

        state
    }

    pub fn calculate_relative_pc(&self, offset: i8) -> u16 {
        if offset < 0 {
            self.next_pc - (-offset) as u16
        } else {
            self.next_pc + offset as u16
        }
    }

    pub fn set_next_pc(&mut self, pc: u16) {
        self.next_pc = pc;
    }

    pub fn update_pc(&mut self) {
        self.program_counter = self.next_pc;
    }

    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn get_sp(&self) -> u8 {
        self.stack_pointer
    }

    pub fn get_status(&self, mask: u8) -> bool {
        self.status & mask > 0
    }

    pub fn set_status(&mut self, field: u8, value: bool) {
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

        state.set_next_pc(0);
        state.update_pc();

        state.offset_next_pc(100);
        state.update_pc();
        assert_eq!(100, state.program_counter);

        state.offset_next_pc(-50);
        state.update_pc();
        assert_eq!(50, state.program_counter);
    }

    #[test]
    fn test_set_status_field() {
        let mut state = State::new();

        let initial = super::SR_MASK_OVERFLOW | super::SR_MASK_CARRY | super::SR_MASK_ZERO;

        state.set_status(initial, true);
        assert_eq!(initial, state.status);
        state.set_status(super::SR_MASK_BREAK, true);
        assert_eq!(initial | super::SR_MASK_BREAK, state.status);
        state.set_status(super::SR_MASK_BREAK, false);
        assert_eq!(initial, state.status);
    }
}