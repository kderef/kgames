use std::fmt::Display;

use colored::Colorize;
use macroquad::ui::root_ui;
use macroquad::{prelude::*, ui::Skin};

pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1. }
}

#[derive(Clone, Debug)]
pub struct UI {
    pub bg: Color,
    pub fg: Color,
    pub border: Color,
    pub bg_hover: Color,
    pub bg_click: Color,
    pub font: Font,
    pub query: String,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            bg: BLACK,
            fg: WHITE,
            border: GRAY,
            bg_hover: DARKGRAY,
            bg_click: LIGHTGRAY,
            font: {
                let mut f =
                    load_ttf_font_from_bytes(include_bytes!("../res/CnC-RedAlert.ttf")).unwrap();
                f.set_filter(FilterMode::Nearest);
                f
            },
            query: String::new(),
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
            ..Default::default()
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

    pub fn skin(&self) -> Skin {
        let Self {
            bg,
            fg,
            bg_hover,
            bg_click,
            border,
            font,
            ..
        } = self;

        let font_size = 38.0;
        let margin = font_size / 2.;

        let base = || {
            root_ui()
                .style_builder()
                .text_color(*fg)
                .color(*bg)
                .color_hovered(*bg_hover)
                .color_clicked(*bg_click)
                .font_size(font_size as u16)
                .with_font(font)
                .unwrap()
        };

        Skin {
            label_style: base().build(),
            button_style: base()
                .margin(RectOffset::new(margin, margin, 0., 0.))
                .build(),
            tabbar_style: base().build(),
            combobox_style: base().build(),
            window_style: base().build(),
            editbox_style: base().build(),
            window_titlebar_style: base().build(),
            scrollbar_style: base().build(),
            scrollbar_handle_style: base().build(),
            checkbox_style: base().build(),
            group_style: base().build(),
            margin: 20.0,
            // title_height: default,
            // scroll_width: default,
            // scroll_multiplier: default,
            ..root_ui().default_skin()
        }
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

// Dialog system
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
        let message_y = y + font_size + 10.0; // Start drawing the message just below the title
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

        chosen
    }
}
