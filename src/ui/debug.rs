use std::collections::HashMap;
use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::{WindowContext, Window};
use sdl2::ttf::Font;
use sdl2::pixels::Color;

use super::super::cpu::instruction::Instruction;
use super::super::cpu::databus;
use super::super::cpu::state::State;
use super::super::cpu::state;

use super::util;

static TEXT_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
static TEXT_COLOR_DARK: (u8, u8, u8, u8) = (175, 175, 175, 175);

static FRAME_BORDER_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
static FRAME_BACKGROUND_COLOR: (u8, u8, u8, u8) = (64, 64, 64, 255);

static FRAME_PADDING: i32 = 15;
static ROW_OFFSET: i32 = 20;

static INSTRUCTION_WINDOW_LINE_WRAP_OFFSET: usize = 3;
static INSTRUCTION_WINDOW_WIDTH: u32 = 300;

static REGISTER_WINDOW_WIDTH: u32 = 300;
static REGISTER_WINDOW_HEIGHT: u32 = 110;


pub struct RegisterWindow<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &'a Font<'a, 'static>,
    x: i32,
    y: i32,
}

impl<'a> RegisterWindow<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>,
               font: &'a Font<'a, 'static>) -> RegisterWindow<'a> {
        let window = RegisterWindow {
            texture_creator,
            font,
            x: 0,
            y: 0,
        };

        window
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, state: &State) -> Result<(), String> {
        const LINE_OFFSET: i32 = 5;
        const STATUS_FLAG_OFFSET: i32 = 30;

        util::render_window(canvas,
                            self.x,
                            self.y,
                            REGISTER_WINDOW_WIDTH,
                            REGISTER_WINDOW_HEIGHT,
                            Color::from(FRAME_BORDER_COLOR),
                            Color::from(FRAME_BACKGROUND_COLOR),
        );

        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING,
                          self.y + FRAME_PADDING,
                          "A:    X:    Y:",
                          Color::from(TEXT_COLOR),
        )?;
        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING,
                          self.y + FRAME_PADDING,
                          format!("  ${:02X}   ${:02X}   ${:02X}", state.acc, state.x, state.y).as_str(),
                          Color::from(TEXT_COLOR_DARK),
        )?;

        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING,
                          self.y + FRAME_PADDING + ROW_OFFSET + LINE_OFFSET,
                          "PC:      SP:",
                          Color::from(TEXT_COLOR),
        )?;
        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING,
                          self.y + FRAME_PADDING + ROW_OFFSET + LINE_OFFSET,
                          format!("   ${:04X}    ${:02X}", state.get_pc(), state.get_sp()).as_str(),
                          Color::from(TEXT_COLOR_DARK),
        )?;

        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING,
                          self.y + FRAME_PADDING + (ROW_OFFSET * 2) + 15,
                          "N",
                          if state.get_status(state::SR_MASK_NEGATIVE) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING,
                          self.y + FRAME_PADDING + (ROW_OFFSET * 2) + 15,
                          "N",
                          if state.get_status(state::SR_MASK_NEGATIVE) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 1),
                          self.y + FRAME_PADDING + (ROW_OFFSET * 2) + 15,
                          "V",
                          if state.get_status(state::SR_MASK_OVERFLOW) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 4),
                          self.y + FRAME_PADDING + (ROW_OFFSET * 2) + 15,
                          "D",
                          if state.get_status(state::SR_MASK_DECIMAL) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 5),
                          self.y + FRAME_PADDING + (ROW_OFFSET * 2) + 15,
                          "I",
                          if state.get_status(state::SR_MASK_INTERRUPT) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 6),
                          self.y + FRAME_PADDING + (ROW_OFFSET * 2) + 15,
                          "Z",
                          if state.get_status(state::SR_MASK_ZERO) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          self.texture_creator,
                          self.font,
                          self.x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 7),
                          self.y + FRAME_PADDING + (ROW_OFFSET * 2) + 15,
                          "C",
                          if state.get_status(state::SR_MASK_CARRY) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;






        Ok(())
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}


pub struct InstructionWindow<'a> {
    instructions: &'a Vec<Instruction>,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &'a Font<'a, 'static>,
    instruction_offset: usize,
    instruction_rom_offset: usize,
    addr_to_instr_index: HashMap<usize, usize>,
    instr_to_addr: HashMap<usize, usize>,
    x: i32,
    y: i32,
    height: usize,
}

impl<'a> InstructionWindow<'a> {
    pub fn new(instructions: &'a Vec<Instruction>,
               texture_creator: &'a TextureCreator<WindowContext>,
               font: &'a Font<'a, 'static>,
               height: usize) -> InstructionWindow<'a> {
        let mut window = InstructionWindow {
            instructions,
            texture_creator,
            font,
            instruction_offset: 0,
            instruction_rom_offset: databus::CARTRIDGE_SPACE_OFFSET,
            addr_to_instr_index: HashMap::new(),
            instr_to_addr: HashMap::new(),
            x: 0,
            y: 0,
            height,
        };

        let mut i = 0;
        let mut addr = databus::CARTRIDGE_SPACE_OFFSET;
        for instr in instructions {
            window.addr_to_instr_index.insert(addr, i);
            window.instr_to_addr.insert(i, addr);

            addr += instr.get_size() as usize;
            i += 1;
        }

        window
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn readjust(&mut self, addr: usize) {
        let active_instr_index = *self.addr_to_instr_index.get(&addr).unwrap();

        if active_instr_index >
            (self.height + self.instruction_offset - INSTRUCTION_WINDOW_LINE_WRAP_OFFSET) {
            self.instruction_offset = active_instr_index - (self.height - INSTRUCTION_WINDOW_LINE_WRAP_OFFSET);
            self.instruction_rom_offset = *self.instr_to_addr.get(&self.instruction_offset).unwrap();
        } else if active_instr_index < self.instruction_offset {
            let mut new_addr: i32 = active_instr_index as i32 - INSTRUCTION_WINDOW_LINE_WRAP_OFFSET as i32;
            if new_addr < 0 {
                new_addr = 0;
            }
            self.instruction_offset = new_addr as usize;
            self.instruction_rom_offset = *self.instr_to_addr.get(&self.instruction_offset).unwrap();
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, pc: u16) -> Result<(), String> {
        const TEXT_ADDR_OFFSET: i32 = 16;
        const TEXT_INSTRUCTION_OFFSET: i32 = 88;

        util::render_window(canvas,
                            self.x,
                            self.y,
                            INSTRUCTION_WINDOW_WIDTH,
                            (self.height as u32 * ROW_OFFSET as u32 + (FRAME_PADDING as u32 * 2)),
                            Color::from(FRAME_BORDER_COLOR),
                            Color::from(FRAME_BACKGROUND_COLOR));

        let mut memory_addr = self.instruction_rom_offset;

        for i in 0..self.height {

            //for i in (0 + self.instruction_offset)..(INSTRUCTION_WINDOW_HEIGHT + self.instruction_offset) {
            let instruction = self.instructions[i + self.instruction_offset];

            if pc as usize == memory_addr {
                util::render_text(canvas,
                                  self.texture_creator,
                                  self.font,
                                  self.x + FRAME_PADDING,
                                  self.y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                                  ">",
                                  Color::from(TEXT_COLOR))?;
            }

            util::render_text(canvas,
                              self.texture_creator,
                              self.font,
                              self.x + TEXT_ADDR_OFFSET + FRAME_PADDING,
                              self.y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                              format!("{:04X}", memory_addr).as_str(),
                              Color::from(TEXT_COLOR_DARK))?;

            util::render_text(canvas,
                              self.texture_creator,
                              self.font,
                              self.x + TEXT_INSTRUCTION_OFFSET + FRAME_PADDING,
                              self.y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                              instruction.format().as_str(), Color::from(TEXT_COLOR))?;

            memory_addr += instruction.get_size() as usize;
        }
        Ok(())
    }
}

