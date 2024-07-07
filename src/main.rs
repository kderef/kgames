#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use logic::menu::Menu;
use miniquad::conf::Platform;

#[cfg(debug_assertions)]
mod hotreload;

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

#[cfg(debug_assertions)]
#[macroquad::main(window)]
async fn main() {
    use logic::wrap::{ctx::Context, ctx_impl::DrawerImpl, MACROQUAD_CTX};
    dbg!();

    let lib = hotreload::load_library();
    let hotreloader = hotreload::HotReloader::new(&lib);
    let drawer: Box<dyn Context> = Box::new(DrawerImpl {});
    hotreloader.set_ctx(&drawer);

    let mut menu = Menu::new();

    loop {
        hotreloader.update(&mut menu);
        hotreloader.draw(&mut menu);
        draw_text(&format!("FPS: {}", get_fps()), 0., 20., 20., GREEN);
        next_frame().await;
    }
}

#[cfg(not(debug_assertions))]
#[macroquad::main(window)]
async fn main() {
    let mut menu = Menu::new();

    loop {
        menu.update();
        menu.draw();
        next_frame().await;
    }
}