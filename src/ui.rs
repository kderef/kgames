use std::fmt::Display;

use colored::Colorize;
use macroquad::prelude::*;

pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1. }
}

#[derive(Clone, Copy, Debug)]
pub struct UI {
    pub bg: Color,
    pub fg: Color,
    pub border: Color,
    pub bg_hover: Color,
    pub bg_click: Color,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            bg: BLACK,
            fg: WHITE,
            border: GRAY,
            bg_hover: DARKGRAY,
            bg_click: LIGHTGRAY,
        }
    }
}

impl UI {
    pub fn new(bg: Color, fg: Color, border: Color) -> Self {
        Self {
            bg,
            fg,
            border,
            bg_hover: Color::new(bg.r, bg.g, bg.b, bg.a * 0.1),
            bg_click: Color::new(bg.r, bg.g, bg.b, bg.a * 1.1),
        }
    }
    fn button_impl(&self, bounds: Rect) -> bool {
        let mouse_pos: Vec2 = mouse_position().into();
        let mouse_hov = bounds.contains(mouse_pos);
        let mouse_clk = mouse_hov && is_mouse_button_pressed(MouseButton::Left);

        let color = if mouse_clk {
            self.bg_click
        } else if mouse_hov {
            self.bg_hover
        } else {
            self.bg
        };

        draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, color);
        draw_rectangle_lines(bounds.x, bounds.y, bounds.w, bounds.h, 5.0, self.border);

        mouse_clk
    }
    pub fn button_icon(&self, icon: &Texture2D, bounds: Rect) -> bool {
        let clicked = self.button_impl(bounds);

        let (x, y, w, h) = (
            bounds.x + 10.0,
            bounds.y + 10.0,
            bounds.w - 20.0,
            bounds.h - 20.0,
        );

        draw_texture_ex(
            icon,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );

        clicked
    }

    pub fn button(&self, text: impl AsRef<str>, bounds: Rect, font_size: f32) -> bool {
        let text = text.as_ref();
        let clicked = self.button_impl(bounds);

        let center = bounds.center();
        let text_size = measure_text(text, None, font_size as u16, 1.0);
        let pos = center + vec2(-text_size.width / 2., text_size.height / 3.);

        draw_text(text, pos.x, pos.y, font_size, self.fg);

        clicked
    }
}

pub struct Logger {
    pub enabled: bool,
}

impl Logger {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    pub fn log(&self, text: impl Display) {
        if !self.enabled {
            return;
        }

        // Log the information
        let log = "info".green();
        println!("{log} {text}")
    }
    pub fn err(&self, text: impl Display) {
        if !self.enabled {
            return;
        }
        let error = "error".bright_red().bold();
        println!("{error} {text}")
    }
    pub fn note(&self, text: impl Display) {
        let note = "note".bright_blue().bold();
        println!("{note} {text}");
    }
    pub fn warn(&self, text: impl Display) {
        let warning = "warn".yellow().bold();
        println!("{warning} {text}");
    }
}
