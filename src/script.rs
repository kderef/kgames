use macroquad::prelude::*;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::{ffi::*, texture::AssetStore};
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
        self.path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("INVALID_SCRIPT_NAME")
    }
}

pub struct Engine<'a> {
    pub engine: rhai::Engine,
    pub global_dir: PathBuf,
    pub script_dir: PathBuf,
    pub asset_dir: PathBuf,
    pub scripts: Vec<Script>,
    pub scope: Scope<'a>,
}

impl<'a> Engine<'a> {
    fn populate_scope(&mut self) {
        for (name, color) in COLORS {
            self.scope.push_constant(name, color);
        }
        for (name, key) in KEYS {
            self.scope.push_constant(name, key);
        }
    }

    fn register_types(&mut self) {
        self.engine.register_type_with_name::<Color>("Color");
        self.engine.register_type_with_name::<Vec2>("Vec2");
        self.engine.register_type_with_name::<Rect>("Rect");
        self.engine.register_type_with_name::<KeyCode>("Key");
        self.engine.register_type_with_name::<MouseButton>("Mouse");
        self.engine.register_type_with_name::<Texture2D>("Texture");
    }

    fn register_fns(&mut self) {
        self.engine
            // Actions
            .register_fn("clear", clear_background)
            .register_fn("text", draw_text)
            .register_fn("circle", draw_circle)
            .register_fn("line", draw_line)
            .register_fn("msgbox", |title: ImmutableString, msg: ImmutableString| {
                let _ = msgbox::create(title.as_str(), msg.as_str(), msgbox::IconType::Info);
            })
            // textures
            .register_fn("load_texture", load_texture_sync)
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
        let global_dir_ = PathBuf::from(GLOBAL_DIR);
        let engine = rhai::Engine::new();
        let scope = Scope::new();

        let mut s = Self {
            engine,
            script_dir: global_dir_.join("scripts"),
            asset_dir: global_dir_.join("assets"),
            global_dir: global_dir_,
            scripts: vec![],
            scope,
        };

        s.register_types();
        s.populate_scope();
        s.register_fns();
        s
    }

    pub fn ensure_dirs_exist(&self) -> anyhow::Result<()> {
        if !self.global_dir.is_dir() || !self.script_dir.is_dir() {
            fs::create_dir_all(&self.script_dir)?;
        }
        if !self.asset_dir.is_dir() {
            fs::create_dir(&self.asset_dir)?;
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
                        "Skipping file with unknown(not .rhai) extension: {path:?}"
                    ));
                    continue;
                }
                _ => {} // Correct extension
            }

            // Error handling
            let mut add_err = |e| {
                errors.push((path.clone(), e));
                if result.is_ok() {
                    result = Err(anyhow::anyhow!("Failed to load scripts"));
                }
            };

            // Check for update
            let metadata = file.metadata()?;
            let modified = metadata.modified()?;

            if let Some(existing) = self.scripts.iter_mut().find(|s| s.path == path) {
                if existing.modified != modified {
                    existing.modified = modified;
                    existing.ast = self.engine.compile(fs::read_to_string(&path)?)?;

                    let contents = match fs::read_to_string(&path) {
                        Ok(c) => c,
                        Err(e) => {
                            add_err(e.into());
                            continue;
                        }
                    };
                    existing.ast = match self.engine.compile(contents) {
                        Ok(a) => a,
                        Err(e) => {
                            add_err(e.into());
                            continue;
                        }
                    };
                }
                continue;
            }

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
                result = Err(anyhow::anyhow!("{e}"));
                errors.push((script.path.clone(), e.into()));
                continue;
            }

            self.scripts.push(script);
        }

        result
    }
}
