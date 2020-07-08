use sdl2::render::{Texture, TextureCreator, Canvas};
use sdl2::video::{WindowContext, Window};
use sdl2::pixels::{PixelFormatEnum};

use crate::nes::nes::NES;
use crate::gfx::render;

use super::window::RenderableWindow;

use crate::ppu::ppu;
use crate::ppu::ppu::{FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT};
use crate::gfx::palette;

pub struct FramebufferWindow<'a> {
    width: u32,
    height: u32,
    texture: Texture<'a>,
}

impl<'a> FramebufferWindow<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>,
               width: u32,
               height: u32) -> FramebufferWindow {
        let texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24,
                                      ppu::FRAMEBUFFER_WIDTH as u32,
                                      ppu::FRAMEBUFFER_HEIGHT as u32).unwrap();

        FramebufferWindow { width, height, texture }
    }

    fn _update_texture(&mut self, nes: &NES) -> Result<(), String> {
        let pixel_data = nes.get_ppu().get_framebuffer();
        let mut texture_rgb_data = [0 as u8; (FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT * 3) as usize];
        for (i, val) in pixel_data.iter().enumerate() {
            let (r, g, b) = palette::NTSC_2C02[*val as usize];
            texture_rgb_data[i * 3] = r;
            texture_rgb_data[i * 3 + 1] = g;
            texture_rgb_data[i * 3 + 2] = b;
        }
        self.texture.update(None, &texture_rgb_data, (FRAMEBUFFER_WIDTH * 3) as usize).map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl<'a> RenderableWindow for FramebufferWindow<'a> {
    fn render(&mut self,
              canvas: &mut Canvas<Window>,
              x: i32,
              y: i32,
              nes: &NES) -> Result<(), String> {
        self._update_texture(nes);
        render::textured_window(canvas, x, y, self.width, self.height, &self.texture)?;

        Ok(())
    }
}


