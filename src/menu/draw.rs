use super::*;
use crate::{cross, ui::DialogOption};
use macroquad::prelude::*;
use miniquad::window::request_quit;

impl<'a> Menu<'a> {
    #[allow(unused)]
    fn draw_ui_temp(&mut self, y: f32) {
        // FIXME: temporary solution
        for (i, name) in self.engine.scripts.iter().enumerate() {
            let i = i + 1;
            let (x, y, w, h) = (200., 200., 500., 50.);
            draw_rectangle(x, y + h * i as f32, w, h, BLACK);
            draw_text(
                &format!("{i} - {}", name.name()),
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
    }

    fn draw_ui(&mut self, y: f32) {
        let (screen_w, screen_h) = (screen_width(), screen_height());

        // draw buttons
        let button_width = (screen_w / 3.0).clamp(100.0, 300.0);
        let button_height = 50.0;
        let font_size = 30.0;

        let mut button_bounds = Rect {
            x: (screen_w - button_width) / 2.,
            y: (screen_h / 2.) - button_height * 3.0,
            w: button_width,
            h: button_height,
        };

        let spacing = 10.0;

        let mut button = |text: &str| {
            button_bounds.y += button_bounds.h + spacing;
            self.ui.button(text, button_bounds, font_size)
        };

        if button("Play") {
            self.state = State::Games;
        }
        if button("Settings") {
            self.state = State::Settings;
        }
        if button("Exit") {
            self.dialog = Some(Dialog::new(
                "Exit",
                "Do you really want to exit?",
                &[DialogOption::Yes, DialogOption::No],
            ));
        }
    }

    #[inline]
    fn draw_fps(&self) {
        let fps = get_fps();
        let color = match fps {
            50.. => GREEN,
            30.. => ORANGE,
            _ => RED,
        };
        draw_text(&format!("FPS: {fps}"), 0., 20., 20., color);
    }

    #[inline]
    pub fn draw(&mut self) {
        if let Some(ref mut err) = self.error {
            if !err.show(&mut self.engine, &mut self.logger) {
                self.error = None;
            }
            return;
        }

        let (screen_w, screen_h) = (screen_width(), screen_height());

        match self.state {
            State::Playing(game) => {
                let script = &mut self.engine.scripts[game];
                let result =
                    self.engine
                        .engine
                        .call_fn::<()>(&mut script.scope, &script.ast, "draw", ());

                if let Err(e) = result {
                    self.logger
                        .err(format!("Error while executings script: {e}"));
                }

                if self.logger.enabled {
                    self.draw_fps();
                }

                return;
            }
            State::Menu => {
                self.draw_menu();
            }
            State::Settings => {
                self.draw_settings();
            }
            State::Games => {
                self.draw_games();
            }
        }

        // Draw dialog
        if let Some(ref dialog) = self.dialog {
            const OVERLAY: Color = Color::new(0., 0., 0., 0.4);
            draw_rectangle(0., 0., screen_w, screen_h, OVERLAY);

            dialog.draw(&self.ui);
        }
    }

    fn draw_menu(&mut self) {
        let (screen_w, screen_h) = (screen_width(), screen_height());

        clear_background(self.background);

        // Draw the Title
        let (sw, sh) = (screen_width(), screen_height());
        let title = "KGames";
        let title_size = (sw / 10.).clamp(40.0, 105.0);
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        let title_pos = vec2(sw / 2. - title_dims.width / 2., title_size);

        draw_text(title, title_pos.x, title_pos.y, title_size, WHITE);

        // Draw UI
        self.draw_ui(title_pos.y + 20.0);

        // Draw small subtext
        draw_text(
            concat!("v", env!("CARGO_PKG_VERSION")),
            0.0,
            sh - 5.0,
            15.0,
            GRAY,
        );

        //===== Draw UI =====//
        // Folder button
        let (w, h) = (60.0, 60.0);
        let mut bounds = Rect {
            x: screen_w - w - 10.0,
            y: 10.0,
            w,
            h,
        };

        if self.ui.button_icon(self.folder_icon, bounds) {
            if let Err(e) = cross::open_path(&self.engine.global_dir) {
                self.logger.err(e);
            }
        }
        bounds.x -= w + 10.0;

        if self.ui.button_icon(self.refresh, bounds) {
            self.reload_scripts();
        }
        bounds.x -= w + 10.0;

        if self.ui.button_icon(self.help, bounds) {
            if let Err(e) = cross::open_path(&self.readme) {
                self.logger.err(e);
            }
        }
        //===== Draw FPS =====//
        if self.show_fps {
            self.draw_fps();
        }
    }
}
