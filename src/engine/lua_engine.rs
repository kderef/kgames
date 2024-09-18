use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use super::*;

pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Script {
    path: PathBuf,
    is_example: bool,
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
    }
    fn populate_scope(&mut self) {
        // TODO
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
        // TODO
        todo!()
    }
    fn load_scripts(
        &mut self,
        console: &mut crate::menu::Console,
        errors: &mut ErrorMap,
        from: &[ScriptDir],
    ) -> anyhow::Result<()> {
        // TODO
        Ok(())
    }
    fn call_function(&mut self, script_index: usize, name: impl AsRef<str>) -> anyhow::Result<()> {
        // TODO
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
