mod draw;
mod games;
mod settings;
mod update;

use crate::error::ErrorPage;
use crate::texture::*;
use crate::ui::rgb;
use crate::ui::Logger;
use crate::ui::{Dialog, UI};
pub use console::*;
use engine::dirs;
use engine::GameScript;
use engine::ScriptEngine;
use macroquad::prelude::*;
use miniquad::window::dropped_file_bytes;
use miniquad::window::dropped_file_count;
use miniquad::window::dropped_file_path;
use std::path::Path;
use std::path::PathBuf;

use fuzzy_matcher::skim::SkimMatcherV2;
// use fuzzy_matcher::FuzzyMatcher;

#[allow(unused)]
pub struct DroppedFile {
    path: PathBuf,
    contents: Vec<u8>,
}
#[allow(unused)]
impl DroppedFile {
    pub fn gather() -> Vec<Self> {
        let dropped_count = dropped_file_count();
        let mut dropped = Vec::with_capacity(dropped_count);

        for i in 0..dropped_count {
            dropped.push(Self {
                path: dropped_file_path(i).unwrap(),
                contents: dropped_file_bytes(i).unwrap(),
            });
        }

        dropped
    }
}

pub enum State {
    Menu,
    Settings,
    Games,
    Playing(usize),
}

pub struct Menu<'a, E: ScriptEngine> {
    engine: E,
    ui: UI,

    // Console
    console: Console,
    cvars: Cvars,

    // Icons
    background: Color,
    folder_icon: &'a Texture2D,
    refresh: &'a Texture2D,
    help: &'a Texture2D,

    // Files
    readme: PathBuf,

    // State
    pub state: State,
    dialog: Option<Dialog<'a>>,
    key_entered: bool,

    // Fzf
    matcher: SkimMatcherV2,
    matches: Vec<usize>,

    pub show_fps: bool,
    pub error: Option<ErrorPage>,
}

impl<'a, E: ScriptEngine> Menu<'a, E> {
    pub fn new(engine: E, console: Console, readme_name: impl AsRef<Path>) -> Self {
        let dirs = dirs();
        Self {
            show_fps: false,
            error: None,

            // Console
            console,
            cvars: Cvars::default(),

            // Icons
            background: rgb(0.11, 0.12, 0.12),
            folder_icon: asset_store().get_texture("folder_open_file").unwrap(),
            refresh: asset_store().get_texture("search_file").unwrap(),
            help: asset_store().get_texture("help_book").unwrap(),
            // Files
            readme: dirs.root.join(readme_name),

            // State
            state: State::Menu,
            dialog: None,
            key_entered: false,

            // Fzf
            matcher: SkimMatcherV2::default(),
            matches: vec![],

            ui: {
                let mut ui = UI::new(
                    rgb(0.05, 0.05, 0.05),
                    rgb(0.92156863, 0.85882353, 0.69803922),
                    rgb(0.5, 0.5, 0.5),
                );
                ui.bg_hover = GRAY;
                ui
            },

            engine,
        }
    }
}
