use super::*;
use macroquad::prelude::*;

// Dialog system
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum DialogOption {
    Yes,
    No,
    Ok,
    Cancel,
}

pub struct Dialog<'a> {
    title: &'a str,
    message: &'a str,
    options: &'a [DialogOption],
}

impl<'a> Dialog<'a> {
    pub fn new(title: &'a str, msg: &'a str, options: &'a [DialogOption]) -> Self {
        Self {
            title,
            message: msg,
            options,
        }
    }
    pub fn show(&self, ui: &UI) -> Option<DialogOption> {
        let (screen_w, screen_h) = (screen_width(), screen_height());
        let mut chosen = None;

        let font_size = 20.0;
        let (w, h) = (400.0, 200.0);
        let (x, y) = ((screen_w - w) / 2., (screen_h - h) / 2.);

        draw_rectangle(x, y, w, h, ui.bg);
        draw_rectangle(x, y, w, font_size, ui.border);
        draw_rectangle_lines(x, y, w, h, 1.0, ui.border);

        let spacing = 10.0;

        // Draw title
        draw_text(self.title, x + 1.0, y + font_size / 1.5, font_size, ui.fg);

        // Draw message
        let message_x = x + spacing; // Align the message with the left spacing
        let message_y = y + font_size + 15.0; // Start drawing the message just below the title
        let message_width = w - (2.0 * spacing); // Width should leave some padding on both sides

        draw_text(self.message, message_x, message_y, font_size, ui.fg); // You can adjust the y-coordinate or text size as needed

        // Draw buttons
        let button_height = 50.0;
        let buttons = self.options.len() as f32;
        let button_width = (w - (spacing * (buttons + 1.0))) / buttons;

        let mut bounds = Rect {
            x: x + spacing,
            y: y + h - spacing - button_height,
            w: button_width,
            h: button_height,
        };

        for option in self.options {
            if ui.button(format!("{option:?}"), bounds, font_size) {
                chosen = Some(*option);
            }
            bounds.x += button_width + spacing;
        }

        if ui.active {
            chosen
        } else {
            None
        }
    }
}
