use super::*;
use include_dir::{include_dir, Dir};
use macroquad::prelude::*;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::{ffi::OsStr, io};

use crate::{ffi::*, reg_type, texture::asset_store};
use rhai::{EvalAltResult, FuncArgs, ImmutableString, Scope, AST};

use crate::menu::Console;

fn load_scripts_from_dir(
    eng: &mut Engine,
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

        if let Some(existing) = eng.scripts.iter_mut().find(|s| s.path == path) {
            if existing.modified != modified {
                existing.modified = modified;

                let contents = match fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        add_err(e.into());
                        continue;
                    }
                };

                existing.ast = match eng.engine.compile(contents) {
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
        eng.engine
            .set_optimization_level(rhai::OptimizationLevel::None);
        let ast = match eng.engine.compile(contents) {
            Ok(a) => a,
            Err(e) => {
                add_err(e.into());
                continue;
            }
        };
        script.populate_scope();

        // Enable optimizations
        eng.engine
            .set_optimization_level(rhai::OptimizationLevel::Simple);
        let ast = eng
            .engine
            .optimize_ast(&script.scope, ast, eng.engine.optimization_level());

        script.ast = ast;
        script.modified = modified;
        script.path = path;

        // Run code once
        if let Err(e) = eng
            .engine
            .run_ast_with_scope(&mut script.scope, &script.ast)
        {
            let e = anyhow::anyhow!("Failed to init script: {e}");
            result = Err(anyhow::anyhow!("{e}"));
            errors.push((script.path.clone(), e.into()));
            continue;
        }

        eng.scripts.push(script);
    }

    result
}

macro_rules! reg_type {
    (
        $engine: expr => {
            $(
                $name:ty as $exposed_name:literal $(=
                    $($field:ident),*)?;
            )*
        }
    ) => {
        $(
            $(
                $engine.register_type_with_name::<$name>($exposed_name);

                // Register get/set for each field or method
                $(
                    $engine.register_get_set(
                        stringify!($field),
                        |_self: &mut $name| _self.$field,
                        |_self: &mut $name, new| _self.$field = new
                    );
                )*

            )*
        )?
    };
    // Getters via methods
    (
        $engine: expr => {
            $(
                $name:ty = $($method:ident()),*;
            )*
        }
    ) => {
       $(
           $(
                $engine.register_get(
                    stringify!($field),
                    |_self: &mut $name| _self.$method(),
                );
           )*
        )*
    };
}

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
impl<'a> GameScript for Script<'a> {
    fn path<'p>(&'p self) -> &'p Path {
        &self.path
    }
    fn name<'n>(&'n self) -> Option<&'n str> {
        self.path.file_name().and_then(OsStr::to_str)
    }
    fn is_example(&self) -> bool {
        self.is_example
    }
    fn reset(&mut self) {
        self.scope.clear();
    }
    fn populate_scope(&mut self) {
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
    pub scripts: Vec<Script<'a>>,
}
impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Self {
            engine: rhai::Engine::new(),
            scripts: vec![],
        }
    }
}

impl<'a> ScriptEngine for Engine<'a> {
    type Script = Script<'a>;

    fn extension<'b>() -> &'b str {
        "rhai"
    }
    fn expose_layer(&mut self) {
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

    fn call_function(&mut self, script_index: usize, name: impl AsRef<str>) -> anyhow::Result<()> {
        let script = &mut self.scripts[script_index];
        self.engine
            .call_fn::<()>(
                &mut script.scope, // NOTE: possible OOB
                &script.ast,
                name,
                (), // IMPORTANT: no args are passed
            )
            .map_err(|e| anyhow::anyhow!("{e}"));
        Ok(())
    }

    fn scripts<'s>(&'s mut self) -> &'s mut [Self::Script] {
        &mut self.scripts
    }

    fn load_scripts(
        &mut self,
        console: &mut Console,
        errors: &mut ErrorMap,
        from: &[ScriptDir],
    ) -> anyhow::Result<()> {
        let mut result = Ok(());
        let mut is_example_dir = false;

        for source in from {
            let src = source.path();

            console.log(format!("==> Loading scripts from {src:?}"));

            match fs::read_dir(src) {
                Ok(scripts) => {
                    if let Err(e) =
                        load_scripts_from_dir(self, console, errors, scripts, is_example_dir)
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

    fn reload_scripts(
        &mut self,
        console: &mut Console,
        errors: &mut ErrorMap,
    ) -> anyhow::Result<()> {
        // NOTE: useless!
        self.load_scripts(console, errors, &[ScriptDir::Scripts, ScriptDir::Examples])
    }

    fn write_examples(&mut self, warnings: &mut Vec<String>) -> Result<(), Vec<io::Error>> {
        static EXAMPLES: Dir = include_dir!("$CARGO_MANIFEST_DIR/res/examples");

        let mut errors = Vec::with_capacity(EXAMPLES.files().count());

        for example in EXAMPLES.files() {
            let write_path = dirs().examples.join(example.path());

            if write_path.is_file() {
                warnings.push(format!("File {write_path:?} already exists. To overwrite the example, rename or delete it."));
                continue;
            }

            if let Err(e) = fs::write(write_path, example.contents()) {
                errors.push(e);
            }
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }
}
