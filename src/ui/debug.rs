use std::collections::HashMap;
use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::{WindowContext, Window};
use sdl2::ttf::Font;
use sdl2::pixels::Color;

use super::super::cpu::instruction::Instruction;
use super::super::cpu::databus;
use super::super::cpu::state;

use super::util;

use crate::nes::nes::NES;

static TEXT_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
static TEXT_COLOR_DARK: (u8, u8, u8, u8) = (175, 175, 175, 175);

static FRAME_BORDER_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
static FRAME_BACKGROUND_COLOR: (u8, u8, u8, u8) = (64, 64, 64, 255);

static FRAME_PADDING: i32 = 10;
static ROW_OFFSET: i32 = 20;
static ROW_OFFSET_SMALL: i32 = 10;

static INSTRUCTION_WINDOW_LINE_WRAP_OFFSET: usize = 3;
static INSTRUCTION_WINDOW_WIDTH: u32 = 300;

static REGISTER_WINDOW_WIDTH: u32 = 300;

static MEMORY_WINDOW_WIDTH: u32 = 440;

pub fn create_instruction_window<'a>(texture_creator: &'a TextureCreator<WindowContext>,
                                     font: &'a Font<'a, 'static>,
                                     height: usize,
                                     instructions: Vec<Instruction>) -> DebugWindow<'a> {
    let instruction_window = InstructionWindow::new(instructions, height);

    DebugWindow::new(texture_creator, font, Box::new(instruction_window))
}

pub fn create_register_window<'a>(texture_creator: &'a TextureCreator<WindowContext>,
                                  font: &'a Font<'a, 'static>) -> DebugWindow<'a> {
    let register_window = RegisterWindow {};

    DebugWindow::new(texture_creator, font, Box::new(register_window))
}

pub fn create_memory_window<'a>(texture_creator: &'a TextureCreator<WindowContext>,
                                font: &'a Font<'a, 'static>,
                                data_start: u16,
                                data_size: usize,
                                height: usize) -> DebugWindow<'a> {
    let memory_window = MemoryWindow::new(data_start, data_size, height);

    DebugWindow::new(texture_creator, font, Box::new(memory_window))
}


pub struct DebugWindow<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &'a Font<'a, 'static>,
    x: i32,
    y: i32,
    active: bool,
    renderable_window: Box<dyn RenderableWindow>,
}

impl<'a> DebugWindow<'a> {
    fn new(texture_creator: &'a TextureCreator<WindowContext>,
           font: &'a Font<'a, 'static>,
           renderable_window: Box<dyn RenderableWindow>) -> DebugWindow<'a> {
        let window = DebugWindow {
            texture_creator,
            font,
            x: 0,
            y: 0,
            active: false,
            renderable_window,
        };

        window
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>, nes: &NES) -> Result<(), String> {
        if self.active {
            self.renderable_window.render(canvas, self.texture_creator, self.font, self.x, self.y, nes)?;
        }
        Ok(())
    }
}

trait RenderableWindow {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              texture_creator: &TextureCreator<WindowContext>,
              font: &Font,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String>;
}

pub struct MemoryWindow {
    scroll_offset: usize,
    data_start: u16,
    data_size: usize,
    height: usize,
}

impl MemoryWindow {
    pub fn new(
        data_start: u16,
        data_size: usize,
        height: usize) -> MemoryWindow {
        let window = MemoryWindow {
            data_start,
            data_size,
            height,
            scroll_offset: 0,
        };

        window
    }
}

impl<'a> RenderableWindow for MemoryWindow {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              texture_creator: &TextureCreator<WindowContext>,
              font: &Font,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {
        const TEXT_MEMORY_OFFSET: i32 = 35;
        util::render_window(canvas,
                            x,
                            y,
                            MEMORY_WINDOW_WIDTH,
                            (self.height * ROW_OFFSET_SMALL as usize + (FRAME_PADDING * 2) as usize) as u32,
                            Color::from(FRAME_BORDER_COLOR),
                            Color::from(FRAME_BACKGROUND_COLOR),
        )?;

        let mut i = 0;
        let data = nes.get_databus().read_slice(self.data_start, self.data_size);

        while i < self.height && (i + 1) * 16 <= self.data_size {
            let row = &data[(i * 16)..(i + 1) * 16];

            util::render_text_small(canvas,
                                    texture_creator,
                                    font,
                                    x + FRAME_PADDING,
                                    y + FRAME_PADDING + (i as i32 * ROW_OFFSET_SMALL),
                                    format!("{:04X}", self.data_start as usize + (i * 16)).as_str(),
                                    Color::from(TEXT_COLOR_DARK),
            )?;

            let line = format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}  {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
                               row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7],
                               row[8], row[9], row[10], row[11], row[12], row[13], row[14], row[15]
            );

            util::render_text_small(canvas,
                                    texture_creator,
                                    font,
                                    x + FRAME_PADDING + TEXT_MEMORY_OFFSET,
                                    y + FRAME_PADDING + (i as i32 * ROW_OFFSET_SMALL),
                                    line.as_str(),
                                    Color::from(TEXT_COLOR),
            )?;


            i += 1;
        }


        Ok(())
    }
}

pub struct RegisterWindow {}

impl<'a> RenderableWindow for RegisterWindow {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              texture_creator: &TextureCreator<WindowContext>,
              font: &Font,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {
        const EXTRA_ROW_OFFSET: i32 = 5;
        const STATUS_FLAG_OFFSET: i32 = 30;

        let state = nes.get_cpu().get_state();

        util::render_window(canvas,
                            x,
                            y,
                            REGISTER_WINDOW_WIDTH,
                            (FRAME_PADDING * 2 + (ROW_OFFSET * 4)) as u32,
                            Color::from(FRAME_BORDER_COLOR),
                            Color::from(FRAME_BACKGROUND_COLOR),
        )?;

        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING,
                          "A:    X:    Y:",
                          Color::from(TEXT_COLOR),
        )?;
        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING,
                          format!("  ${:02X}   ${:02X}   ${:02X}", state.acc, state.x, state.y).as_str(),
                          Color::from(TEXT_COLOR_DARK),
        )?;

        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING + ROW_OFFSET + EXTRA_ROW_OFFSET,
                          "PC:      SP:",
                          Color::from(TEXT_COLOR),
        )?;
        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING + ROW_OFFSET + EXTRA_ROW_OFFSET,
                          format!("   ${:04X}    ${:02X}", state.get_pc(), state.get_sp()).as_str(),
                          Color::from(TEXT_COLOR_DARK),
        )?;

        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "N",
                          if state.get_status(state::SR_MASK_NEGATIVE) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "N",
                          if state.get_status(state::SR_MASK_NEGATIVE) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 1),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "V",
                          if state.get_status(state::SR_MASK_OVERFLOW) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 4),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "D",
                          if state.get_status(state::SR_MASK_DECIMAL) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 5),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "I",
                          if state.get_status(state::SR_MASK_INTERRUPT) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 6),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "Z",
                          if state.get_status(state::SR_MASK_ZERO) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;
        util::render_text(canvas,
                          texture_creator,
                          font,
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 7),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "C",
                          if state.get_status(state::SR_MASK_CARRY) { Color::from(TEXT_COLOR) } else { Color::from(TEXT_COLOR_DARK) },
        )?;

        Ok(())
    }
}


pub struct InstructionWindow {
    instructions: Vec<Instruction>,
    instruction_offset: usize,
    instruction_rom_offset: usize,
    addr_to_instr_index: HashMap<usize, usize>,
    instr_to_addr: HashMap<usize, usize>,
    height: usize,
}

impl InstructionWindow {
    fn new(instructions: Vec<Instruction>,
           height: usize) -> InstructionWindow {
        let mut window = InstructionWindow {
            instructions,
            instruction_offset: 0,
            instruction_rom_offset: databus::CARTRIDGE_SPACE_OFFSET,
            addr_to_instr_index: HashMap::new(),
            instr_to_addr: HashMap::new(),
            height,
        };

        let mut i = 0;
        let mut addr = databus::CARTRIDGE_SPACE_OFFSET;

        let instructions = &window.instructions;

        for instr in instructions {
            window.addr_to_instr_index.insert(addr, i);
            window.instr_to_addr.insert(i, addr);

            addr += instr.get_size() as usize;
            i += 1;
        }

        window
    }

    fn readjust(&mut self, addr: usize) {
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
}

impl<'a> RenderableWindow for InstructionWindow {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              texture_creator: &TextureCreator<WindowContext>,
              font: &Font,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {
        const TEXT_ADDR_OFFSET: i32 = 16;
        const TEXT_INSTRUCTION_OFFSET: i32 = 88;

        let pc = nes.get_cpu().get_state().get_pc() as usize;
        self.readjust(pc);

        util::render_window(canvas,
                            x,
                            y,
                            INSTRUCTION_WINDOW_WIDTH,
                            self.height as u32 * ROW_OFFSET as u32 + (FRAME_PADDING as u32 * 2),
                            Color::from(FRAME_BORDER_COLOR),
                            Color::from(FRAME_BACKGROUND_COLOR))?;

        let mut memory_addr = self.instruction_rom_offset;

        for i in 0..self.height {

            //for i in (0 + self.instruction_offset)..(INSTRUCTION_WINDOW_HEIGHT + self.instruction_offset) {
            let instruction = self.instructions[i + self.instruction_offset];

            if pc == memory_addr {
                util::render_text(canvas,
                                  texture_creator,
                                  font,
                                  x + FRAME_PADDING,
                                  y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                                  ">",
                                  Color::from(TEXT_COLOR),
                )?;
            }

            util::render_text(canvas,
                              texture_creator,
                              font,
                              x + TEXT_ADDR_OFFSET + FRAME_PADDING,
                              y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                              format!("{:04X}", memory_addr).as_str(),
                              Color::from(TEXT_COLOR_DARK),
            )?;

            util::render_text(canvas,
                              texture_creator,
                              font,
                              x + TEXT_INSTRUCTION_OFFSET + FRAME_PADDING,
                              y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                              instruction.format().as_str(), Color::from(TEXT_COLOR),
            )?;

            memory_addr += instruction.get_size() as usize;
        }
        Ok(())
    }
}
