use macroquad::prelude::*;
use std::mem;

#[derive(Clone, Copy)]
pub struct Style {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Theme {
    Default,
    Gruvbox,
    Matrix,
    Midnight,
    Bolus,
}

impl Theme {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Gruvbox => "Gruvbox",
            Self::Matrix => "Matrix",
            Self::Midnight => "Midnight",
            Self::Bolus => "BOLUS",
        }
    }
    pub fn cycle(&mut self) {
        *self = unsafe {
            mem::transmute(
                // Max
                if let Self::Bolus = self {
                    0
                } else {
                    *self as u32 + 1
                },
            )
        };
    }
    pub const fn style(self) -> Style {
        match self {
            Self::Default => Style {
                bg: rgb(0., 0., 0.),
                fg: rgb(1., 1., 1.),
                accent: rgb(0., 0.894, 0.188),
            },
            Self::Gruvbox => Style {
                bg: rgb(0.156, 0.156, 0.156),
                fg: rgb(0.921, 0.858, 0.698),
                accent: rgb(0.800, 0.141, 0.113),
            },
            Self::Matrix => Style {
                bg: rgb(0.074, 0.090, 0.129),
                fg: rgb(0.196, 0.776, 0.011),
                accent: rgb(0.000, 0.474, 0.945),
            },
            Self::Midnight => Style {
                bg: rgb(0., 0., 0.),
                fg: rgb(0.784, 0.784, 0.784),
                accent: rgb(0., 0.474, 0.945),
            },
            Self::Bolus => Style {
                bg: rgb(0., 0., 0.),
                fg: rgb(1., 1., 1.),
                accent: rgb(0., 0.894, 0.188),
            },
        }
    }
}

const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1.0 }
}
