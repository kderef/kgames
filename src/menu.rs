use macroquad::prelude::*;

pub struct Menu {}

impl Menu {
    const GAME_TITLE_SIZE: f32 = 20.0;

    pub fn new() -> Self {
        Self {}
    }

    #[inline]
    pub fn update(&mut self) {}

    #[inline]
    pub fn draw(&mut self) {
        const BG: Color = Color::new(0.11, 0.12, 0.12, 1.0);
        clear_background(BG);

        // Draw the Title
        let (sw, sh) = (screen_width(), screen_height());
        let title = "KGames";
        let title_size = (sw / 10.).clamp(60.0, sw);
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            sw / 2. - title_dims.width / 2.,
            title_size,
            title_size,
            WHITE,
        );

        // Draw All the games.
        // TODO: draw games

        // Draw small subtext
        draw_text(
            concat!("v", env!("CARGO_PKG_VERSION")),
            0.0,
            sh - 5.0,
            15.0,
            GRAY,
        );
    }
}
