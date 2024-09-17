use std::{
    cell::{LazyCell, OnceCell},
    io,
    path::PathBuf,
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
    fn extension<'a>() -> &'a str;
    fn expose_layer(&mut self);
    fn write_examples(&mut self, warnings: &mut Vec<String>) -> Result<(), Vec<io::Error>>;

    fn load_scripts(
        console: &mut Console,
        errors: &mut ErrorMap,
        from: &[ScriptDir],
    ) -> anyhow::Result<()>;

    fn reload_scripts(console: &mut Console, errors: &mut ErrorMap) -> anyhow::Result<()> {
        Ok(())
    }
}
