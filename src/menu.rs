use macroquad::prelude::*;
use rhai::Scope;
use std::path::PathBuf;

use crate::{
    error::ErrorPage,
    script::{Engine, ScriptDir},
    ui::Logger,
};

pub struct Menu<'a> {
    engine: Engine<'a>,
    selected: Option<usize>,
    logger: Logger,
    pub show_fps: bool,
    pub error: Option<ErrorPage>,
}

// File functions
impl<'a> Menu<'a> {}

impl<'a> Menu<'a> {
    pub fn new(engine: Engine<'a>, logger: Logger) -> Self {
        Self {
            engine,
            logger,
            show_fps: false,
            selected: None,
            error: None,
        }
    }

    fn draw_selection(&mut self) {
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
                self.selected = Some(i - 1);
                return;
            }
        }
    }

    /// Call update() of the script, and update menu state
    #[inline]
    pub fn update(&mut self) {
        // TODO: Make it report error if script reload caused error

        if is_key_pressed(KeyCode::F10) {
            self.logger.enabled = !self.logger.enabled;
            self.logger.log(if self.logger.enabled {
                "Enabling logging!"
            } else {
                "WARNING: disabling logging! Reenable with F10"
            });
        }
        self.show_fps ^= is_key_pressed(KeyCode::F12);

        if is_key_pressed(KeyCode::F5) {
            let mut errors = vec![];
            if let Err(e) = self.engine.load_scripts(
                &mut self.logger,
                &mut errors,
                &[ScriptDir::Examples, ScriptDir::Scripts],
            ) {
                self.error = Some(ErrorPage::new(errors, e));
            }
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

    #[inline]
    fn draw_fps(&self) {
        let fps = get_fps();
        let color = if fps >= 50 {
            GREEN
        } else if fps >= 30 {
            ORANGE
        } else {
            RED
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

        if let Some(index) = self.selected {
            let script = &mut self.engine.scripts[index];
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

        const BG: Color = Color::new(0.11, 0.12, 0.12, 1.0);
        clear_background(BG);

        // Draw the Title
        let (sw, sh) = (screen_width(), screen_height());
        let title = "KGames";
        let title_size = (sw / 10.).clamp(60.0, sw);
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            sw / 2. - title_dims.width / 2.,
            title_size,
            title_size,
            WHITE,
        );

        // Draw All the games.
        // TODO: draw games
        self.draw_selection();

        // Draw small subtext
        draw_text(
            concat!("v", env!("CARGO_PKG_VERSION")),
            0.0,
            sh - 5.0,
            15.0,
            GRAY,
        );

        //===== Draw FPS =====//
        if self.show_fps {
            self.draw_fps();
        }
    }
}
