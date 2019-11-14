extern crate sdl2;

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use super::debug;
use super::debug::DebugWindow;

use crate::nes::nes::NES;
use crate::cpu::instruction;
use crate::ui::font::Font;

static SCREEN_WIDTH: u32 = 1250;
static SCREEN_HEIGHT: u32 = 600;

static BACKGROUND_COLOR: (u8, u8, u8, u8) = (128, 128, 128, 255);
static TEXT_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 255);
static TEXT_COLOR_DARK: (u8, u8, u8, u8) = (175, 175, 175, 175);

fn render(canvas: &mut Canvas<Window>,
          windows: &mut Vec<&mut DebugWindow>,
          nes: &NES) -> Result<(), String> {
    canvas.set_draw_color(Color::from(BACKGROUND_COLOR));
    canvas.clear();

    for window in windows {
        window.render(canvas, nes)?;
    }
    canvas.present();

    Ok(())
}

pub fn run(nes: &mut NES) -> Result<(), String> {
    let deassembled_instructions = instruction::deassemble(nes.get_databus().get_cartridge());

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

    let ttf_font = ttf_context.load_font(font_path, 128)?;

    let font = super::font::Font::new(&texture_creator, &ttf_font, Color::from(TEXT_COLOR));
    let dark_font = super::font::Font::new(&texture_creator, &ttf_font, Color::from(TEXT_COLOR_DARK));

    let mut windows = Vec::new();

    let mut instr_window = debug::create_instruction_window(&font, &dark_font, 22, deassembled_instructions);
    instr_window.set_pos(20, 130);
    instr_window.set_active(true);
    windows.push(&mut instr_window);

    let mut register_window = debug::create_register_window(&font, &dark_font);
    register_window.set_pos(20, 20);
    register_window.set_active(true);
    windows.push(&mut register_window);

    let mut zeropage_window = debug::create_memory_window(&font, &dark_font, 0, 256, 16);
    zeropage_window.set_pos(330, 20);
    zeropage_window.set_active(true);
    windows.push(&mut zeropage_window);

    let mut stack_window = debug::create_memory_window(&font, &dark_font, 0x100, 256, 16);
    stack_window.set_pos(330, 210);
    stack_window.set_active(true);
    windows.push(&mut stack_window);

    let mut ram_window = debug::create_memory_window(&font, &dark_font, 0x200, 0x600, 48);
    ram_window.set_pos(780, 20);
    ram_window.set_active(true);
    windows.push(&mut ram_window);

    let mut event_pump = sdl_context.event_pump()?;
    let mut timer = sdl_context.timer()?;

    'mainloop: loop {
        let time = timer.ticks();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    nes.tick();

                    //render(&mut canvas, &mut windows, nes)?;
                }
                _ => {}
            }
        }

        render(&mut canvas, &mut windows, nes)?;

        let delta = timer.ticks() - time;
        println!("{} ", delta);
    }

    Ok(())
}

