// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use colored::Colorize;
use std::env;
use std::process;
use std::process::ExitStatus;

use error::ErrorPage;
use macroquad::prelude::*;
use menu::Menu;
use miniquad::{
    conf::{Icon, Platform},
    date,
};
use script::{Engine, ScriptDir};
use ui::Logger;

mod cross;
mod error;
mod ffi;
mod menu;
mod script;
mod texture;
mod ui;

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
    #[cfg(target_os = "windows")]
    match process::Command::new("cmd").args(["/C", "cls"]).status() {
        Err(e) => eprintln!("ERROR: failed to clear screen: error = {e}"),
        Ok(s) if !s.success() => eprintln!("ERROR: failed to clear screen: code = {s}"),
        _ => {}
    }

    println!(
        "{name} {ver} {version}",
        name = env!("CARGO_PKG_NAME").bold(),
        ver = "version".bold(),
        version = env!("CARGO_PKG_VERSION").green()
    );
    println!(
        "Repository: {}",
        env!("CARGO_PKG_REPOSITORY").green().underline()
    );
    println!("=========================");

    let mut logger = Logger::new(true);
    let mut engine = Engine::new();

    logger.log("Scripting engine initialized");

    // Create dirs (if not exist)
    engine.ensure_dirs_exist().unwrap_or_else(|e| {
        logger.err(format!("Failed to create required directories: {e}"));
        process::exit(1);
    });
    logger.log(format!(
        "Required folders {:?}, {:?} and {:?} OK.",
        engine.global_dir, engine.script_dir, engine.asset_dir
    ));

    // Create readme
    let readme = "README.txt";
    match engine.create_readme(readme) {
        Ok(created) => logger.log(&format!("Created readme '{readme}' at {created:?}")),
        Err(e) => logger.err(format!("Failed to create readme '{readme}': {e}")),
    }

    // Write examples
    let mut warnings = vec![];
    logger.log("Writing examples...");
    if let Err(e) = engine.write_examples(&mut warnings) {
        logger.err(format!(
            "Failed to write examples due to the following errors: {e:#?}"
        ));
    }
    if warnings.len() > 0 {
        logger.warn("Encountered the following warnings:");
        for warning in warnings {
            logger.warn(format!(" - {warning}"));
        }
    }

    // Try to load scripts on startup.
    let mut start_error = None;
    let mut errors = vec![];
    if let Err(e) = engine.load_scripts(
        &mut logger,
        &mut errors,
        &[ScriptDir::Scripts, ScriptDir::Examples],
    ) {
        start_error = Some(ErrorPage::new(
            errors,
            format!("Failed to init scripts: {e}"),
        ));
    }

    // Report script count
    let scripts_count = engine.scripts.len();
    if scripts_count == 0 {
        logger.log(format!(
            "WARNING: No scripts ending in .rhai found in {:?}!",
            engine.script_dir
        ));
    } else {
        logger.log(format!("Loaded {scripts_count} scripts!"));
    }

    // Info messages
    println!();
    logger.note("Scripts     can be reloaded           with F5");
    logger.note("logging     can be disabled / toggled with F10");
    logger.note("FPS counter can be enabled  / toggled with F12");
    println!();

    // Watch for script changes
    let mut menu = Menu::new(engine, logger, readme);
    menu.error = start_error;

    loop {
        menu.update();
        menu.draw();

        next_frame().await;
    }
}
