use super::*;
use crate::error::ErrorPage;
use crate::script::*;
use macroquad::prelude::*;
use miniquad::window::{dropped_file_bytes, dropped_file_count, dropped_file_path};

impl<'a> Menu<'a> {
    pub fn reload_scripts(&mut self) {
        self.logger.log("### Reloading scripts");
        let mut errors = vec![];
        if let Err(e) = self.engine.load_scripts(
            &mut self.logger,
            &mut errors,
            &[ScriptDir::Examples, ScriptDir::Scripts],
        ) {
            self.error = Some(ErrorPage::new(errors, e));
        }
    }
    /// Call update() of the script, and update menu state
    #[inline]
    pub fn update(&mut self) {
        self.key_entered = false;

        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::F10 => {
                    let l = &mut self.logger;
                    if l.enabled {
                        l.warn("disabling logging! Reenable with F10");
                        l.enabled = false;
                    } else {
                        l.enabled = true;
                        l.log("Enabling logging!");
                    };
                }
                KeyCode::F5 => {
                    self.reload_scripts();
                }
                KeyCode::F12 => {
                    self.show_fps ^= true;
                }
                _ => {
                    // TODO: make list-by-query stored
                    self.key_entered = true;
                }
            }
        }

        if self.error.is_some() {
            return;
        }

        match self.state {
            State::Playing(game) => {
                let script = &mut self.engine.scripts[game];
                let result =
                    self.engine
                        .engine
                        .call_fn::<()>(&mut script.scope, &script.ast, "update", ());

                if let Err(e) = result {
                    self.logger
                        .err(format!("Error while executing script -> update(): {e}"));
                }
            }
            State::Settings => {}
            State::Menu => {}
            State::Games => {}
        }

        if is_key_pressed(KeyCode::Escape) {
            if self.dialog.is_none() {
                self.dialog = None;
                return;
            }
            // Reset game on escape
            if let State::Playing(game) = self.state {
                let script = &mut self.engine.scripts[game];
                script.scope.clear();
                self.reload_scripts();
            }
            self.state = State::Menu;
        }
    }
}
