mod common;
mod engine;

pub mod ffi;
pub mod texture;

#[cfg(feature = "rhai-engine")]
mod rhai_engine;

#[cfg(feature = "lua-engine")]
mod lua_engine;

pub use common::*;
pub use engine::*;
use std::fmt::Display;

use std::path::Path;
use std::{
    cell::{LazyCell, OnceCell},
    io,
    path::PathBuf,
};

pub const IS_RHAI: bool = cfg!(feature = "rhai-engine");
pub const IS_LUA: bool = cfg!(feature = "lua-engine");

// Types
#[cfg(feature = "rhai-engine")]
pub mod scripting {
    pub use rhai::{EvalAltResult, ImmutableString};
    pub type Error = Box<EvalAltResult>;
    pub type Result<T> = std::result::Result<T, Error>;
    pub use super::rhai_engine::Engine;
}

#[cfg(feature = "lua-engine")]
mod scripting {
    pub type Error = mlua::Error;
    pub type Result<T> = std::result::Result<T, Error>;
    pub use super::lua_engine::Engine;
}

pub use scripting::Engine;

pub fn external_error(e: impl ToString + Display) -> scripting::Error {
    use scripting::*;
    #[cfg(feature = "rhai-engine")]
    return Box::new(EvalAltResult::from(e.to_string()));
    #[cfg(feature = "lua-engine")]
    return scripting::Error::external(anyhow::anyhow!("{e}"));
}

pub const ENGINE_NAME: &str = if cfg!(feature = "rhai-engine") {
    "rhai"
} else if cfg!(feature = "lua-engine") {
    "lua"
} else {
    unimplemented!()
};

use console::Console;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptDir {
    Scripts,
    Examples,
}
impl ScriptDir {
    pub fn path(self) -> &'static Path {
        match self {
            Self::Scripts => &dirs().scripts,
            Self::Examples => &dirs().examples,
        }
    }
}

pub type ErrorMap = Vec<(PathBuf, anyhow::Error)>;

pub trait ScriptEngine {
    type Script: GameScript;

    fn extension<'a>() -> &'a str;
    fn expose_layer(&mut self);
    fn write_examples(&mut self, warnings: &mut Vec<String>) -> Result<(), Vec<io::Error>>;

    fn scripts<'a>(&'a mut self) -> &'a mut [Self::Script];

    fn load_scripts(
        &mut self,
        console: &mut Console,
        errors: &mut ErrorMap,
        from: &[ScriptDir],
    ) -> anyhow::Result<()>;

    fn call_function(&mut self, script_index: usize, name: impl AsRef<str>) -> anyhow::Result<()>;

    fn reload_scripts(
        &mut self,
        console: &mut Console,
        errors: &mut ErrorMap,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

pub trait GameScript {
    fn path<'a>(&'a self) -> &'a Path;
    fn name<'a>(&'a self) -> Option<&'a str>;
    fn is_example(&self) -> bool;
    fn reset(&mut self);
    fn populate_scope(&mut self);
}
