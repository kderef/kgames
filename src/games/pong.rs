use crate::game::Game;
use macroquad::prelude::*;

pub struct Pong {
    icon: Texture2D,
}

impl Game for Pong {
    fn init() -> Self {
        Self {
            icon: Texture2D::empty()
        }
    }
    fn title(&self) -> &'static str {
        "Pong"
    }
    fn draw(&self) {
        
    }
    fn update(&mut self) {
        
    }
}