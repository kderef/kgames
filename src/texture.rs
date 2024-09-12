// # sources
// - https://artage.io/en/icon-packs/original-windows-95-icons
// - https://win98icons.alexmeub.com

use macroquad::prelude::*;
use std::cell::OnceCell;
use std::collections::HashMap;

macro_rules! textures {
    (
        $($name:literal : $kind:ident = $path:literal),*
        // TODO: rest
    ) => {
        {
            use macroquad::prelude::*;

            let mut t = HashMap::new();
            $(
                t.insert($name, Texture2D::from_file_with_format(
                    include_bytes!(concat!("../res/", $path)),
                    Some(macroquad::prelude::ImageFormat::$kind)
                ));
            )*
            t
        }
    };
}

#[derive(Default)]
pub struct AssetStore {
    pub builtin_textures: HashMap<&'static str, Texture2D>,
    pub user_textures: HashMap<Box<str>, Texture2D>,
}

impl AssetStore {
    fn init() -> Self {
        let mut s = Self {
            user_textures: HashMap::new(),
            builtin_textures: textures! {
                "folder_open": Png = "sys/folder_open.png",
                "folder_open_file": Png = "sys/folder_open_file.png",

                "yes": Png = "sys/yes.png",
                "no": Png = "sys/no.png",
                "warning": Png = "sys/warning.png",
                "search_file": Png = "sys/search_file.png",
                "help_book": Png = "sys/help_book.png",

                "brick": Png = "PixelTexPack/Bricks/CLAYBRICKS.png"
            },
        };

        // Set pixelated textures to nearest
        for name in [
            "folder_open",
            "yes",
            "no",
            "warning",
            "search_file",
            "help_book",
        ] {
            s.builtin_textures
                .get_mut(name)
                .unwrap()
                .set_filter(FilterMode::Nearest);
        }
        s
    }
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.user_textures
            .get(name)
            .or(self.builtin_textures.get(name))
    }
}

// Singleton
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
