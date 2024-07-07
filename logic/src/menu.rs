use crate::{
    game::Game,
    game_objects,
    games::{self, breakout::Breakout, pong::Pong}, wrap::{ctx::Context, screen},
};
use macroquad::prelude::*;

#[repr(C)]
pub struct Menu {
    games: [Box<dyn Game>; 2],
    selected: Option<usize>,
    scroll: f32,
}

#[no_mangle]
pub extern "C" fn menu_update(menu: &mut Menu, ctx: &dyn Context) {
    if let Some(game) = menu.selected {
        menu.games[game].update();
        return;
    }
    let (_scroll_x, scroll_y) = mouse_wheel();
    let scroll_y = scroll_y.min(1.0);
    menu.scroll += scroll_y;
}

#[no_mangle]
pub extern "C" fn menu_draw(menu: &mut Menu, ctx: &dyn Context) {
    if let Some(game) = menu.selected {
        let g = &mut menu.games[game];
        g.draw();
        if g.requested_exit() {
            g.reset();
            menu.selected = None;
        }
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
    menu.draw_games(title_size + 20.0);

    // Draw small subtext
    draw_text(concat!("v", env!("CARGO_PKG_VERSION")), 0.0, sh - 5.0, 15.0, GRAY);
}

impl Menu {
    const GAME_TITLE_SIZE: f32 = 20.0;

    pub fn new() -> Self {
        Self {
            games: game_objects! [
                Breakout, Pong,
            ],
            selected: None,
            scroll: 0.0,
        }
    }
    #[inline]
    pub fn update(&mut self) {
        menu_update(self);
    }
    #[inline]
    pub fn draw(&mut self) {
        menu_draw(self);
    }

    /// Draw the game with an icon in 2:1 aspect.
    /// Returns if the game was clicked
    fn draw_game(&self, bounds: Rect, g: &Box<dyn Game>, mouse_pos: Vec2) -> bool {
        const BG: Color = Color::new(0.1, 0.1, 0.1, 1.0);
        const BORDER: Color = Color::new(0.92, 0.85, 0.86, 1.0);
        const BORDER_THICK: f32 = 1.0;
        const TITLE_BG: Color = Color::new(0.05, 0.05, 0.05, 1.0);

        let (x, y, w, h) = (bounds.x, bounds.y, bounds.w, bounds.h);
        let text_y = y + h - Self::GAME_TITLE_SIZE;

        draw_rectangle(x, y, w, h, BG);
        draw_rectangle(x, text_y, w, Self::GAME_TITLE_SIZE, TITLE_BG);
        draw_text(
            g.title(),
            x + BORDER_THICK,
            text_y + Self::GAME_TITLE_SIZE / 1.5,
            Self::GAME_TITLE_SIZE,
            WHITE,
        );

        // Draw icon
        draw_rectangle_lines(x, y, w, h, BORDER_THICK, BORDER);
        if let Some(icon) = g.icon() {
            draw_texture_ex(
                icon,
                x + BORDER_THICK * 2.,
                y + BORDER_THICK * 2.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        w - BORDER_THICK * 4.,
                        h - BORDER_THICK * 4. - Self::GAME_TITLE_SIZE,
                    )),
                    ..Default::default()
                },
            )
        }

        // Check if it was clicked
        bounds.contains(mouse_pos) && is_mouse_button_pressed(MouseButton::Left)
    }
    fn draw_games(&mut self, from_y: f32) {
        let (sw, _sh) = (screen_width(), screen_height());
        let mouse_pos: Vec2 = mouse_position().into();

        let spacing = 10.0;
        let games_x = 4;

        let game_w = (sw - (spacing * (games_x + 1) as f32)) / games_x as f32;
        let mut game_bounds = Rect::new(
            spacing,
            spacing + from_y,
            game_w,
            game_w / 2.0 + Self::GAME_TITLE_SIZE,
        );

        let game_dx = spacing + game_bounds.w;
        let game_dy = spacing + game_bounds.h;
        // Draw thumbnails
        for (i, game) in self.games.iter().enumerate() {
            if self.draw_game(game_bounds, game, mouse_pos) {
                self.selected = Some(i);
            }
            game_bounds.x += game_dx;
            if (i + 1) % games_x == 0 {
                game_bounds.x = spacing;
                game_bounds.y += game_dy;
            }
        }
    }
}
