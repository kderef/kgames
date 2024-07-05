use crate::game::Game;
use macroquad::prelude::*;

pub struct Breakout {
    icon: Texture2D,
}

impl Game for Breakout {
    fn init() -> Self {
        Self {
            icon: Texture2D::empty()
        }
    }
    fn title(&self) -> &'static str {
        "Breakout"
    }
    fn draw(&self) {
        
    }
    fn update(&mut self) {
        
    }
}