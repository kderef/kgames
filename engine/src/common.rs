use console::Console;

use super::*;
use std::cell::LazyCell;
use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub const GLOBAL_DIR: &str = env!("CARGO_PKG_NAME");

// Types
#[cfg(feature = "rhai-engine")]
pub mod scripting {
    pub use rhai::{EvalAltResult, ImmutableString};
    pub type Error = Box<EvalAltResult>;
    pub type Result<T> = std::result::Result<T, Error>;
}

#[cfg(feature = "lua-engine")]
mod scripting {
    pub type Error = mlua::Error;
    pub type Result<T> = std::result::Result<T, Error>;
}

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

pub fn create_readme(filename: impl AsRef<Path>) -> io::Result<PathBuf> {
    static README: &str = include_str!("../../README.md");
    let path = dirs().root.join(filename);
    fs::write(&path, README)?;
    Ok(path)
}

pub fn read_files_from_dir<P, E>(
    from: &[P],
    ext: E,
    console: &mut Console,
    errors: &mut ErrorMap,
) -> anyhow::Result<Vec<(DirEntry, Vec<u8>, bool)>>
where
    P: AsRef<Path>,
    E: AsRef<OsStr>,
{
    let mut contents = vec![];
    let mut err = None;

    for src in from {
        let example = src.as_ref() == ScriptDir::Examples.path();

        console.log(format!("==> Loading scripts from {:?}", src.as_ref()));

        match fs::read_dir(src) {
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            let extension = path.extension();

                            // If extension matches, read the file.
                            if extension.is_some_and(|e| e == ext.as_ref()) {
                                match fs::read(&path) {
                                    Err(e) => {
                                        console.err(format!("Failed to read {path:?}: {e}"));
                                        errors.push((path, e.into()));
                                    }
                                    Ok(c) => {
                                        contents.push((entry, c, example));
                                    }
                                }
                            } else {
                                console
                                    .warn(format!("Skipping file with wrong extension: {path:?}"));
                            }
                        }
                        Err(e) => {
                            console.err(format!("Skipping file: {e}"));
                        }
                    }
                }
            }
            Err(e) => {
                errors.push((src.as_ref().to_path_buf(), e.into()));
            }
        }
    }

    if let Some(err) = err {
        Err(err)
    } else {
        Ok(contents)
    }
}
