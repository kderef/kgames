#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, allow(warnings))]

use engine::create_readme;
use engine::dirs;
use engine::ENGINE_NAME;
use menu::Console;
use std::env;
use std::process;

#[allow(unused_imports)]
use cross::cmd;
#[allow(unused_imports)]
use miniquad::conf::Icon;

use error::ErrorPage;
use macroquad::prelude::*;
use menu::Menu;
use miniquad::conf::Platform;
// use script::{Engine, ScriptDir};
use engine::engine_impl::*;
use engine::ScriptDir;
use engine::ScriptEngine;

mod cross;
mod engine;
mod error;
mod ffi;
mod menu;
mod texture;
mod ui;

pub mod key {
    use super::*;
    pub const REFRESH: KeyCode = KeyCode::F5;
    pub const FPS: KeyCode = KeyCode::F12;
    pub const CONSOLE: &[KeyCode] = &[KeyCode::GraveAccent, KeyCode::Semicolon];
}

#[cfg(not(target_os = "macos"))]
fn window_icon() -> Icon {
    // TODO: Add an icon from memory
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
    let mut console = Console::new();
    let dirs = dirs();

    console.print(format!(
        "{name} version {version}",
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION")
    ));

    console.print(format!("Repository: {}", env!("CARGO_PKG_REPOSITORY")));
    console.print(format!("Scripting engine: {}", ENGINE_NAME));
    console.print("=========================");

    let mut engine = Engine::new();

    console.log("Scripting engine initialized");

    engine.expose_layer();
    console.log("Prepared scripting interface");

    // Create dirs (if not exist)
    dirs.create().unwrap_or_else(|e| {
        console.err(format!("Failed to create required directories: {e}"));
        process::exit(1);
    });
    console.log(format!(
        "Required folders {:?}, {:?} and {:?} OK.",
        dirs.root, dirs.scripts, dirs.assets
    ));

    // Create readme
    let readme = "README.txt";
    match create_readme(readme) {
        Ok(created) => console.log(&format!("Created readme '{readme}' at {created:?}")),
        Err(e) => console.err(format!("Failed to create readme '{readme}': {e}")),
    }

    // Write examples
    let mut warnings = vec![];
    console.log("Writing examples...");
    if let Err(e) = engine.write_examples(&mut warnings) {
        console.err(format!(
            "Failed to write examples due to the following errors: {e:#?}"
        ));
    }
    if warnings.len() > 0 {
        console.warn("Encountered the following warnings while writing examples:");
        for warning in warnings {
            console.warn(format!(" - {warning}"));
        }
    }

    // Try to load scripts on startup.
    let mut start_error = None;
    let mut errors = vec![];
    if let Err(e) = engine.load_scripts(
        &mut console,
        &mut errors,
        &[ScriptDir::Scripts, ScriptDir::Examples],
    ) {
        start_error = Some(ErrorPage::new(
            errors,
            format!("Failed to init scripts: {e}"),
        ));
    }

    // Report script count
    let scripts_count = engine.scripts().len();
    if scripts_count == 0 {
        console.log(format!(
            "WARNING: No scripts ending in .rhai found in {:?}!",
            dirs.scripts
        ));
    } else {
        console.log(format!("Loaded {scripts_count} scripts!"));
    }

    // Info messages
    use key::*;

    println!();
    console.note(format!(
        "Scripts     can be reloaded           with {REFRESH:?}"
    ));
    console.note(format!(
        "FPS counter can be enabled  / toggled with {FPS:?}"
    ));
    println!();

    // Watch for script changes
    let mut menu = Menu::new(engine, console, readme);
    menu.error = start_error;

    loop {
        menu.update();
        menu.draw();
        menu.console();

        next_frame().await;
    }
}
