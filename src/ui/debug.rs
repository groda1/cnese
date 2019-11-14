use std::collections::HashMap;
use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::{WindowContext, Window};
use sdl2::pixels::Color;

use super::super::cpu::instruction::Instruction;
use super::super::cpu::databus;
use super::super::cpu::state;

use super::util;
use super::font::Font;

use crate::nes::nes::NES;

static FRAME_BORDER_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
static FRAME_BACKGROUND_COLOR: (u8, u8, u8, u8) = (64, 64, 64, 255);

static FRAME_PADDING: i32 = 10;
static ROW_OFFSET: i32 = 20;
static ROW_OFFSET_SMALL: i32 = 10;

static INSTRUCTION_WINDOW_LINE_WRAP_OFFSET: usize = 3;
static INSTRUCTION_WINDOW_WIDTH: u32 = 300;

static REGISTER_WINDOW_WIDTH: u32 = 300;

static MEMORY_WINDOW_WIDTH: u32 = 440;

pub fn create_instruction_window<'a>(font: &'a Font<'a>,
                                     secondary_font: &'a Font<'a>,
                                     height: usize,
                                     instructions: Vec<Instruction>) -> DebugWindow<'a> {
    let instruction_window = InstructionWindow::new(instructions, height);

    DebugWindow::new(font, secondary_font, Box::new(instruction_window))
}

pub fn create_register_window<'a>(font: &'a Font<'a>, secondary_font: &'a Font<'a>) -> DebugWindow<'a> {
    let register_window = RegisterWindow {};

    DebugWindow::new(font, secondary_font, Box::new(register_window))
}

pub fn create_memory_window<'a>(font: &'a Font<'a>,
                                secondary_font: &'a Font<'a>,
                                data_start: u16,
                                data_size: usize,
                                height: usize) -> DebugWindow<'a> {
    let memory_window = MemoryWindow::new(data_start, data_size, height);

    DebugWindow::new(font, secondary_font, Box::new(memory_window))
}

pub fn create_framerate_window<'a>(font: &'a Font<'a>,
                                   secondary_font: &'a Font<'a>)-> DebugWindow<'a> {

    let counter = FramerateCounter::new();
    DebugWindow::new(font, secondary_font, Box::new(counter))
}

pub struct DebugWindow<'a> {
    font: &'a Font<'a>,
    secondary_font: &'a Font<'a>,
    x: i32,
    y: i32,
    active: bool,
    renderable_window: Box<dyn RenderableWindow>,
}

impl<'a> DebugWindow<'a> {
    fn new(font: &'a Font<'a>,
           secondary_font: &'a Font<'a>,
           renderable_window: Box<dyn RenderableWindow>) -> DebugWindow<'a> {
        let window = DebugWindow {
            font,
            secondary_font,
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
            self.renderable_window.render(canvas, self.font, self.secondary_font, self.x, self.y, nes)?;
        }
        Ok(())
    }
}

trait RenderableWindow {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              font: &Font,
              secondary_font: &Font,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String>;
}

pub struct FramerateCounter {}

impl FramerateCounter {
    pub fn new() -> FramerateCounter {
        FramerateCounter { }
    }
}

impl<'a> RenderableWindow for FramerateCounter {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              font: &Font,
              secondary_font: &Font,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {
        util::render_text_small(canvas, font, x, y,
                                format!("FPS: {}", nes.get_frames_dropped()).as_str(),
        )?;

        Ok(())
    }
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
              font: &Font,
              secondary_font: &Font,
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
                                    secondary_font,
                                    x + FRAME_PADDING,
                                    y + FRAME_PADDING + (i as i32 * ROW_OFFSET_SMALL),
                                    format!("{:04X}", self.data_start as usize + (i * 16)).as_str(),
            )?;

            let line = format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}  {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
                               row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7],
                               row[8], row[9], row[10], row[11], row[12], row[13], row[14], row[15]
            );

            util::render_text_small(canvas,
                                    font,
                                    x + FRAME_PADDING + TEXT_MEMORY_OFFSET,
                                    y + FRAME_PADDING + (i as i32 * ROW_OFFSET_SMALL),
                                    line.as_str(),
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
              font: &Font,
              secondary_font: &Font,
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
                          font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING,
                          "A:    X:    Y:",
        )?;
        util::render_text(canvas,
                          secondary_font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING,
                          format!("  ${:02X}   ${:02X}   ${:02X}", state.acc, state.x, state.y).as_str(),
        )?;

        util::render_text(canvas,
                          font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING + ROW_OFFSET + EXTRA_ROW_OFFSET,
                          "PC:      SP:",
        )?;
        util::render_text(canvas,
                          secondary_font,
                          x + FRAME_PADDING,
                          y + FRAME_PADDING + ROW_OFFSET + EXTRA_ROW_OFFSET,
                          format!("   ${:04X}    ${:02X}", state.get_pc(), state.get_sp()).as_str(),
        )?;


        util::render_text(canvas,
                          if state.get_status(state::SR_MASK_NEGATIVE) { font } else { secondary_font },
                          x + FRAME_PADDING,
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "N",
        )?;

        util::render_text(canvas,
                          if state.get_status(state::SR_MASK_OVERFLOW) { font } else { secondary_font },
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 1),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "V",
        )?;

        util::render_text(canvas,
                          if state.get_status(state::SR_MASK_DECIMAL) { font } else { secondary_font },
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 4),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "D",
        )?;

        util::render_text(canvas,
                          if state.get_status(state::SR_MASK_INTERRUPT) { font } else { secondary_font },
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 5),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "I",
        )?;

        util::render_text(canvas,
                          if state.get_status(state::SR_MASK_ZERO) { font } else { secondary_font },
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 6),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "Z",
        )?;

        util::render_text(canvas,
                          if state.get_status(state::SR_MASK_CARRY) { font } else { secondary_font },
                          x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 7),
                          y + FRAME_PADDING + ROW_OFFSET * 3,
                          "C",
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
              font: &Font,
              secondary_font: &Font,
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
            let instruction = self.instructions[i + self.instruction_offset];

            if pc == memory_addr {
                util::render_text(canvas,
                                  font,
                                  x + FRAME_PADDING,
                                  y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                                  ">",
                )?;
            }

            util::render_text(canvas,
                              secondary_font,
                              x + TEXT_ADDR_OFFSET + FRAME_PADDING,
                              y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                              format!("{:04X}", memory_addr).as_str(),
            )?;

            util::render_text(canvas,
                              font,
                              x + TEXT_INSTRUCTION_OFFSET + FRAME_PADDING,
                              y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                              instruction.format().as_str(),
            )?;

            memory_addr += instruction.get_size() as usize;
        }
        Ok(())
    }
}
