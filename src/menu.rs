use crate::{
    game::Game, game_objects, games::{self, breakout::Breakout, pong::Pong}
};
use macroquad::prelude::*;

pub struct Menu {
    games: Vec<Box<dyn Game>>,
    selected: Option<usize>
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
        let title_size = sw / 10.;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(title, sw/2. - title_dims.width/2., title_size, title_size, WHITE);

        // Draw All the games.
        self.draw_games(title_size);
    }
    fn draw_game(&self, bounds: Rect, g: &Box<dyn Game>) {
        draw_rectangle(
            bounds.x, bounds.y, bounds.w, bounds.h, RED
        );
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
            100.0
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
