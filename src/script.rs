use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::SystemTime;

use rhai::Shared;

use crate::ui::Logger;

pub const GLOBAL_DIR: &str = env!("CARGO_PKG_NAME");

pub struct Script {
    path: PathBuf,
    modified: SystemTime,
    src: String,
}

pub struct Engine {
    pub engine: rhai::Engine,
    pub global_dir: PathBuf,
    pub script_dir: PathBuf,
    pub scripts: Vec<Script>,
}

pub enum Message {
    Error(anyhow::Error),
    ReloadScripts,
}

pub fn watch(dir: PathBuf, tx: Sender<Option<Message>>) {
    let path = dir.as_path();
    let mut last_modified = SystemTime::UNIX_EPOCH;

    loop {
        if tx.send(None).is_err() {
            // Stop loop if sender hung up (main exited)
            break;
        }
        match fs::metadata(path) {
            Err(e) => {
                let _ = tx.send(Some(Message::Error(e.into())));
                continue;
            }
            Ok(m) => match m.modified() {
                Ok(modified) => {
                    if last_modified != modified {
                        last_modified = modified;
                        println!("A");
                        let _ = tx.send(Some(Message::ReloadScripts));
                    }
                }
                Err(e) => {
                    let _ = tx.send(Some(Message::Error(e.into())));
                }
            },
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        let global_dir = PathBuf::from(GLOBAL_DIR);
        Self {
            engine: rhai::Engine::new(),
            script_dir: global_dir.join("scripts"),
            global_dir,
            scripts: vec![],
        }
    }
    pub fn watch(&self, tx: Sender<Option<Message>>) {
        loop {
            if tx.send(None).is_err() {
                // Stop loop if sender hung up (main exited)
                break;
            }
            match fs::metadata(&self.script_dir) {
                Err(e) => {
                    let _ = tx.send(Some(Message::Error(e.into())));
                    continue;
                }
                Ok(d) => {}
            }
        }
    }
    pub fn ensure_dirs_exist(&self) -> anyhow::Result<()> {
        if !self.global_dir.is_dir() || !self.script_dir.exists() {
            fs::create_dir_all(&self.script_dir)?;
        }
        Ok(())
    }
    pub fn load_scripts(&mut self, logger: &mut Logger) -> anyhow::Result<()> {
        let scripts = fs::read_dir(&self.script_dir)?;
        let ext = OsStr::new("rhai");

        for file in scripts {
            if let Err(e) = file {
                logger.log(format!("Skipping file: {e}"));
                continue;
            }

            let file = file.unwrap();
            let path = file.path();

            match path.extension() {
                None => {
                    logger.log(format!("Skipping file with no extension: {path:?}"));
                    continue;
                }
                Some(e) if e != ext => {
                    logger.log(format!(
                        "Skipping file with unkown(not .rhai) extension: {path:?}"
                    ));
                    continue;
                }
                _ => {} // Correct extension
            }

            // Check for update
            let file_name = path.file_name();
            let metadata = file.metadata()?;
            let modified = metadata.modified()?;
            if let Some(existing) = self.scripts.iter_mut().find(|s| s.path == path) {
                if existing.modified != modified {
                    existing.modified = modified;
                    existing.src = fs::read_to_string(&path)?;
                }
                continue;
            }

            // Add new script
            logger.log(format!("Adding new script {path:?}"));

            let script = Script {
                src: fs::read_to_string(&path)?,
                modified,
                path,
            };

            self.scripts.push(script);
        }

        Ok(())
    }
}
