use macroquad::prelude::*;
use std::cell::OnceCell;
use std::collections::HashMap;

#[derive(Default)]
pub struct AssetStore {
    pub builtin_textures: HashMap<Box<str>, Texture2D>,
    pub user_textures: HashMap<Box<str>, Texture2D>,
}

impl AssetStore {
    fn init() -> Self {
        let mut new = AssetStore::default();
        new.load_textures(TEXTURES.into_iter());
        new
    }
    pub fn load_textures<'a, T>(&mut self, textures: T)
    where
        T: Iterator<Item = &'a (&'a str, &'static [u8], ImageFormat)>,
    {
        for (name, bytes, img_format) in textures {
            let name = name.to_string().into_boxed_str();
            let texture = Texture2D::from_file_with_format(bytes, Some(*img_format));
            self.builtin_textures.insert(name, texture);
        }
    }
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.user_textures
            .get(name)
            .or(self.builtin_textures.get(name))
    }
}

pub static TEXTURES: &[(&'static str, &[u8], ImageFormat)] = &[
    // ("grass", &[], ImageFormat::Png)
];
static mut ASSET_STORE: OnceCell<AssetStore> = OnceCell::new();

pub fn asset_store() -> &'static AssetStore {
    unsafe { ASSET_STORE.get_or_init(AssetStore::init) }
}

pub fn asset_store_mut() -> &'static mut AssetStore {
    unsafe {
        // Init if not initialized
        let _ = asset_store();
        ASSET_STORE.get_mut().unwrap_unchecked()
    }
}
