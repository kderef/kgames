use macroquad::prelude::*;
use std::path::{Path, PathBuf};

use crate::{script::Engine, ui::Logger};

pub struct ErrorPage {
    context: String,
    errors: Vec<(PathBuf, anyhow::Error)>,
}

fn draw_centered(text: &str, y: f32, size: f32, color: Color) {
    let dims = measure_text(text, None, size as u16, 1.0);
    let screen_width = screen_width();
    let screen_center_x = screen_width / 2.0;

    draw_text(text, screen_center_x - dims.width / 2.0, y, size, color);
}

impl ErrorPage {
    pub fn new(errors: Vec<(PathBuf, anyhow::Error)>, ctx: impl ToString) -> Self {
        Self {
            errors,
            context: ctx.to_string(),
        }
    }

    fn draw_errors(&mut self, y: &mut f32) {
        let size = 30.0;
        for (i, (source, error)) in self.errors.iter().enumerate() {
            let source = source.to_string_lossy();
            let i = i + 1;

            let size = 60.0;
            *y += size;
            draw_centered(&format!("Error #{i}"), *y, size, WHITE);

            // Draw message
            let size = 30.0;
            *y += size;
            draw_centered(&format!("Source: {source}"), *y, size, LIGHTGRAY);

            // Draw error
            let size = 30.0;
            *y += size;
            draw_centered(&format!("Error: {error}"), *y, size, LIGHTGRAY);
        }
    }

    /// Returns if should keep showing
    pub fn show(&mut self, engine: &mut Engine, logger: &mut Logger) -> bool {
        let bg = Color::new(0.7, 0., 0., 1.);
        clear_background(bg);

        // Draw Title
        let (sw, _sh) = (screen_width(), screen_height());

        let err_size = (sw / 5.).clamp(10., 200.);
        draw_centered("ERROR", err_size, err_size, WHITE);

        let mut y = err_size + 10.0;
        // Draw error count
        let size = 20.0;
        y += size;
        draw_centered(
            &format!("Encountered {} errors", self.errors.len()),
            y,
            size,
            WHITE,
        );

        self.draw_errors(&mut y);

        let size = 50.0;
        y += size * 2.0;
        draw_centered("Press Escape to return", y, size, WHITE);
        let size = 50.0;
        y += size;
        draw_centered("Press F5 to reload scripts", y, size, WHITE);

        if is_key_pressed(KeyCode::Escape) {
            return false;
        }
        if is_key_pressed(KeyCode::F5) {
            self.errors.clear();
            if let Err(e) = engine.load_scripts(logger, &mut self.errors) {
                self.context = format!("Failed to reload scripts: {e}")
            }
        }

        true
    }
}
