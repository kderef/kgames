use crate::{
    game::Game,
    games::{ball::Ball, breakout::bricks::Brick}, ui::{self, ExtendedDraw}, wrap::{ctx::Context, screen},
};
use crate::wrap::prelude::*;
use super::bricks::Bricks;

pub struct Breakout {
    icon: Texture2D,
    ball: Ball<5>,
    paddle: Rect,
    bricks: Bricks<5, 10>,
    gameover: bool,
    exit: bool,
}

impl Breakout {
    const PADDLE_WIDTH: f32 = 100.0;
    const PADDLE_HEIGHT: f32 = 10.0;
    const PADDLE_SPEED: f32 = 110.0;
    const BALL_RADIUS: f32 = 5.0;

    fn update_paddle(&mut self, screen: Vec2, dt: f32) {
        self.paddle.y = screen.y - Self::PADDLE_HEIGHT - 2.0;

        // Take input
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.paddle.x = (self.paddle.x - Self::PADDLE_SPEED * dt).max(0.0);
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.paddle.x =
                (self.paddle.x + Self::PADDLE_SPEED * dt).min(screen.x - Self::PADDLE_WIDTH);
        }
    }

    fn paddle_collides_with_ball(&self) -> bool {
        let col_x =
            self.ball.right() >= self.paddle.x && self.ball.left() <= self.paddle.x + self.paddle.w;
        let col_y =
            self.ball.down() >= self.paddle.y && self.ball.up() <= self.paddle.y + self.paddle.h;

        col_x && col_y
    }

    fn draw_game_over(&mut self, ctx: &dyn Context) {
        const OVERLAY: Color = Color::new(0.0, 0.0, 0.0, 0.5);
        let screen = Rect::new(0.0, 0.0, ctx.screen_width(), ctx.screen_height());
        ctx.draw_rectangle(screen.x, screen.y, screen.w, screen.h, OVERLAY);

        // Draw the title
        let center = screen.center();

        let gameover_text = "Game Over!";
        let gameover_ftsz = (screen.w / 8.0).clamp(40.0, 100.0);
        let gameover_dims = ctx.measure_text(gameover_text, None, gameover_ftsz as u16, 1.0);
        ctx.draw_text(
            gameover_text,
            center.x - gameover_dims.width / 2.0,
            center.y - gameover_ftsz,
            gameover_ftsz,
            WHITE
        );
        // Draw the subtext
        let bricks_left = self.bricks.bricks.iter().flatten().map(|b| b.alive()).count();
        let subtext = format!("Bricks left: {bricks_left}");
        let subtext_ftsz = gameover_ftsz / 2.0;
        let subtext_dims = ctx.measure_text(&subtext, None, subtext_ftsz as u16, 1.0);
        let subtext_y = center.y - gameover_ftsz + subtext_ftsz;
        ctx.draw_text(
            &subtext, center.x - subtext_dims.width / 2.0,
            subtext_y,
            subtext_ftsz, LIGHTGRAY
        );

        // Draw the buttons
        const SPACING: f32 = 10.0;
        
        let button_width = screen.w / 2.5;
        let button_ftsz = subtext_ftsz;
        let button_y = subtext_y + subtext_ftsz + SPACING;
        let button_height = button_ftsz * 2.0;
    
        let mut bounds = Rect {
            x: center.x - button_width / 2.0,
            y: button_y,
            w: button_width,
            h: button_height
        };
        if ui::button("Play Again", bounds, button_ftsz, ctx) {
            self.reset(ctx);
        }
        bounds.y += button_height + SPACING;
        if ui::button("Exit To Menu", bounds, button_ftsz, ctx) {
            self.exit = true;
        }
    }
}

impl Game for Breakout {
    fn requested_exit(&self) -> bool {
        self.exit
    }
    fn init(ctx: &dyn Context) -> Self {
        dbg!();
        let ad = screen().as_ref();
        println!("{:p}", ad);

        Self {
            icon: texture_empty(),
            ball: Ball::new(screen_width() / 2.0, screen_height() - Self::PADDLE_HEIGHT - 10.0, 100.0, -100.0),
            paddle: Rect::new(
                screen_width() / 2.0 - Self::PADDLE_WIDTH / 2.0,
                screen_height() - Self::PADDLE_HEIGHT - 2.0,
                Self::PADDLE_WIDTH,
                Self::PADDLE_HEIGHT,
            ),
            bricks: Bricks::new(),
            gameover: false,
            exit: false,
        }
    }

    fn title(&self) -> &str {
        "Breakout"
    }

    fn reset(&mut self, ctx: &dyn Context) {
        self.ball = Ball::new(screen_width() / 2.0, screen_height() / 2.0, 100.0, 100.0);
        self.paddle = Rect::new(
            screen_width() / 2.0 - Self::PADDLE_WIDTH / 2.0,
            screen_height() - Self::PADDLE_HEIGHT - 2.0,
            Self::PADDLE_WIDTH,
            Self::PADDLE_HEIGHT,
        );
        self.bricks = Bricks::new();
        self.gameover = false;
        self.exit = false;
    }

    fn draw(&mut self, ctx: &dyn Context) {
        const BG: Color = Color::new(0.2, 0.2, 0.2, 1.0);
        const PADDLE: Color = Color::new(0.9, 0.9, 0.9, 1.0);

        clear_background(BG);

        self.ball.draw(WHITE);
        self.paddle.draw(PADDLE);

        // Draw the bricks
        let screen_w = screen_width();
        let rows = self.bricks.rows();
        let cols = self.bricks.cols();
        let brick_width = self.bricks.brick_width(screen_w, Brick::SPACING);

        let mut y = Brick::SPACING;
        for row in 0..rows {
            let mut x = Brick::SPACING;
            for col in 0..cols {
                if !self.bricks.bricks[row][col].destroyed() {
                    ctx.draw_rectangle(x, y, brick_width, Brick::HEIGHT, RED);
                }
                x += brick_width + Brick::SPACING;
            }
            y += Brick::HEIGHT + Brick::SPACING;
        }

        if self.gameover {
            self.draw_game_over(ctx);
        }
    }

    fn update(&mut self, ctx: &dyn Context) {
        let screen: Vec2 = (screen_width(), screen_height()).into();
        let dt = ctx.get_frame_time();

        if self.gameover {
            self.paddle.y = screen.y - Self::PADDLE_HEIGHT - 2.0;
            return;
        }

        // Check for Game Over
        if self.ball.down() > screen.y {
            self.gameover = true;
            return;
        }

        // Update Ball Position
        let ball_deflected = self.paddle_collides_with_ball();
        if self.ball.right() >= screen.x || self.ball.left() <= 0.0 {
            self.ball.vel.x *= -1.0;
        }
        let ball_out_of_bounds_y = self.ball.down() >= screen.y || self.ball.up() <= 0.0;
        let brick_touched = self.bricks.brick_hit_by_ball(&self.ball, self.bricks.brick_width(screen.x, Brick::SPACING));
        if let Some((x, y)) = brick_touched {
            self.bricks.bricks[x][y].destroy();
            self.ball.vel.y *= -1.0;
        }

        if ball_out_of_bounds_y || ball_deflected {
            self.ball.vel.y *= -1.0;
        }
        self.ball.pos += self.ball.vel * dt;
        self.ball.pos.x = self
            .ball
            .pos
            .x
            .clamp(Self::BALL_RADIUS, screen.x - Self::BALL_RADIUS);

        self.update_paddle(screen, dt);
    }
}
