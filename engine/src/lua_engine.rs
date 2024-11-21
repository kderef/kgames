use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

use mlua::Chunk;
use mlua::Compiler;
use mlua::Function;
use mlua::Lua;
use mlua::Scope;

use crate::ffi;
use crate::ffi::COLORS;
use macroquad::prelude::*;

use super::*;

macro_rules! readonly_table {
    ($lua:expr, $type_:ty {$($field:ident),+}) => {
        {
            use super::*;
            |__x: $type_| $lua.create_table_from([
                $(
                    (stringify!($field), __x.$field),
                )+
            ]).and_then(|t| {
                t.set_readonly(true);
                Ok(t)
            })
        }
    };
}

pub struct Engine {
    scripts: Vec<Script>,
    compiler: Compiler,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            scripts: vec![],
            compiler: Compiler::new(),
        }
    }
    fn compile_and_init_script(
        &self,
        path: PathBuf,
        src: impl AsRef<[u8]>,
        example: bool,
        modified: SystemTime,
    ) -> anyhow::Result<Script> {
        let mut vm = Lua::new();
        let bytecode = self.compiler.compile(src.as_ref());

        let mut script = Script {
            path,
            is_example: example,
            bytecode,
            modified,
            vm,
        };

        // Load the bytecode
        script.populate_scope();
        // Run Once! (load globals, etc)
        script.vm.load(&script.bytecode).exec()?;

        Ok(script)
    }
}

pub struct Script {
    path: PathBuf,
    bytecode: Vec<u8>,
    // TODO: store chunks in script for perf
    // chunk: Chunk<'a, 'a>,
    vm: Lua,
    is_example: bool,
    modified: SystemTime,
}

impl GameScript for Script {
    fn path<'a>(&'a self) -> &'a Path {
        &self.path
    }
    fn name<'a>(&'a self) -> Option<&'a str> {
        match self.path.file_name() {
            Some(path) => path.to_str(),
            _ => None,
        }
    }
    fn is_example(&self) -> bool {
        self.is_example
    }
    fn reset(&mut self) {
        // TODO
        self.vm.globals().clear();
        self.populate_scope();
    }
    fn populate_scope(&mut self) {
        let lua = &mut self.vm;
        let globals = lua.globals();

        let table_color = |Color { r, g, b, a }: Color| {
            lua.create_table_from([("r", r), ("g", g), ("b", b), ("a", a)])
                .and_then(|t| {
                    t.set_readonly(true);
                    Ok(t)
                })
        };
        let table_s = readonly_table!(lua, Vec2 { x, y });
        let table_vec2 = |Vec2 { x, y }: Vec2| {
            lua.create_table_from([("x", x), ("y", y)]).and_then(|t| {
                t.set_readonly(true);
                Ok(t)
            })
        };
        let table_vec3 = |Vec3 { x, y, z }: Vec3| {
            lua.create_table_from([("x", x), ("y", y), ("z", z)])
                .and_then(|t| {
                    t.set_readonly(true);
                    Ok(t)
                })
        };
        let table_rect = |Rect { x, y, w, h }: Rect| {
            lua.create_table_from([("x", x), ("y", y), ("w", w), ("h", h)])
                .and_then(|t| {
                    t.set_readonly(true);
                    Ok(t)
                })
        };

        for (name, color) in COLORS {
            globals.set(name, table_color(color).unwrap()).unwrap();
        }
        for (key, code) in ffi::KEYS {
            globals.set(key, code as u16);
        }
        for (name, mb) in ffi::MOUSE_BUTTONS {
            globals.set(name, mb as u8);
        }
    }
}

impl ScriptEngine for Engine {
    type Script = Script;

    fn extension<'a>() -> &'a str {
        "lua"
    }
    fn expose_layer(&mut self) {
        // TODO
    }
    fn write_examples(&mut self, warnings: &mut Vec<String>) -> Result<(), Vec<std::io::Error>> {
        // TODO
        Ok(())
    }
    fn scripts<'a>(&'a mut self) -> &'a mut [Self::Script] {
        &mut self.scripts
    }
    fn load_scripts(
        &mut self,
        console: &mut crate::menu::Console,
        errors: &mut ErrorMap,
        from: &[ScriptDir],
    ) -> anyhow::Result<()> {
        let files = read_files_from_dir(
            &[&dirs().scripts, &dirs().examples],
            lua_engine::Engine::extension(),
            console,
            errors,
        )?;

        // TODO: add_error()

        for (entry, contents, example) in files {
            let path = entry.path();

            // Extract metadata
            let metadata = if let Ok(m) = entry.metadata() {
                m
            } else {
                console.err(format!(
                    "Failed to read metadata of file '{entry:?}', skipping"
                ));
                continue;
            };

            let modified = if let Ok(m) = metadata.modified() {
                m
            } else {
                console.err(format!(
                    "Failed to read modification time of file '{entry:?}', skipping"
                ));
                continue;
            };

            // Check for existing
            let existing_idx = self
                .scripts
                .iter()
                .enumerate()
                .find(|(_, s)| s.path == path && s.modified != modified)
                .and_then(|(i, _)| Some(i));

            // Update the script
            if let Some(idx) = existing_idx {
                // HACK: do not store the script, to avoid mutably borrowing self
                console.log(format!("Updating existing script {path:?}"));
                let prev_time = self.scripts[idx].modified;
                self.scripts[idx] =
                    self.compile_and_init_script(path.clone(), contents, true, prev_time)?;
                continue;
            }

            // New script
            console.log(format!("Adding new script {path:?}"));
            let now = SystemTime::now();

            // TODO: handle
            let script = self.compile_and_init_script(path.clone(), contents, example, now)?;

            self.scripts.push(script);
        }

        // TODO
        Ok(())
    }
    fn call_function(&mut self, script_index: usize, name: impl AsRef<str>) -> anyhow::Result<()> {
        // NOTE: Possible OOB
        let script = &mut self.scripts[script_index];
        let globals = script.vm.globals();
        let func: Function = globals.get(name.as_ref())?;
        // Call the function
        func.call::<_, ()>(())?;

        Ok(())
    }
    fn reload_scripts(
        &mut self,
        console: &mut crate::menu::Console,
        errors: &mut ErrorMap,
    ) -> anyhow::Result<()> {
        // TODO
        Ok(())
    }
}
