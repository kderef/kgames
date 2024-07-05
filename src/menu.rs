use crate::{
    game::Game,
    game_objects,
    games::{self, breakout::Breakout, pong::Pong},
};
use macroquad::prelude::*;

pub struct Menu {
    games: Vec<Box<dyn Game>>,
    selected: Option<usize>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            games: game_objects! {
                Breakout, Pong,
            },
            selected: None,
        }
    }
    pub fn update(&mut self) {
        if let Some(game) = self.selected {
            self.games[game].update();
        }
    }
    pub fn draw(&mut self) {
        if let Some(game) = self.selected {
            self.games[game].draw();
            return;
        }

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
        self.draw_games(title_size + 20.0);
    }
    fn draw_game(&self, bounds: Rect, g: &Box<dyn Game>) {
        const BG: Color = Color::new(0.1, 0.1, 0.1, 1.0);
        const BORDER: Color = Color::new(0.92, 0.85, 0.86, 1.0);
        const BORDER_THICK: f32 = 1.0;
        const TITLE_BG: Color = Color::new(0.05, 0.05, 0.05, 1.0);
        const TITLE_SIZE: f32 = 20.0;

        let (x, y, w, h) = (bounds.x, bounds.y, bounds.w, bounds.h);
        let text_y = y + h - TITLE_SIZE;

        draw_rectangle(x, y, w, h, BG);
        draw_rectangle(x, text_y, w, TITLE_SIZE, TITLE_BG);
        draw_text(g.title(), x + BORDER_THICK, text_y + TITLE_SIZE/1.5, TITLE_SIZE, WHITE);

        // Draw icon
        draw_rectangle_lines(x, y, w, h, BORDER_THICK, BORDER);
        if let Some(icon) = g.icon() {
            draw_texture_ex(
                icon,
                x + BORDER_THICK * 2.,
                y + BORDER_THICK * 2.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(
                        vec2(w - BORDER_THICK * 2., h - BORDER_THICK * 2.)
                    ),
                    ..Default::default()
                }
            )
        }
    }
    fn draw_games(&mut self, from_y: f32) {
        let (sw, sh) = (screen_width(), screen_height());

        let spacing = 10.0;
        let games_x = 4;
        let games_count = self.games.len();

        let mut game_bounds = Rect::new(
            spacing,
            spacing + from_y,
            (sw - (spacing * (games_x as f32 + 1.0))) / games_x as f32,
            100.0,
        );

        for (i, game) in self.games.iter().enumerate() {
            self.draw_game(game_bounds, game);
            game_bounds.x += spacing + game_bounds.w;
            if (i + 1) % games_x == 0 {
                game_bounds.x = spacing;
                game_bounds.y += game_bounds.h + spacing;
            }
        }
    }
}
