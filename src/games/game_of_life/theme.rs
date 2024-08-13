use std::ops::Index;

use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Style {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
}

#[derive(Clone, Copy)]
pub enum Theme {
    Default(Style),
    Gruvbox(Style),
    Matrix(Style),
    Midnight(Style),
    Bolus(Style),
}
impl Theme {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Default(_) => "Default",
            Self::Gruvbox(_) => "Gruvbox",
            Self::Matrix(_) => "Matrix",
            Self::Midnight(_) => "Midnight",
            Self::Bolus(_) => "BOLUS",
        }
    }
}

pub const THEMES: [Theme; 5] = [
    Theme::Default(Style {
        bg: rgb(0., 0., 0.),
        fg: rgb(1., 1., 1.),
        accent: rgb(0., 0.894, 0.188),
    }),
    Theme::Gruvbox(Style {
        bg: rgb(0.156, 0.156, 0.156),
        fg: rgb(0.921, 0.858, 0.698),
        accent: rgb(0.800, 0.141, 0.113),
    }),
    Theme::Matrix(Style {
        bg: rgb(0.074, 0.090, 0.129),
        fg: rgb(0.196, 0.776, 0.011),
        accent: rgb(0.000, 0.474, 0.945),
    }),
    Theme::Midnight(Style {
        bg: rgb(0., 0., 0.),
        fg: rgb(0.784, 0.784, 0.784),
        accent: rgb(0., 0.474, 0.945),
    }),
    Theme::Bolus(Style {
        bg: rgb(0., 0., 0.),
        fg: rgb(1., 1., 1.),
        accent: rgb(0., 0.894, 0.188),
    }),
];

const BOLUS_INDEX: usize = {
    let mut i = 0;
    while i < THEMES.len() {
        if let Theme::Bolus(_) = THEMES[i] {
            break;
        }
        i += 1;
    }
    i
};

fn toggle_bolus(current: &mut usize) -> Theme {
    *current = if *current == BOLUS_INDEX {
        0
    } else {
        BOLUS_INDEX
    };
    THEMES[*current]
}

fn next(current: &mut usize) -> Theme {
    *current += 1;
    if *current >= THEMES.len() {
        *current = 0;
    }

    THEMES[*current]
}

const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1.0 }
}
