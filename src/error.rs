use macroquad::{
    prelude::*,
    ui::{root_ui, Id, Skin},
};
use std::path::{Path, PathBuf};

use crate::{script::Engine, ui::Logger};

pub struct ErrorPage {
    context: String,
    errors: Vec<(PathBuf, anyhow::Error)>,
}

fn sub_strings(string: &str, sub_len: usize) -> Vec<&str> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}

fn draw_centered(text: &str, y: f32, size: f32, color: Color) {
    let dims = measure_text(text, None, size as u16, 1.0);
    let screen_width = screen_width();
    let screen_center_x = screen_width / 2.0;

    draw_text(text, screen_center_x - dims.width / 2.0, y, size, color);
}

fn draw_clamped(text: &str, x: f32, y: f32, size: f32, color: Color) -> usize {
    let screen_width = screen_width();
    let mut current_x = x;
    let mut current_y = y;
    let mut num_rows = 1;

    for word in text.split_whitespace() {
        let word_width = measure_text(word, None, size as u16, 1.0).width;

        // Check if the word fits in the current line
        if current_x + word_width > screen_width {
            // Move to the next line
            current_x = x;
            current_y += size;
            num_rows += 1;
        }

        draw_text(word, current_x, current_y, size, color);

        // Move the cursor to the end of the word, including a space
        current_x += word_width + measure_text(" ", None, size as u16, 1.0).width;
    }

    num_rows
}

impl ErrorPage {
    pub fn new(errors: Vec<(PathBuf, anyhow::Error)>, ctx: impl ToString) -> Self {
        Self {
            errors,
            context: ctx.to_string(),
        }
    }

    fn draw_errors(&mut self, y: &mut f32) {
        let screen_w = screen_width();

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

            *y += 25.0;

            // Draw error
            let width_base = screen_w;
            let size = vec2(width_base - 20.0 * 2., 30.0);

            let error = error.to_string();
            draw_clamped(&error, 10., *y, size.y, WHITE);

            *y += size.y;
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
