use crate::game::Game;
use macroquad::prelude::*;

#[derive(Default, Clone, Copy)]
struct Brick(bool);
impl Brick {
    fn destroy(&mut self) {
        self.0 = false;
    }
}

struct Bricks<const R: usize, const C: usize> {
    bricks: [[Brick; C]; R],
}
impl<const R: usize, const C: usize> Bricks<R, C> {
    fn new() -> Self {
        Self {
            bricks: [[Brick(true); C]; R],
        }
    }
}

pub struct Breakout {
    icon: Texture2D,
    ball: Vec2,
    ball_v: Vec2,
    paddle: Rect,
    bricks: Bricks<5, 10>,
    gameover: bool,
}

impl Breakout {
    const PADDLE_WIDTH: f32 = 100.0;
    const PADDLE_HEIGHT: f32 = 10.0;
    const PADDLE_SPEED: f32 = 90.0;
    const BALL_RADIUS: f32 = 5.0;
}

impl Game for Breakout {
    fn init() -> Self {
        Self {
            icon: Texture2D::empty(),
            ball_v: vec2(100.0, 100.0),
            paddle: Rect::new(
                screen_width() / 2. - Self::PADDLE_WIDTH / 2.,
                screen_height() - Self::PADDLE_HEIGHT - 2.0,
                Self::PADDLE_WIDTH,
                Self::PADDLE_HEIGHT,
            ),
            ball: vec2(screen_width() / 2., screen_height() / 2.),
            bricks: Bricks::new(),
            gameover: false,
        }
    }
    fn title(&self) -> &'static str {
        "Breakout"
    }
    fn draw(&self) {
        const BG: Color = Color::new(0.2, 0.2, 0.2, 1.0);
        const PADDLE: Color = Color::new(0.9, 0.9, 0.9, 1.0);

        clear_background(BG);

        draw_circle(self.ball.x, self.ball.y, Self::BALL_RADIUS, WHITE);
        draw_rectangle(self.paddle.x, self.paddle.y, self.paddle.w, self.paddle.h, PADDLE);

        if self.gameover {
            const OVERLAY: Color = Color::new(0.0, 0.0, 0.0, 0.5);
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), OVERLAY);
            // TODO: draw game over text and buttons.
        }
    }
    fn update(&mut self) {
        let screen: Vec2 = (screen_width(), screen_height()).into();
        let dt = get_frame_time();

        // Check for Game Over
        if self.ball.y > screen.y - Self::BALL_RADIUS {
            self.gameover = true;
            return;
        }

        // Update Ball Position
        if self.ball.x + Self::BALL_RADIUS >= screen.x || self.ball.x - Self::BALL_RADIUS <= 0.0 {
            self.ball_v.x *= -1.0;
        }
        if self.ball.y + Self::BALL_RADIUS >= screen.y || self.ball.y - Self::BALL_RADIUS <= 0.0 {
            self.ball_v.y *= -1.0;
        }
        self.ball += self.ball_v * dt;
        self.ball.x = self.ball.x.clamp(Self::BALL_RADIUS, screen.x - Self::BALL_RADIUS);


        self.paddle.y = screen.y - Self::PADDLE_HEIGHT - 2.0;

        // Take input
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.paddle.x = (self.paddle.x - Self::PADDLE_SPEED * dt).max(0.0);
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.paddle.x = (self.paddle.x + Self::PADDLE_SPEED * dt).min(screen.x - Self::PADDLE_WIDTH);
        }
    }
}
