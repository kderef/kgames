use crate::game::Game;
use crate::games::game_of_life::cell::Cell;
use crate::games::game_of_life::theme;

use macroquad::prelude::*;

use super::grid::Grid;
use super::theme::Theme;

pub struct GameOfLife {
    exit: bool,
    paused: bool,
    help: bool,
    grid: Grid,
    grid_pos: U64Vec2,
    theme: Theme,
    screen_size: U64Vec2,
}

impl GameOfLife {
    fn size_changed(&mut self) -> bool {
        let (nw, nh) = (screen_width(), screen_height());
        if self.screen_size.x as f32 != nw || self.screen_size.y as f32 != nh {
            self.screen_size.x = nw as u64;
            self.screen_size.y = nh as u64;
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
            paused: false,
            help: false,
            grid: Grid::new(screen_width() as usize, screen_height() as usize),
            grid_pos: Default::default(),
            theme: Theme::Default,
            screen_size: Default::default(),
        }
    }
    fn update(&mut self) {
        let dt = get_frame_time();

        if self.size_changed() {
            let new_w = self.screen_size.x as usize / self.grid.scale;
            let new_h = self.screen_size.y as usize / self.grid.scale;
            self.grid.resize(new_w, new_h);
        }

        let mouse: Vec2 = mouse_position().into();
        let mouse = U64Vec2::new(mouse.x.floor() as u64, mouse.y.floor() as u64);

        self.grid_pos = mouse.clamp(
            U64Vec2::ZERO,
            (self.screen_size - 1) / self.grid.scale as u64,
        );

        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::C => self.grid.fill(Cell(false)),
                KeyCode::A => self.grid.fill(Cell(true)),
                KeyCode::T => self.theme.cycle(),
                KeyCode::R => self.grid.fill_random(),
                KeyCode::I => self.grid.invert(),
                _ => {}
            }
        }
    }
    fn reset(&mut self) {}
    fn draw(&mut self) {}
    fn requested_exit(&self) -> bool {
        self.exit
    }
}
