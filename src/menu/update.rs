use super::*;
use crate::key;
use crate::script::*;
use crate::{cross::fuzzy_search, error::ErrorPage};
use macroquad::prelude::*;
use miniquad::window::{dropped_file_bytes, dropped_file_count, dropped_file_path};

impl<'a> Menu<'a> {
    pub fn reload_scripts(&mut self) {
        self.console.log("### Reloading scripts");
        let mut errors = vec![];
        if let Err(e) = self.engine.load_scripts(
            &mut self.console,
            &mut errors,
            &[ScriptDir::Examples, ScriptDir::Scripts],
        ) {
            self.error = Some(ErrorPage::new(errors, e));
        }
    }
    /// Call update() of the script, and update menu state
    #[inline]
    pub fn update(&mut self) {
        let console_open = self.console.is_open();
        self.ui.active = !console_open;

        if console_open {
            return;
        }

        self.key_entered = false;

        if let Some(key) = get_last_key_pressed() {
            match key {
                key::REFRESH => {
                    self.reload_scripts();
                }
                key::FPS => {
                    self.show_fps ^= true;
                }
                _key => {
                    let needle = &self.ui.query;

                    if !needle.is_empty() {
                        // A Key was entered into the search bar
                        let haystack = self.engine.scripts.iter().map(|s| s.name());

                        let min_score = 20;

                        self.matches = fuzzy_search(&self.matcher, needle, haystack, min_score);
                        self.console.log(format!(
                            "Fuzzy search '{needle}' with min_score = {min_score} returned {:#?}",
                            self.matches
                        ));

                        self.key_entered = true;
                    }
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
                    self.console
                        .err(format!("Error while executing script -> update(): {e}"));
                }
            }
            State::Settings => {}
            State::Menu => {}
            State::Games => {}
        }

        if is_key_pressed(KeyCode::Escape) {
            if self.dialog.is_some() {
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
