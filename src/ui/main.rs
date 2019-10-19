extern crate sdl2;

use std::env;
use std::path::Path;
use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::render::{TextureQuery, Canvas, TextureCreator};
use sdl2::Sdl;
use sdl2::video::{Window, WindowContext};
use sdl2::ttf::Font;
use sdl2::pixels::Color;

use super::super::cpu::databus;
use super::super::cpu::databus::Databus;
use super::super::cpu::cpu::Cpu;
use super::super::instruction;
use super::super::instruction::Instruction;

use super::debug::{InstructionWindow, RegisterWindow};

static SCREEN_WIDTH: u32 = 1200;
static SCREEN_HEIGHT: u32 = 600;

static BACKGROUND_COLOR: (u8, u8, u8, u8) = (128, 128, 128, 255);


fn render(canvas: &mut Canvas<Window>,
          instr_window: &mut InstructionWindow,
          register_window: &mut RegisterWindow,
          cpu: &Cpu,
          bus: &Databus) {
    let pc = cpu.get_state().get_pc();

    canvas.set_draw_color(Color::from(BACKGROUND_COLOR));
    canvas.clear();


    instr_window.readjust(pc as usize);
    instr_window.render(canvas, pc);

    register_window.render(canvas, cpu.get_state());

    canvas.present();
}

pub fn run(cpu: &mut Cpu, bus: &mut Databus) -> Result<(), String> {
    let deassembled_instructions = instruction::deassemble(bus.get_cartridge());

    let pc = cpu.get_state().get_pc();


    let font_path: &Path = Path::new("src/ui/resources/nesfont.fon");

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsys.window("cnese", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = ttf_context.load_font(font_path, 128)?;

    let mut instr_window = InstructionWindow::new(&deassembled_instructions, &texture_creator, &font, 25);
    instr_window.set_pos(20, 20);

    let mut register_window = RegisterWindow::new(&texture_creator, &font);
    register_window.set_pos(350, 20);

    render(&mut canvas, &mut instr_window, &mut register_window, cpu, bus);

    //render(&mut canvas, &texture_creator, &font, cpu, bus, &deassembled_instructions);

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    cpu.tick(bus);
                    render(&mut canvas, &mut instr_window, &mut register_window, cpu, bus);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

