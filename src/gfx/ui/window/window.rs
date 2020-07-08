use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::nes::nes::NES;
use crate::cpu::instruction::Instruction;

use super::debug;
use super::patterntable;
use super::ppu_framebuffer;
use super::super::font::Font;

pub fn create_instruction_window<'a>(font: &'a Font<'a>,
                                     secondary_font: &'a Font<'a>,
                                     height: usize,
                                     instructions: Vec<Instruction>,
                                     instruction_rom_offest: usize) -> CneseWindow<'a> {
    let instruction_window = debug::InstructionWindow::new(
        font, secondary_font, instructions,instruction_rom_offest, height);

    CneseWindow::new(Box::new(instruction_window))
}

pub fn create_register_window<'a>(font: &'a Font<'a>, secondary_font: &'a Font<'a>) -> CneseWindow<'a> {
    let register_window = debug::RegisterWindow::new(font, secondary_font);

    CneseWindow::new(Box::new(register_window))
}

pub fn create_memory_window<'a>(font: &'a Font<'a>,
                                secondary_font: &'a Font<'a>,
                                data_start: u16,
                                data_size: usize,
                                height: usize) -> CneseWindow<'a> {
    let memory_window = debug::MemoryWindow::new(font, secondary_font, data_start, data_size, height);

    CneseWindow::new(Box::new(memory_window))
}

pub fn create_framerate_window<'a>(font: &'a Font<'a>) -> CneseWindow<'a> {
    let counter = debug::FramerateCounter::new(font);
    CneseWindow::new(Box::new(counter))
}

pub fn create_patterntable_window(texture_creator: &TextureCreator<WindowContext>,
                                  width: u32,
                                  height: u32) -> CneseWindow {
    let patterntable = patterntable::PatternTableWindow::new(
        texture_creator, width, height, 0);
    CneseWindow::new(Box::new(patterntable))
}

pub fn create_framebuffer_window(texture_creator: &TextureCreator<WindowContext>,
                                  width: u32,
                                  height: u32) -> CneseWindow {
    let framebuffer = ppu_framebuffer::FramebufferWindow::new(
        texture_creator, width, height);
    CneseWindow::new(Box::new(framebuffer))
}

pub fn create_ppu_window<'a>(font: &'a Font<'a>, secondary_font: &'a Font<'a>) -> CneseWindow<'a> {
    let ppu_window = debug::PpuWindow::new(font, secondary_font);

    CneseWindow::new(Box::new(ppu_window))
}

pub struct CneseWindow<'a> {
    x: i32,
    y: i32,
    active: bool,
    renderable_window: Box<dyn RenderableWindow + 'a>,
}

impl<'a> CneseWindow<'a> {
    fn new(renderable_window: Box<dyn RenderableWindow + 'a>) -> CneseWindow<'a> {
        let window = CneseWindow {
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
            self.renderable_window.render(canvas, self.x, self.y, nes)?;
        }
        Ok(())
    }
}

pub trait RenderableWindow {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String>;
}


