mod draw;
mod games;
mod settings;
mod update;

use crate::error::ErrorPage;
use crate::script::Engine;
use crate::texture::*;
use crate::ui::rgb;
use crate::ui::Logger;
use crate::ui::{Dialog, UI};
use macroquad::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub enum State {
    Menu,
    Settings,
    Games,
    Playing(usize),
}

pub struct Menu<'a> {
    engine: Engine<'a>,
    logger: Logger,
    ui: UI,

    // Icons
    background: Color,
    folder_icon: &'a Texture2D,
    refresh: &'a Texture2D,
    help: &'a Texture2D,

    // Files
    readme: PathBuf,

    // State
    state: State,
    dialog: Option<Dialog<'a>>,

    pub show_fps: bool,
    pub error: Option<ErrorPage>,
}

impl<'a> Menu<'a> {
    pub fn new(engine: Engine<'a>, logger: Logger, readme_name: impl AsRef<Path>) -> Self {
        Self {
            logger,
            show_fps: false,
            error: None,
            // Icons
            background: rgb(0.11, 0.12, 0.12),
            folder_icon: asset_store().get_texture("folder_open_file").unwrap(),
            refresh: asset_store().get_texture("search_file").unwrap(),
            help: asset_store().get_texture("help_book").unwrap(),
            // Files
            readme: engine.global_dir.join(readme_name),

            // State
            state: State::Menu,
            dialog: None,

            ui: UI::new(
                rgb(0.05, 0.05, 0.05),
                rgb(0.92156863, 0.85882353, 0.69803922),
                rgb(0.5, 0.5, 0.5),
            ),

            engine,
        }
    }
}
