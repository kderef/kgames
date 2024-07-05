use macroquad::{miniquad::conf::Platform, prelude::Conf};

mod game;
mod menu;

fn window() -> Conf {
    Conf {
        window_title: "KGames".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,
        icon: None,
        platform: Platform::default(),
    }
}

#[macroquad::main(window)]
async fn main() {
    println!("Hello, world!");
}