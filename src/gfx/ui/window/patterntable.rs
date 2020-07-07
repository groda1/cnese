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
    left_texture: Texture<'a>,
    right_texture : Texture<'a>

}

impl<'a> PatternTableWindow<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>,
               width: u32,
               height: u32,
               pattern_table_index: u8) -> PatternTableWindow {

        let left_texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, TEXTURE_WIDTH, TEXTURE_HEIGHT).unwrap();
        let right_texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, TEXTURE_WIDTH, TEXTURE_HEIGHT).unwrap();

        PatternTableWindow { left_texture,right_texture, width, height, pattern_table_index }
    }

    fn _update_texture(&mut self, nes: &NES, pattern_table_index: u8) -> Result<(), String>{
        let pixel_data = nes.borrow_ppu().patterntable_to_texture_data(pattern_table_index);
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

        if pattern_table_index == 0 {
            self.left_texture.update(None, &texture_rgb_data, (TEXTURE_WIDTH * 3) as usize)
                .map_err(|e| e.to_string())?;
        } else {
            self.right_texture.update(None, &texture_rgb_data, (TEXTURE_WIDTH * 3) as usize).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}


impl<'a> RenderableWindow for PatternTableWindow<'a> {

    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {

        self._update_texture(nes, 0)?;
        self._update_texture(nes, 1)?;

        render::textured_window(canvas, x, y, self.width, self.height, &self.left_texture)?;
        render::textured_window(canvas, x + self.width as i32 + 5, y, self.width, self.height, &self.right_texture)?;

        Ok(())
    }
}
