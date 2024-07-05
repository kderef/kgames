use macroquad::prelude::*;

pub trait Game {
    fn title(&self) -> &'static str;
    fn icon(&self) -> &Texture2D;
    fn init(&mut self) {}
    fn update(&mut self) {}
    fn draw(&self) {}
}