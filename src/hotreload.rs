use libloading::{Library, Symbol};
use logic::wrap::ctx::Context;
use logic::Menu;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::path::Path;

type UpdateFn = unsafe extern "C" fn(m: &mut Menu);
type DrawFn = unsafe extern "C" fn(m: &mut Menu);
type SetCtxfn = unsafe extern "C" fn(*const Box<dyn Context>);

pub struct HotReloader<'a> {
    update: Symbol<'a, UpdateFn>,
    draw: Symbol<'a, DrawFn>,
    set_ctx: Symbol<'a, SetCtxfn>,
}

pub fn load_library() -> Library {
    unsafe {
        Library::new(HotReloader::LIB_PATH).unwrap()
    }
}

const LIB_EXT: &'static str = if cfg!(target_os = "windows") {
    ".dll"
} else if cfg!(target_os = "macos") {
    ".dylib"
} else {
    ".so"
};

impl<'a> HotReloader<'a> {
    #[cfg(target_os = "windows")]
    const LIB_PATH: &'static str = if cfg!(target_os = "windows") {
        "target/debug/logic.dll"
    } else if cfg!(target_os = "macos") {
        "target/debug/liblogic.dylib"
    } else {
        "target/debug/liblogic.so"
    };

    pub fn new(lib: &'a Library) -> Self {
        let s = unsafe {
            Self {
                update: lib.get(b"menu_update").unwrap(),
                draw: lib.get(b"menu_draw").unwrap(),
                set_ctx: lib.get(b"set_ctx").unwrap(),
            }
        };

        let mut watcher = recommended_watcher(|res| {
            match res {
                Ok(_) => {
                    println!("File change detected, reloading library...");
                    // Unload the previous library and load the new one
                    let lib = unsafe { Library::new(Self::LIB_PATH).unwrap() };
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }).unwrap();

        watcher.watch(Path::new(Self::LIB_PATH), RecursiveMode::NonRecursive).unwrap();

        s
    }
    #[inline]
    pub fn update(&self, menu: &mut Menu) {
        unsafe {(self.update)(menu) }
    }
    #[inline]
    pub fn draw(&self, menu: &mut Menu) {
        unsafe {
            (self.draw)(menu);
        }
    }
    #[inline]
    pub fn set_ctx(&self, dr: *const Box<dyn Context>) {
        unsafe {
            (self.set_ctx)(dr);
        }
    }
}