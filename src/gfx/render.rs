use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::ui::font::Font;

macro_rules! rect (
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn render_text(canvas: &mut Canvas<Window>,
                   font: &Font,
                   x: i32,
                   y: i32,
                   text: &str) -> Result<(), String> {

    _render_text(canvas, font, x, y, text, 2)?;

    Ok(())
}

pub fn render_text_small(canvas: &mut Canvas<Window>,
                         font: &Font,
                         x: i32,
                         y: i32,
                         text: &str) -> Result<(), String> {
    _render_text(canvas, font, x, y, text, 1)?;

    Ok(())
}

fn _render_text(canvas: &mut Canvas<Window>,
                font: &Font,
                x: i32,
                y: i32,
                text: &str,
                scale: u32) -> Result<(), String> {
    let mut x_off = 0;
    for character in text.chars() {
        _render_textured_rect(canvas,
                              font.get_character_texture(character),
                              x + x_off, y,
                              font.get_width() * scale,
                              font.get_height() * scale)?;

        x_off += (font.get_width() * scale) as i32;
    }

    Ok(())
}

fn _render_textured_rect(canvas: &mut Canvas<Window>,
                         texture: &Texture,
                         x: i32,
                         y: i32,
                         w: u32,
                         h: u32) -> Result<(), String> {
    let target = rect!(x, y, w, h);
    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}


pub fn window(canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              w: u32,
              h: u32,
              border_color: Color,
              bg_color: Color) -> Result<(), String> {
    let rect = rect!(x,y,w,h);

    canvas.set_draw_color(bg_color);
    canvas.fill_rect(rect)?;

    canvas.set_draw_color(border_color);
    canvas.draw_rect(rect)?;

    Ok(())
}

pub fn textured_window(canvas: &mut Canvas<Window>,
                       x: i32,
                       y: i32,
                       w: u32,
                       h: u32,
                       _border_width: u32,
                       texture: &Texture) -> Result<(), String> {
    _render_textured_rect(canvas, texture, x, y, w, h)?;

    Ok(())
}

