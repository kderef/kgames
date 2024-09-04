use std::fmt::Display;

use macroquad::prelude::*;

pub fn button(text: &str, bounds: Rect, font_size: f32) -> bool {
    const BG: Color = Color::new(0., 0., 0., 0.3);
    const BG_HOVER: Color = Color::new(0., 0., 0., 0.5);
    const BG_CLICK: Color = Color::new(0., 0., 0., 0.0);

    let mouse_pos: Vec2 = mouse_position().into();
    let mouse_hovered = bounds.contains(mouse_pos);
    let button_clicked = mouse_hovered && is_mouse_button_pressed(MouseButton::Left);

    let color = if button_clicked {
        BG_CLICK
    } else if mouse_hovered {
        BG_HOVER
    } else {
        BG
    };
    draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, color);

    let center = bounds.center();
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let text_x = center.x - text_size.width / 2.0;
    let text_y = center.y + text_size.height / 3.0;

    draw_text(text, text_x, text_y, font_size, WHITE);

    button_clicked
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
        println!("LOGGER: {text}")
    }
    pub fn err(&self, text: impl Display) {
        if !self.enabled {
            return;
        }

        println!("LOGGER: ERROR: {text}")
    }
}
