use super::Menu;
use super::*;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

impl<'a, E: ScriptEngine> Menu<'a, E> {
    pub fn draw_games(&mut self) {
        clear_background(self.background);

        // FIXME: remove
        for (i, name) in self.engine.scripts().iter().enumerate() {
            let i = i + 1;
            let (x, y, w, h) = (200., 200., 500., 50.);
            draw_rectangle(x, y + h * i as f32, w, h, BLACK);
            draw_text(
                &format!("{i} - {}", name.name().unwrap_or("INVALID_NAME")),
                x,
                y + h * i as f32 + h / 2.,
                h * 0.8,
                WHITE,
            );
            if is_key_pressed(unsafe { std::mem::transmute(KeyCode::Key0 as u16 + i as u16) }) {
                self.state = State::Playing(i - 1);
                return;
            }
        }

        let skin = self.ui.skin();

        let mut d = String::new();

        root_ui().push_skin(&skin);
        // root_ui().button(vec2(400.0, 200.0), "Hello");
        root_ui().input_text(hash!(), "Search ...", &mut self.ui.query);
    }
}
