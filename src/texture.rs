use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct AssetStore<'a> {
    pub textures: HashMap<&'a str, Texture2D>,
}

impl<'a> AssetStore<'a> {
    pub fn load_textures<T>(&mut self, textures: T)
    where
        T: Iterator<Item = &'a (&'a str, &'static [u8], ImageFormat)>,
    {
        for (name, bytes, img_format) in textures {
            let texture = Texture2D::from_file_with_format(bytes, Some(*img_format));
            self.textures.insert(name, texture);
        }
    }
    pub fn get_texture(&self, name: &'a str) -> Option<&Texture2D> {
        self.textures.get(name)
    }
}
