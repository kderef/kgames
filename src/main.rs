#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    cell::OnceCell,
    process::{self, ExitCode},
    sync::{atomic::AtomicBool, mpsc},
    thread,
};

use macroquad::prelude::*;
use menu::Menu;
use miniquad::{
    conf::{Icon, Platform},
    window::blocking_event_loop,
};
use script::{Engine, Message};
use ui::Logger;

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

    let mut menu = Menu::new();
    let mut show_fps = false;
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

    // Disable logging before starting loop
    logger.log("TIP: to toggle logging, press F5");
    logger.enabled = false;

    // Watch for script changes
    let (tx, rx) = mpsc::channel();

    let watch_dir = script_engine.script_dir.clone();
    thread::spawn(move || script::watch(watch_dir, tx));

    loop {
        if let Ok(Some(msg)) = rx.try_recv() {
            match msg {
                Message::ReloadScripts => {
                    logger.log("Reloading scripts...");
                    if let Err(e) = script_engine.load_scripts(&mut logger) {
                        //
                    }
                }
                Message::Error(e) => {
                    // TODO
                    logger.log("Hello");
                }
            }
        }

        logger.enabled ^= is_key_pressed(KeyCode::F5);

        menu.update();
        menu.draw();

        if logger.enabled {
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
