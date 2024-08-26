use macroquad::{color, prelude::*};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::SystemTime;

use crate::ffi::*;
use rhai::{ImmutableString, Scope, AST};

use crate::ui::Logger;

pub const GLOBAL_DIR: &str = env!("CARGO_PKG_NAME");

#[derive(Clone)]
pub struct Script {
    path: PathBuf,
    modified: SystemTime,
    pub ast: AST,
}
impl Script {
    pub fn name(&self) -> &str {
        self.path.file_name().and_then(|s| s.to_str()).unwrap_or("")
    }
}

pub struct Engine<'a> {
    pub engine: rhai::Engine,
    pub global_dir: PathBuf,
    pub script_dir: PathBuf,
    pub scripts: Vec<Script>,
    pub scope: Scope<'a>,
}

impl<'a> Engine<'a> {
    fn populate_scope(scope: &mut Scope) {
        for (name, color) in COLORS {
            scope.push_constant(name, color);
        }
        for (name, key) in KEYS {
            scope.push_constant(name, key);
        }
    }

    fn register_types(engine: &mut rhai::Engine) {
        engine.register_type_with_name::<Color>("Color");
        engine.register_type_with_name::<Vec2>("Vec2");
        engine.register_type_with_name::<Rect>("Rect");
        engine.register_type_with_name::<KeyCode>("Key");
        engine.register_type_with_name::<MouseButton>("Mouse");
    }

    fn register_fns(engine: &mut rhai::Engine) {
        engine
            // Actions
            .register_fn("clear", clear_background)
            .register_fn("text", draw_text)
            .register_fn("circle", draw_circle)
            .register_fn("line", draw_line)
            // Information
            .register_fn("deltatime", get_frame_time)
            .register_fn("screen_width", screen_width)
            .register_fn("screen_height", screen_height)
            .register_fn("last_keypress", get_last_key_pressed)
            .register_fn("key_down", is_key_down)
            .register_fn("key_pressed", is_key_pressed)
            .register_fn("key_released", is_key_released)
            .register_fn("mouse_down", is_mouse_button_down)
            .register_fn("mouse_pressed", is_mouse_button_pressed)
            .register_fn("mouse_released", is_mouse_button_released)
            .register_fn("fps", get_fps)
            // Getters/Setters
            .register_fn("rectangle", draw_rectangle)
            .register_fn("color", Color::new);
    }

    pub fn new() -> Self {
        let global_dir = PathBuf::from(GLOBAL_DIR);
        let mut engine = rhai::Engine::new();
        let mut scope = Scope::new();

        Self::register_types(&mut engine);
        Self::populate_scope(&mut scope);
        Self::register_fns(&mut engine);

        Self {
            engine,
            script_dir: global_dir.join("scripts"),
            global_dir,
            scripts: vec![],
            scope,
        }
    }

    pub fn ensure_dirs_exist(&self) -> anyhow::Result<()> {
        if !self.global_dir.is_dir() || !self.script_dir.exists() {
            fs::create_dir_all(&self.script_dir)?;
        }
        Ok(())
    }

    pub fn load_scripts(
        &mut self,
        logger: &mut Logger,
        errors: &mut Vec<(PathBuf, anyhow::Error)>,
    ) -> anyhow::Result<()> {
        let mut result = Ok(());
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
                    existing.ast = self.engine.compile(fs::read_to_string(&path)?)?;
                }
                continue;
            }

            let mut add_err = |e| {
                errors.push((path.clone(), e));
                result = Err(anyhow::anyhow!("Failed to load scripts"));
            };

            // Add new script
            logger.log(format!("Adding new script {path:?}"));
            let contents = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    add_err(e.into());
                    continue;
                }
            };
            let ast = match self.engine.compile(contents) {
                Ok(a) => a,
                Err(e) => {
                    add_err(e.into());
                    continue;
                }
            };

            let script = Script {
                ast,
                modified,
                path,
            };
            // Run code once
            if let Err(e) = self.engine.run_ast_with_scope(&mut self.scope, &script.ast) {
                let e = anyhow::anyhow!("Failed to init script: {e}");
                errors.push((script.path.clone(), e.into()));
                continue;
            }

            self.scripts.push(script);
        }

        result
    }
}
