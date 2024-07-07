use macroquad::prelude::*;

pub trait Game {
    fn title(&self) -> &str;
    fn icon(&self) -> Option<&Texture2D> { None }
    fn init() -> Self where Self: Sized;
    fn update(&mut self) {}
    fn reset(&mut self) {}
    fn draw(&mut self) {
        clear_background(PURPLE);
        draw_text("DRAW NOT IMPLEMENTED", 100.0, 100.0, 100.0, WHITE);
    }
    fn requested_exit(&self) -> bool {false}
}

/// `Vec<Box<dyn Game>>`
#[macro_export]
macro_rules! game_objects_vec {
    ($($t:tt),* $(,)*) => {
        vec! [
            $(
                Box::new($t::init()),
            )*
        ]
    };
}
/// `[Box<dyn Game>]`
#[macro_export]
macro_rules! game_objects {
    ($($t:tt),* $(,)*) => {
        [
            $(
                Box::new($t::init()),
            )*
        ]
    };
}