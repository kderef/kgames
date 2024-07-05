use macroquad::prelude::*;

pub trait Game {
    fn title(&self) -> &str;
    fn icon(&self) -> Option<&Texture2D> { None }
    fn init() -> Self where Self: Sized;
    fn update(&mut self) {}
    fn draw(&self) {}
}

/// `Vec<Box<dyn Game>>`
#[macro_export]
macro_rules! game_objects {
    ($($t:tt),* $(,)*) => {
        vec! [
            $(
                Box::new($t::init()),
            )*
        ]
    };
}