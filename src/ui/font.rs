use std::collections::HashMap;

use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::video::WindowContext;
use std::str::Chars;
use sdl2::pixels::Color;


const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!()$:><#, ";

pub struct Font<'a> {
    character_map: Vec<Texture<'a>>,
    width: u32,
    height: u32,
}

impl<'a> Font<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>,
               font: &sdl2::ttf::Font,
               color: Color) -> Font<'a> {
        let mut character_map = Vec::new();

        for _i in 0..255 {
            let texture = build_character_texture(texture_creator, font, '-', color);
            character_map.push(texture);
        }

        for renderable_char in CHARACTERS.chars() {
            character_map[renderable_char as usize] = build_character_texture(texture_creator, font, renderable_char, color);
        }

        let TextureQuery { width, height, .. } = character_map[0].query();

        Font { character_map, width, height }
    }

    pub fn get_character_texture(&self, character: char) -> &Texture {
        &self.character_map[character as usize]
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

fn build_character_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>,
                               font: &sdl2::ttf::Font,
                               character: char,
                                color: Color) -> Texture<'a> {
    let surface = font.render_char(character).
        blended(color).map_err(|e| e.to_string()).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string()).unwrap();

    texture
}

