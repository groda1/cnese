use sdl2::render::{Texture, TextureCreator, TextureQuery, Canvas};
use sdl2::video::{WindowContext, Window};
use sdl2::pixels::{Color, PixelFormatEnum};

use crate::nes::nes::NES;
use crate::gfx::render;
use super::window::RenderableWindow;

const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;

pub struct PatternTableWindow<'a> {
    texture: Texture<'a>
}

impl<'a> PatternTableWindow<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> PatternTableWindow {
        let texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT).unwrap();

        PatternTableWindow { texture }
    }
}


impl<'a> RenderableWindow for PatternTableWindow<'a> {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {

        let derp = [230 as u8; 300 * 300 * 3];
        self.texture.update(None, &derp, 900);

        render::textured_window(canvas, x, y, WIDTH, HEIGHT, 1, &self.texture );

        Ok(())
    }
}
