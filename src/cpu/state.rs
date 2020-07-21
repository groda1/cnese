pub const SR_MASK_NEGATIVE: u8 = 1 << 7;
pub const SR_MASK_OVERFLOW: u8 = 1 << 6;
pub const SR_MASK_B_FLAG: u8 = 1 << 5;
pub const SR_MASK_BREAK: u8 = 1 << 4;
pub const SR_MASK_DECIMAL: u8 = 1 << 3;
pub const SR_MASK_INTERRUPT: u8 = 1 << 2;
pub const SR_MASK_ZERO: u8 = 1 << 1;
pub const SR_MASK_CARRY: u8 = 1 << 0;

const DEFAULT_STATUS: Status = Status { status: SR_MASK_INTERRUPT };

#[derive(Clone, Copy)]
pub struct Status {
    status: u8,
}

impl Status {
    pub fn from_u8(status: u8) -> Status {
        Status { status }
    }

    pub fn get_as_u8(&self) -> u8 {
        self.status
    }

    pub fn get(&self, mask: u8) -> bool {
        self.status & mask > 0
    }

    pub fn set(&mut self, mask: u8, value: bool) {
        if value {
            self.status |= mask;
        } else {
            self.status &= !mask;
        }
    }
}

pub struct State {
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    pub stack_pointer: u8,

    program_counter: u16,
    next_pc: u16,

    status: Status,
}

impl State {
    pub fn new() -> State {
        let state = State {
            acc: 0,
            x: 0,
            y: 0,
            program_counter: 0,
            next_pc: 0,
            stack_pointer: 0,
            status: DEFAULT_STATUS,
        };

        state
    }

    pub fn clear(&mut self) {
        self.acc = 0;
        self.x = 0;
        self.y = 0;

        self.program_counter = 0;
        self.next_pc = 0;
        self.stack_pointer = 0;
        self.status = DEFAULT_STATUS;
    }

    pub fn calculate_relative_pc(&self, offset: i8) -> u16 {
        self.next_pc.wrapping_add(offset as u16)
    }

    pub fn get_next_pc(&self) -> u16 { self.next_pc }
    pub fn set_next_pc(&mut self, pc: u16) {
        self.next_pc = pc;
    }
    pub fn update_pc(&mut self) {
        self.program_counter = self.next_pc;
    }
    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn dec_sp(&mut self) { self.stack_pointer = self.stack_pointer.wrapping_sub(1); }
    pub fn inc_sp(&mut self) { self.stack_pointer = self.stack_pointer.wrapping_add(1); }

    pub fn get_status_field(&self, mask: u8) -> bool {
        self.status.get(mask)
    }

    pub fn get_status_ref(&self) -> &Status {
        &self.status
    }

    pub fn set_status_field(&mut self, mask: u8, value: bool) {
        self.status.set(mask, value);
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
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

        state.set_next_pc(state.calculate_relative_pc(5));
        state.set_next_pc(state.calculate_relative_pc(-5));
        assert_eq!(0, state.program_counter);

        state.set_next_pc(state.calculate_relative_pc(100));
        state.update_pc();
        assert_eq!(100, state.program_counter);

        state.set_next_pc(state.calculate_relative_pc(-50));
        state.update_pc();
        assert_eq!(50, state.program_counter);

        state.set_next_pc(state.calculate_relative_pc(-128));
        state.update_pc();
        assert_eq!(65458, state.program_counter);
    }

    #[test]
    fn test_set_status_field() {
        let mut state = State::new();

        let initial = super::SR_MASK_OVERFLOW | super::SR_MASK_CARRY | super::SR_MASK_ZERO | super::SR_MASK_INTERRUPT;

        state.set_status_field(initial, true);
        assert_eq!(initial, state.status.get_as_u8());
        state.set_status_field(super::SR_MASK_BREAK, true);
        assert_eq!(initial | super::SR_MASK_BREAK, state.status.get_as_u8());
        state.set_status_field(super::SR_MASK_BREAK, false);
        assert_eq!(initial, state.status.get_as_u8());
    }
}