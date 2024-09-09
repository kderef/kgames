use crate::{cross, texture};
use macroquad::prelude::*;
use rhai::Scope;
use std::path::PathBuf;

use crate::{
    error::ErrorPage,
    script::{Engine, ScriptDir},
    ui::{rgb, Logger, UI},
};

pub struct Menu<'a> {
    engine: Engine<'a>,
    selected: Option<usize>,
    logger: Logger,
    ui: UI,
    folder_icon: &'a Texture2D,
    refresh: &'a Texture2D,
    pub show_fps: bool,
    pub error: Option<ErrorPage>,
}

// File functions
impl<'a> Menu<'a> {}

impl<'a> Menu<'a> {
    pub fn new(engine: Engine<'a>, logger: Logger) -> Self {
        Self {
            engine,
            logger,
            show_fps: false,
            selected: None,
            error: None,
            folder_icon: texture::asset_store().get_texture("folder_open").unwrap(),
            refresh: texture::asset_store().get_texture("search_file").unwrap(),
            ui: UI::new(
                rgb(0.05, 0.05, 0.05),
                rgb(0.92156863, 0.85882353, 0.69803922),
                rgb(0.5, 0.5, 0.5),
            ),
        }
    }

    fn draw_selection(&mut self) {
        // FIXME: temporary solution
        for (i, name) in self.engine.scripts.iter().enumerate() {
            let i = i + 1;
            let (x, y, w, h) = (200., 200., 500., 50.);
            draw_rectangle(x, y + h * i as f32, w, h, BLACK);
            draw_text(
                &format!("{i} - {}", name.name()),
                x,
                y + h * i as f32 + h / 2.,
                h * 0.8,
                WHITE,
            );
            if is_key_pressed(unsafe { std::mem::transmute(KeyCode::Key0 as u16 + i as u16) }) {
                self.selected = Some(i - 1);
                return;
            }
        }
    }

    fn reload_scripts(&mut self) {
        self.logger.log("### Reloading scripts");
        let mut errors = vec![];
        if let Err(e) = self.engine.load_scripts(
            &mut self.logger,
            &mut errors,
            &[ScriptDir::Examples, ScriptDir::Scripts],
        ) {
            self.error = Some(ErrorPage::new(errors, e));
        }
    }

    /// Call update() of the script, and update menu state
    #[inline]
    pub fn update(&mut self) {
        // TODO: Make it report error if script reload caused error

        if is_key_pressed(KeyCode::F10) {
            let l = &mut self.logger;
            if l.enabled {
                l.warn("disabling logging! Reenable with F10");
                l.enabled = false;
            } else {
                l.enabled = true;
                l.log("Enabling logging!");
            };
        }
        self.show_fps ^= is_key_pressed(KeyCode::F12);

        if is_key_pressed(KeyCode::F5) {
            self.reload_scripts();
        }

        if self.error.is_some() {
            return;
        }

        if is_key_pressed(KeyCode::Escape) {
            self.selected = None;
        }

        if let Some(index) = self.selected {
            let script = &mut self.engine.scripts[index];
            let result =
                self.engine
                    .engine
                    .call_fn::<()>(&mut script.scope, &script.ast, "update", ());

            if let Err(e) = result {
                self.logger
                    .err(format!("Error while executings script -> update(): {e}"));
            }
        }
    }

    #[inline]
    fn draw_fps(&self) {
        let fps = get_fps();
        let color = match fps {
            50.. => GREEN,
            30.. => ORANGE,
            _ => RED,
        };
        draw_text(&format!("FPS: {fps}"), 0., 20., 20., color);
    }

    #[inline]
    pub fn draw(&mut self) {
        if let Some(ref mut err) = self.error {
            if !err.show(&mut self.engine, &mut self.logger) {
                self.error = None;
            }
            return;
        }

        if let Some(index) = self.selected {
            let script = &mut self.engine.scripts[index];
            let result =
                self.engine
                    .engine
                    .call_fn::<()>(&mut script.scope, &script.ast, "draw", ());

            if let Err(e) = result {
                self.logger
                    .err(format!("Error while executings script: {e}"));
            }

            if self.logger.enabled {
                self.draw_fps();
            }

            return;
        }
        self.draw_menu();
    }

    fn draw_menu(&mut self) {
        let (screen_w, screen_h) = (screen_width(), screen_height());

        const BG: Color = Color::new(0.11, 0.12, 0.12, 1.0);
        clear_background(BG);

        // Draw the Title
        let (sw, sh) = (screen_width(), screen_height());
        let title = "KGames";
        let title_size = (sw / 10.).clamp(60.0, sw);
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        let title_pos = vec2(sw / 2. - title_dims.width / 2., title_size);

        draw_text(title, title_pos.x, title_pos.y, title_size, WHITE);

        // Draw All the games.
        // TODO: draw games
        self.draw_selection();

        // Draw small subtext
        draw_text(
            concat!("v", env!("CARGO_PKG_VERSION")),
            0.0,
            sh - 5.0,
            15.0,
            GRAY,
        );

        //===== Draw UI =====//
        // Folder button
        let (w, h) = (60.0, 60.0);
        let mut bounds = Rect {
            x: screen_w - w - 10.0,
            y: 10.0,
            w,
            h,
        };

        if self.ui.button_icon(self.folder_icon, bounds) {
            if let Err(e) = cross::open_folder(&self.engine.global_dir) {
                self.logger.err(e);
            }
        }

        bounds.x -= w + 10.0;

        if self.ui.button_icon(self.refresh, bounds) {
            self.reload_scripts();
        }
        //===== Draw FPS =====//
        if self.show_fps {
            self.draw_fps();
        }
    }
}
