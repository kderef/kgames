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
    pub active: bool,
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
                    load_ttf_font_from_bytes(include_bytes!("../../res/CnC-RedAlert.ttf")).unwrap();
                f.set_filter(FilterMode::Nearest);
                f
            },
            query: String::new(),
            active: true,
        }
    }
}

impl UI {
    pub const BORDER_W: f32 = 5.0;
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

        let color = if !self.active {
            self.bg
        } else if mouse_clk {
            self.bg_click
        } else if mouse_hov {
            self.bg_hover
        } else {
            self.bg
        };

        draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, color);
        draw_rectangle_lines(
            bounds.x,
            bounds.y,
            bounds.w,
            bounds.h,
            Self::BORDER_W,
            self.border,
        );

        mouse_clk && self.active
    }
    pub fn button_icon(&self, icon: &Texture2D, bounds: Rect, hover: impl AsRef<str>) -> bool {
        let clicked = self.button_impl(bounds);
        let hov = bounds.contains(mouse_position().into());
        let hover = hover.as_ref();

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

        if hov {
            // Check for Out Of Bounds
            let mut popup = Rect::new(bounds.x, bounds.bottom(), hover.len() as f32 * 4., 50.0);
            let oob = screen_width() - popup.right();
            if oob < 0.0 {
                popup.x -= -oob;
            }

            // Draw rectangle
            draw_rectangle(popup.x, popup.y, popup.w, popup.h, self.bg);
            draw_rectangle_lines(
                popup.x,
                popup.y,
                popup.w,
                popup.h,
                Self::BORDER_W,
                self.border,
            );

            // Draw the text
            let chunk_size = popup.w * 0.13;
            let text_size = 15.0;
            let mut text_y = popup.y + text_size;
            let text_x = popup.x + 5.0;

            let text: Vec<String> = hover
                .chars()
                .collect::<Vec<_>>() // Collect characters into a vector
                .chunks(chunk_size as usize) // Split into chunks
                .map(|chunk| chunk.iter().collect::<String>()) // Convert each chunk back into a String
                .collect();

            for chunk in text {
                draw_text(&chunk, text_x, text_y, text_size, self.fg);
                text_y += text_size;
            }
        }

        clicked
    }

    pub fn button<S: AsRef<str>>(&self, text: S, bounds: Rect, font_size: f32) -> bool {
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
