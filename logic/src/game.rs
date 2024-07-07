use macroquad::prelude::*;

use crate::wrap::ctx::Context;

pub trait Game {
    fn title(&self) -> &str;
    fn icon(&self) -> Option<&Texture2D> { None }
    fn init(ctx: &dyn Context) -> Self where Self: Sized;
    fn update(&mut self, ctx: &dyn Context) {}
    fn reset(&mut self, ctx: &dyn Context) {}
    fn draw(&mut self, ctx: &dyn Context) {}
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
                Box::new($t::init(), &ctx),
            )*
        ]
    };
}