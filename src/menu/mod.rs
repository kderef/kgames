mod draw;
mod update;

use crate::error::ErrorPage;
use crate::script::Engine;
use crate::texture::*;
use crate::ui::rgb;
use crate::ui::Logger;
use crate::ui::UI;
use macroquad::prelude::*;
use std::path::PathBuf;

pub struct Menu<'a> {
    engine: Engine<'a>,
    pub selected: Option<usize>,
    logger: Logger,
    ui: UI,
    // Icons
    folder_icon: &'a Texture2D,
    refresh: &'a Texture2D,
    help: &'a Texture2D,

    // Files
    readme: PathBuf,

    pub show_fps: bool,
    pub error: Option<ErrorPage>,
}

impl<'a> Menu<'a> {
    pub fn new(engine: Engine<'a>, logger: Logger, readme_name: &str) -> Self {
        Self {
            logger,
            show_fps: false,
            selected: None,
            error: None,
            // Icons
            folder_icon: asset_store().get_texture("folder_open").unwrap(),
            refresh: asset_store().get_texture("search_file").unwrap(),
            help: asset_store().get_texture("help_book").unwrap(),
            // Files
            readme: engine.global_dir.join(readme_name),

            ui: UI::new(
                rgb(0.05, 0.05, 0.05),
                rgb(0.92156863, 0.85882353, 0.69803922),
                rgb(0.5, 0.5, 0.5),
            ),

            engine,
        }
    }
}
