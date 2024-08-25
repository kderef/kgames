#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use menu::Menu;
use miniquad::conf::{Icon, Platform};
use script::Engine;

mod ffi;
mod menu;
mod script;
mod ui;

#[cfg(not(target_os = "macos"))]
fn window_icon() -> Icon {
    const BLACK: u8 = 0;

    Icon {
        small: [BLACK; 1024],
        medium: [BLACK; 4096],
        big: [BLACK; 16384],
    }
}

fn window() -> Conf {
    Conf {
        window_title: "KGames".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        fullscreen: false,
        sample_count: 4,
        window_resizable: true,
        // Window icon
        #[cfg(target_os = "macos")]
        icon: None,
        #[cfg(not(target_os = "macos"))]
        icon: Some(window_icon()),

        platform: Platform::default(),
    }
}

#[macroquad::main(window)]
async fn main() {
    let mut menu = Menu::new();
    let mut show_fps = false;
    let mut script_engine = Engine::new();

    loop {
        show_fps |= is_key_pressed(KeyCode::F5);

        menu.update();
        menu.draw();

        if show_fps {
            let fps = get_fps();
            let color = if fps >= 50 {
                GREEN
            } else if fps >= 30 {
                ORANGE
            } else {
                RED
            };
            draw_text(&format!("FPS: {fps}"), 0., 20., 20., color);
        }
        next_frame().await;
    }
}
