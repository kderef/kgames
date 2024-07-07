#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use menu::Menu;
use miniquad::conf::Platform;

mod game;
mod games;
mod menu;
mod ui;

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
    let mut menu = Menu::new();

    loop {
        menu.update();
        menu.draw();
        #[cfg(debug_assertions)]
        draw_text(&format!("FPS: {}", get_fps()), 0., 20., 20., GREEN);
        next_frame().await;
    }
}