use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::video::{Window, WindowContext};
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

macro_rules! rect (
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);


pub fn render_text(canvas: &mut Canvas<Window>,
               texture_creator: &TextureCreator<WindowContext>,
               font: &Font,
               x: i32,
               y: i32,
               text: &str,
               color: Color) -> Result<(), String> {
    let surface = font.render(text).
        blended(color).map_err(|e| e.to_string())?;
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

    let target = rect!(x,y, width * 2, height * 2);

    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}


pub fn render_window(canvas: &mut Canvas<Window>,
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

