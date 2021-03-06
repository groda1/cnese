use std::collections::HashMap;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::cpu::instruction::Instruction;
use crate::cpu::state;
use crate::nes::nes::NES;
use crate::gfx::render;

use super::window::RenderableWindow;
use super::super::font::Font;

static FRAME_BORDER_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
static FRAME_BACKGROUND_COLOR: (u8, u8, u8, u8) = (64, 64, 64, 255);

static FRAME_PADDING: i32 = 10;
static ROW_OFFSET: i32 = 20;
static ROW_OFFSET_SMALL: i32 = 10;

static INSTRUCTION_WINDOW_LINE_WRAP_OFFSET: usize = 3;
static INSTRUCTION_WINDOW_WIDTH: u32 = 300;
static REGISTER_WINDOW_WIDTH: u32 = 300;
const PPU_WINDOW_WIDTH: u32 = 300;

static MEMORY_WINDOW_WIDTH: u32 = 440;

pub struct InstructionWindow<'a> {
    instructions: Vec<Instruction>,
    instruction_offset: usize,
    instruction_rom_offset: usize,
    addr_to_instr_index: HashMap<usize, usize>,
    instr_to_addr: HashMap<usize, usize>,
    height: usize,

    font: &'a Font<'a>,
    secondary_font: &'a Font<'a>,
}

impl<'a> InstructionWindow<'a> {
    pub fn new(font: &'a Font<'a>,
               secondary_font: &'a Font<'a>,
               instructions: Vec<Instruction>,
               instruction_rom_offset: usize,
               height: usize) -> InstructionWindow<'a> {
        let mut window = InstructionWindow {
            font,
            secondary_font,
            instructions,
            instruction_offset: 0,
            instruction_rom_offset,
            addr_to_instr_index: HashMap::new(),
            instr_to_addr: HashMap::new(),
            height,
        };

        let mut i = 0;
        let mut addr = instruction_rom_offset;

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
        if self.addr_to_instr_index.get(&addr).is_none() { return; };
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

impl<'a> RenderableWindow for InstructionWindow<'a> {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {
        const TEXT_ADDR_OFFSET: i32 = 16;
        const TEXT_INSTRUCTION_OFFSET: i32 = 88;

        let pc = nes.get_cpu().get_state().get_pc() as usize;
        self.readjust(pc);

        render::window(canvas,
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
                render::render_text(canvas,
                                    self.font,
                                    x + FRAME_PADDING,
                                    y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                                    ">",
                )?;
            }

            render::render_text(canvas,
                                self.secondary_font,
                                x + TEXT_ADDR_OFFSET + FRAME_PADDING,
                                y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                                format!("{:04X}", memory_addr).as_str(),
            )?;

            render::render_text(canvas,
                                self.font,
                                x + TEXT_INSTRUCTION_OFFSET + FRAME_PADDING,
                                y + i as i32 * ROW_OFFSET + FRAME_PADDING,
                                instruction.format().as_str(),
            )?;

            memory_addr += instruction.get_size() as usize;
        }
        Ok(())
    }
}

pub struct FramerateCounter<'a> {
    font: &'a Font<'a>,
}

impl<'a> FramerateCounter<'a> {
    pub fn new(font: &'a Font) -> FramerateCounter<'a> {
        FramerateCounter { font }
    }
}

impl<'a> RenderableWindow for FramerateCounter<'a> {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {
        render::render_text_small(canvas, self.font, x, y,
                                  format!("FPS: {}", nes.get_actual_framerate()).as_str(),
        )?;
        render::render_text_small(canvas, self.font, x + 90, y,
                                  format!("Cycles: {}", nes.get_cpu().get_cycle_count()).as_str(),
        )?;
        render::render_text_small(canvas, self.font, x + 200, y,
                                  format!("Instructions: {}", nes.get_cpu().get_instruction_count()).as_str(),
        )?;

        Ok(())
    }
}

pub struct MemoryWindow<'a> {
    font: &'a Font<'a>,
    secondary_font: &'a Font<'a>,
    // scroll_offset: usize,
    data_start: u16,
    data_size: usize,
    height: usize,
}

impl<'a> MemoryWindow<'a> {
    pub fn new(
        font: &'a Font,
        secondary_font: &'a Font,
        data_start: u16,
        data_size: usize,
        height: usize) -> MemoryWindow<'a> {
        let window = MemoryWindow {
            font,
            secondary_font,
            data_start,
            data_size,
            height,
            // scroll_offset: 0,
        };

        window
    }
}

impl<'a> RenderableWindow for MemoryWindow<'a> {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {
        const TEXT_MEMORY_OFFSET: i32 = 35;
        render::window(canvas,
                       x,
                       y,
                       MEMORY_WINDOW_WIDTH,
                       (self.height * ROW_OFFSET_SMALL as usize + (FRAME_PADDING * 2) as usize) as u32,
                       Color::from(FRAME_BORDER_COLOR),
                       Color::from(FRAME_BACKGROUND_COLOR),
        )?;

        let mut i = 0;
        let bus = nes.get_databus();

        while i < self.height && (i + 1) * 16 <= self.data_size {
            let mut row = vec![0 as u8; 16];
            for j in 0..16 {
                row[j] = bus.read(self.data_start + (i * 16) as u16 + j as u16)
            }

            render::render_text_small(canvas,
                                      self.secondary_font,
                                      x + FRAME_PADDING,
                                      y + FRAME_PADDING + (i as i32 * ROW_OFFSET_SMALL),
                                      format!("{:04X}", self.data_start as usize + (i * 16)).as_str(),
            )?;

            let line = format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}  {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
                               row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7],
                               row[8], row[9], row[10], row[11], row[12], row[13], row[14], row[15]
            );

            render::render_text_small(canvas,
                                      self.font,
                                      x + FRAME_PADDING + TEXT_MEMORY_OFFSET,
                                      y + FRAME_PADDING + (i as i32 * ROW_OFFSET_SMALL),
                                      line.as_str(),
            )?;

            i += 1;
        }

        Ok(())
    }
}

pub struct RegisterWindow<'a> {
    font: &'a Font<'a>,
    secondary_font: &'a Font<'a>,
}

impl<'a> RegisterWindow<'a> {
    pub fn new(font: &'a Font<'a>,
               secondary_font: &'a Font<'a>) -> RegisterWindow<'a> {
        RegisterWindow { font, secondary_font }
    }
}

impl<'a> RenderableWindow for RegisterWindow<'a> {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {
        const EXTRA_ROW_OFFSET: i32 = 5;
        const STATUS_FLAG_OFFSET: i32 = 30;

        let state = nes.get_cpu().get_state();

        render::window(canvas,
                       x,
                       y,
                       REGISTER_WINDOW_WIDTH,
                       (FRAME_PADDING * 2 + (ROW_OFFSET * 4)) as u32,
                       Color::from(FRAME_BORDER_COLOR),
                       Color::from(FRAME_BACKGROUND_COLOR),
        )?;

        render::render_text(canvas,
                            self.font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING,
                            "A:    X:    Y:",
        )?;
        render::render_text(canvas,
                            self.secondary_font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING,
                            format!("  ${:02X}   ${:02X}   ${:02X}", state.acc, state.x, state.y).as_str(),
        )?;

        render::render_text(canvas,
                            self.font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET + EXTRA_ROW_OFFSET,
                            "PC:      SP:",
        )?;
        render::render_text(canvas,
                            self.secondary_font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET + EXTRA_ROW_OFFSET,
                            format!("   ${:04X}    ${:02X}", state.get_pc(), state.stack_pointer).as_str(),
        )?;


        render::render_text(canvas,
                            if state.get_status_field(state::SR_MASK_NEGATIVE) { self.font } else { self.secondary_font },
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET * 3,
                            "N",
        )?;

        render::render_text(canvas,
                            if state.get_status_field(state::SR_MASK_OVERFLOW) { self.font } else { self.secondary_font },
                            x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 1),
                            y + FRAME_PADDING + ROW_OFFSET * 3,
                            "V",
        )?;

        render::render_text(canvas,
                            if state.get_status_field(state::SR_MASK_DECIMAL) { self.font } else { self.secondary_font },
                            x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 4),
                            y + FRAME_PADDING + ROW_OFFSET * 3,
                            "D",
        )?;

        render::render_text(canvas,
                            if state.get_status_field(state::SR_MASK_INTERRUPT) { self.font } else { self.secondary_font },
                            x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 5),
                            y + FRAME_PADDING + ROW_OFFSET * 3,
                            "I",
        )?;

        render::render_text(canvas,
                            if state.get_status_field(state::SR_MASK_ZERO) { self.font } else { self.secondary_font },
                            x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 6),
                            y + FRAME_PADDING + ROW_OFFSET * 3,
                            "Z",
        )?;

        render::render_text(canvas,
                            if state.get_status_field(state::SR_MASK_CARRY) { self.font } else { self.secondary_font },
                            x + FRAME_PADDING + (STATUS_FLAG_OFFSET * 7),
                            y + FRAME_PADDING + ROW_OFFSET * 3,
                            "C",
        )?;

        Ok(())
    }
}


pub struct PpuWindow<'a> {
    font: &'a Font<'a>,
    secondary_font: &'a Font<'a>,
}

impl<'a> PpuWindow<'a> {
    pub fn new(font: &'a Font<'a>,
               secondary_font: &'a Font<'a>) -> PpuWindow<'a> {
        PpuWindow { font, secondary_font }
    }
}

impl<'a> RenderableWindow for PpuWindow<'a> {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {

        let ppuctrl = nes.get_ppu().get_ppuctrl();
        let ppumask = nes.get_ppu().get_ppuctrl();
        let ppustatus = nes.get_ppu().get_ppustatus();
        let oamaddr = nes.get_ppu().get_oamaddr();
        let ppuscroll = nes.get_ppu().get_ppuscroll();
        let ppuaddr = nes.get_ppu().get_ppuaddr();

        render::window(canvas,
                       x,
                       y,
                       PPU_WINDOW_WIDTH,
                       (FRAME_PADDING * 2 + (ROW_OFFSET * 6)) as u32,
                       Color::from(FRAME_BORDER_COLOR),
                       Color::from(FRAME_BACKGROUND_COLOR),
        )?;

        render::render_text(canvas,
                            self.font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING,
                            "PPUCTRL:",
        )?;
        render::render_text(canvas,
                            self.secondary_font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING,
                            format!("          ${:02X}", ppuctrl).as_str(),
        )?;

        render::render_text(canvas,
                            self.font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET,
                            "PPUMASK:",
        )?;
        render::render_text(canvas,
                            self.secondary_font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET,
                            format!("          ${:02X}", ppumask).as_str(),
        )?;

        render::render_text(canvas,
                            self.font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET * 2,
                            "PPUSTATUS:",
        )?;
        render::render_text(canvas,
                            self.secondary_font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET * 2,
                            format!("          ${:02X}", ppustatus).as_str(),
        )?;

        render::render_text(canvas,
                            self.font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET * 3,
                            "OAMADDR:",
        )?;
        render::render_text(canvas,
                            self.secondary_font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET * 3,
                            format!("          ${:02X}", oamaddr).as_str(),
        )?;

        render::render_text(canvas,
                            self.font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET * 4,
                            "PPUSCROLL:",
        )?;
        render::render_text(canvas,
                            self.secondary_font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET * 4,
                            format!("          ${:04X}", ppuscroll).as_str(),
        )?;

        render::render_text(canvas,
                            self.font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET * 5,
                            "PPUADDR:",
        )?;
        render::render_text(canvas,
                            self.secondary_font,
                            x + FRAME_PADDING,
                            y + FRAME_PADDING + ROW_OFFSET * 5,
                            format!("          ${:04X}", ppuaddr).as_str(),
        )?;

        Ok(())
    }
}