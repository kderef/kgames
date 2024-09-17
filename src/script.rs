use include_dir::{include_dir, Dir};
use macroquad::prelude::*;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::{ffi::OsStr, io};

use crate::{ffi::*, reg_type, texture::asset_store};
use rhai::{EvalAltResult, ImmutableString, Scope, AST};

use crate::menu::Console;

pub const GLOBAL_DIR: &str = env!("CARGO_PKG_NAME");

#[derive(Clone)]
pub struct Script<'a> {
    path: PathBuf,
    modified: SystemTime,
    pub ast: AST,
    //TODO:
    pub scope: Scope<'a>,
    pub is_example: bool,
}
impl<'a> Default for Script<'a> {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            modified: SystemTime::now(),
            ast: AST::empty(),
            scope: Scope::new(),
            is_example: false,
        }
    }
}

impl<'a> Script<'a> {
    pub fn name(&self) -> &str {
        self.path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("INVALID_SCRIPT_NAME")
    }
    pub fn populate_scope(&mut self) {
        for (name, color) in COLORS {
            self.scope.push_constant(name, color);
        }
        for (name, key) in KEYS {
            self.scope.push_constant(name, key);
        }
        for (name, key) in MOUSE_BUTTONS {
            self.scope.push_constant(name, key);
        }
        self.ast
            .iter_literal_variables(true, true)
            .for_each(|(name, is_const, val)| {
                if is_const {
                    self.scope.push_constant(name, val);
                } else {
                    self.scope.push_dynamic(name, val);
                }
            });
    }
}

pub struct Engine<'a> {
    pub engine: rhai::Engine,
    pub global_dir: PathBuf,
    pub script_dir: PathBuf,
    pub example_dir: PathBuf,
    pub asset_dir: PathBuf,
    pub scripts: Vec<Script<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptDir {
    Scripts,
    Examples,
}

impl<'a> Engine<'a> {
    fn register_types(&mut self) {
        // Fields
        reg_type! {
            self.engine => {
                Vec2 as "Vec2" = x, y;
                Vec3 as "Vec3" = x, y, z;
                Rect as "Rect" = x, y, w, h;
                Color as "Color" = r, g, b, a;
                KeyCode as "Key";
                Texture2D as "Texture";
                MouseButton as "Mouse";
            }
        }

        // Methods
        reg_type! {
            self.engine => {
                Texture2D = width(), height();
                Rect = size(), center();
            }
        }
    }

    fn register_fns(&mut self) {
        self.engine
            // Actions
            .register_fn("clear", clear_background)
            .register_fn(
                "text",
                |t: ImmutableString, x: f32, y: f32, sz: f32, tint: Color| {
                    draw_text(t.as_str(), x, y, sz, tint);
                },
            )
            .register_fn("circle", draw_circle)
            .register_fn("line", draw_line)
            .register_fn("triangle", draw_triangle)
            .register_fn("rectangle", draw_rectangle)
            .register_fn("rectangle_lines", draw_rectangle_lines)
            .register_fn("msgbox", |title: ImmutableString, msg: ImmutableString| {
                let _ = msgbox::create(title.as_str(), msg.as_str(), msgbox::IconType::Info);
            })
            .register_fn("overlaps", Rect::overlaps)
            .register_fn("overlaps", |a: Rect, b: Rect| a.overlaps(&b))
            .register_fn("texture", draw_texture)
            .register_fn("texture", draw_texture_stored)
            .register_fn(
                "texture_ex",
                |t: &Texture2D, x: f32, y: f32, tint: Color, dest_size: Vec2, rotation: f32| {
                    draw_texture_ex(
                        t,
                        x,
                        y,
                        tint,
                        DrawTextureParams {
                            dest_size: Some(dest_size),
                            source: None,
                            rotation,
                            flip_x: false,
                            flip_y: false,
                            pivot: None,
                        },
                    )
                },
            )
            .register_fn(
                "texture_pro",
                |t: &Texture2D,
                 x: f32,
                 y: f32,
                 tint: Color,
                 dest_size: Vec2,
                 source: Rect,
                 rotation: f32,
                 pivot: Vec2| {
                    draw_texture_ex(
                        t,
                        x,
                        y,
                        tint,
                        DrawTextureParams {
                            dest_size: Some(dest_size),
                            source: Some(source),
                            rotation,
                            flip_x: false,
                            flip_y: false,
                            pivot: Some(pivot),
                        },
                    )
                },
            )
            // textures
            .register_fn("load_texture", load_texture_sync)
            .register_fn("get_texture", load_texture_stored)
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
            .register_fn("vec2", vec2)
            .register_fn("vec3", vec3)
            .register_fn("rect", Rect::new)
            .register_fn("color", Color::new);
    }

    pub fn new() -> Self {
        let home = PathBuf::from(GLOBAL_DIR);
        let engine = rhai::Engine::new();

        let mut s = Self {
            engine,
            script_dir: home.join("scripts"),
            asset_dir: home.join("assets"),
            example_dir: home.join("examples"),
            global_dir: home,
            scripts: vec![],
        };

        s.register_types();
        s.register_fns();
        s
    }

    /// Check if all the dirs exist, if not creating them
    pub fn ensure_dirs_exist(&self) -> anyhow::Result<()> {
        let dirs = [
            &self.global_dir,
            &self.script_dir,
            &self.example_dir,
            &self.script_dir,
        ];

        for dir in dirs {
            if !dir.is_dir() {
                if let Err(e) = fs::create_dir_all(dir) {
                    match e.kind() {
                        // Make sure
                        io::ErrorKind::AlreadyExists => {
                            continue;
                        }
                        _ => return Err(e.into()),
                    }
                }
            }
        }
        Ok(())
    }

    /// Tries to create the readme file, returning the full path on success
    pub fn create_readme(&self, filename: &str) -> anyhow::Result<PathBuf> {
        static README: &str = include_str!("../README.md");
        let path = self.global_dir.join(filename);
        fs::write(&path, README)?;
        Ok(path)
    }

    pub fn write_examples(&self, warnings: &mut Vec<String>) -> Result<(), Vec<std::io::Error>> {
        static EXAMPLES: Dir = include_dir!("$CARGO_MANIFEST_DIR/res/examples");

        let mut errors = Vec::with_capacity(EXAMPLES.files().count());

        for example in EXAMPLES.files() {
            let write_path = self.example_dir.join(example.path());

            if write_path.is_file() {
                warnings.push(format!("File {write_path:?} already exists. To overwrite the example, rename or delete it."));
                continue;
            }

            if let Err(e) = fs::write(self.example_dir.join(example.path()), example.contents()) {
                errors.push(e);
            }
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }

    /// Internal
    fn load_scripts_from_dir(
        &mut self,
        console: &mut Console,
        errors: &mut Vec<(PathBuf, anyhow::Error)>,
        scripts: impl Iterator<Item = io::Result<DirEntry>>,
        is_example_dir: bool,
    ) -> anyhow::Result<()> {
        let mut result = Ok(());
        let ext = OsStr::new("rhai");

        for file in scripts {
            if let Err(e) = file {
                console.log(format!("Skipping file: {e}"));
                continue;
            }

            let file = file.unwrap();
            let path = file.path();

            match path.extension() {
                None => {
                    console.log(format!("Skipping file with no extension: {path:?}"));
                    continue;
                }
                Some(e) if e != ext => {
                    console.log(format!(
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
                    existing.populate_scope();
                }
                continue;
            }

            // Add new script
            console.log(format!("Adding new script {path:?}"));

            let contents = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    add_err(e.into());
                    continue;
                }
            };

            // *** Compile script! *** //
            let mut script = Script::default();
            script.is_example = is_example_dir;

            // Disable optimizations
            self.engine
                .set_optimization_level(rhai::OptimizationLevel::None);
            let ast = match self.engine.compile(contents) {
                Ok(a) => a,
                Err(e) => {
                    add_err(e.into());
                    continue;
                }
            };
            script.populate_scope();

            // Enable optimizations
            self.engine
                .set_optimization_level(rhai::OptimizationLevel::Simple);
            let ast =
                self.engine
                    .optimize_ast(&script.scope, ast, self.engine.optimization_level());

            script.ast = ast;
            script.modified = modified;
            script.path = path;

            // Run code once
            if let Err(e) = self
                .engine
                .run_ast_with_scope(&mut script.scope, &script.ast)
            {
                let e = anyhow::anyhow!("Failed to init script: {e}");
                result = Err(anyhow::anyhow!("{e}"));
                errors.push((script.path.clone(), e.into()));
                continue;
            }

            self.scripts.push(script);
        }

        result
    }

    /// Load scripts from `from`, in order.
    pub fn load_scripts(
        &mut self,
        console: &mut Console,
        errors: &mut Vec<(PathBuf, anyhow::Error)>,
        from: &[ScriptDir],
    ) -> anyhow::Result<()> {
        let mut result = Ok(());
        let mut is_example_dir = false;

        for source in from {
            let src = match source {
                ScriptDir::Scripts => &self.script_dir,
                ScriptDir::Examples => {
                    is_example_dir = true;
                    &self.example_dir
                }
            };

            console.log(format!("==> Loading scripts from {src:?}"));

            match fs::read_dir(src) {
                Ok(scripts) => {
                    if let Err(e) =
                        self.load_scripts_from_dir(console, errors, scripts, is_example_dir)
                    {
                        result = Err(e);
                    }
                }
                Err(e) => {
                    console.err(format!("Failed to read dir {src:?}: {e}"));
                    result = Err(e.into());
                }
            }
        }
        result
    }
}
