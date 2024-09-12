use super::*;
use crate::error::ErrorPage;
use crate::script::*;
use macroquad::prelude::*;

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
        // TODO: Make it report error if script reload caused error

        if is_key_pressed(KeyCode::F10) {
            let l = &mut self.logger;
            if l.enabled {
                l.warn("disabling logging! Reenable with F10");
                l.enabled = false;
            } else {
                l.enabled = true;
                l.log("Enabling logging!");
            };
        }
        self.show_fps ^= is_key_pressed(KeyCode::F12);

        if is_key_pressed(KeyCode::F5) {
            self.reload_scripts();
        }

        if self.error.is_some() {
            return;
        }

        if is_key_pressed(KeyCode::Escape) {
            self.selected = None;
        }

        if let Some(index) = self.selected {
            let script = &mut self.engine.scripts[index];
            let result =
                self.engine
                    .engine
                    .call_fn::<()>(&mut script.scope, &script.ast, "update", ());

            if let Err(e) = result {
                self.logger
                    .err(format!("Error while executings script -> update(): {e}"));
            }
        }
    }
}
