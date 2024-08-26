#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process;

use error::ErrorPage;
use macroquad::prelude::*;
use menu::Menu;
use miniquad::conf::{Icon, Platform};
use script::Engine;
use ui::Logger;

mod error;
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
    let mut logger = Logger::new(true);
    let mut script_engine = Engine::new();

    logger.log("UI and scripting engine initialized");

    script_engine.ensure_dirs_exist().unwrap_or_else(|e| {
        logger.log(format!("Failed to create required directories: {e}"));
        process::exit(1);
    });
    logger.log(format!(
        "Required directories in {:?} OK.",
        script_engine.global_dir
    ));

    let mut start_error = None;
    let mut errors = vec![];

    if let Err(e) = script_engine.load_scripts(&mut logger, &mut errors) {
        let ctx = format!("Failed to load scripts: {e}");
        logger.err(&ctx);
        start_error = Some(ErrorPage::new(errors, ctx));
    }

    // Disable logging before starting loop
    logger.log("TIP: to toggle logging, press F10");
    logger.enabled = false || cfg!(debug_assertions);

    // Watch for script changes
    let mut menu = Menu::new(script_engine, logger);
    menu.error = start_error;

    loop {
        menu.update();
        menu.draw();

        next_frame().await;
    }
}
