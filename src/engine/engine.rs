use std::path::Path;
use std::{
    cell::{LazyCell, OnceCell},
    io,
    path::PathBuf,
};

pub const ENGINE_NAME: &str = if cfg!(feature = "rhai-engine") {
    "rhai"
} else if cfg!(feature = "lua-engine") {
    "lua"
} else {
    unimplemented!()
};

use super::*;
use crate::menu::Console;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptDir {
    Scripts,
    Examples,
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

    fn reload_scripts(console: &mut Console, errors: &mut ErrorMap) -> anyhow::Result<()> {
        Ok(())
    }
}

pub trait GameScript {
    fn path<'a>(&'a self) -> &'a Path;
    fn name<'a>(&'a self) -> Option<&'a str>;
    fn is_example(&self) -> bool;
    fn reset(&mut self);
}
