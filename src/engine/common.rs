use super::*;
use std::cell::LazyCell;
use std::fs;
use std::io;
use std::path::PathBuf;

pub const GLOBAL_DIR: &str = env!("CARGO_PKG_NAME");

pub struct Dirs {
    pub root: PathBuf,
    pub scripts: PathBuf,
    pub examples: PathBuf,
    pub assets: PathBuf,
}
impl Dirs {
    pub fn create(&self) -> io::Result<()> {
        let mut dirs = dirs();
        let dirs = [&dirs.root, &dirs.scripts, &dirs.examples, &dirs.assets];

        for dir in dirs {
            if !dir.is_dir() {
                if let Err(e) = fs::create_dir_all(dir) {
                    if let io::ErrorKind::AlreadyExists = e.kind() {
                        continue;
                    }
                    return Err(e.into());
                }
            }
        }
        Ok(())
    }
}

pub static mut DIRS: LazyCell<Dirs> = LazyCell::new(|| {
    let root = PathBuf::from(GLOBAL_DIR);
    Dirs {
        scripts: root.join("scripts"),
        assets: root.join("assets"),
        examples: root.join("examples"),
        root,
    }
});

pub fn dirs() -> &'static Dirs {
    unsafe { &DIRS }
}
