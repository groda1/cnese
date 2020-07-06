use sdl2::render::{Texture, TextureCreator, Canvas};
use sdl2::video::{WindowContext, Window};
use sdl2::pixels::{PixelFormatEnum};

use crate::nes::nes::NES;
use crate::gfx::render;
use super::window::RenderableWindow;

const TEXTURE_WIDTH: u32 = 128;
const TEXTURE_HEIGHT: u32 = 128;

pub struct PatternTableWindow<'a> {
    width: u32,
    height: u32,
    pattern_table_index: u8,
    texture: Texture<'a>,
}

impl<'a> PatternTableWindow<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>,
               width: u32,
               height: u32,
               pattern_table_index: u8) -> PatternTableWindow {
        let texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, TEXTURE_WIDTH, TEXTURE_HEIGHT).unwrap();

        PatternTableWindow { texture, width, height, pattern_table_index }
    }
}


impl<'a> RenderableWindow for PatternTableWindow<'a> {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {

        let pixel_data = nes.borrow_ppu().patterntable_to_texture_data(self.pattern_table_index);
        let mut texture_rgb_data = [0 as u8; (128 * 128 * 3) as usize];

        for (i, val) in pixel_data.iter().enumerate() {
            match val {
                0 => {},
                1 => {
                    texture_rgb_data[i * 3] = 255;
                }
                2 => {
                    texture_rgb_data[i * 3 + 1] = 255;
                },
                3 => {
                    texture_rgb_data[i * 3 ] = 255;
                    texture_rgb_data[i * 3 + 1] = 255;
                    texture_rgb_data[i * 3 + 2] = 255;
                }
                _ => unreachable!()
            }
        }

        self.texture.update(None, &texture_rgb_data, (TEXTURE_WIDTH * 3) as usize)
            .map_err(|e| e.to_string())?;
        render::textured_window(canvas, x, y, self.width, self.height, 1, &self.texture)?;

        Ok(())
    }
}
