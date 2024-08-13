use crate::game::Game;
use crate::games::game_of_life::theme;

use macroquad::prelude::*;

use super::grid::Grid;

pub struct GameOfLife {
    exit: bool,
    grid: Grid,
    theme: usize,
    mouse_pos: Vec2,
    screen_size: I64Vec2,
}

impl GameOfLife {
    fn size_changed(&mut self) -> bool {
        let (nw, nh) = (screen_width(), screen_height());
        if self.screen_size.x as f32 != nw || self.screen_size.y as f32 != nh {
            self.screen_size.x = nw as i64;
            self.screen_size.y = nh as i64;
            true
        } else {
            false
        }
    }
}

impl Game for GameOfLife {
    fn title(&self) -> &str {
        "Game of Life"
    }
    fn icon(&self) -> Option<&Texture2D> {
        // TODO: finish icon
        None
    }
    fn init() -> Self
    where
        Self: Sized,
    {
        Self {
            exit: false,
            grid: Grid::new(screen_width() as usize, screen_height() as usize),
            theme: 0,
            mouse_pos: Vec2::ZERO,
            screen_size: I64Vec2::ZERO,
        }
    }
    fn update(&mut self) {
        let dt = get_frame_time();

        if self.size_changed() {}
    }
    fn reset(&mut self) {}
    fn draw(&mut self) {}
    fn requested_exit(&self) -> bool {
        self.exit
    }
}
