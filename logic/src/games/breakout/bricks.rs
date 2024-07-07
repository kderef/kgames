use macroquad::prelude::*;

use crate::games::ball::Ball;

#[derive(Default, Debug, Clone, Copy)]
pub struct Brick(bool);

impl Brick {
    pub const HEIGHT: f32 = 30.0;
    pub const SPACING: f32 = 10.0;
    pub fn destroy(&mut self) {
        self.0 = false;
    }
    pub fn destroyed(&self) -> bool {
        !self.0
    }
    pub fn alive(&self) -> bool {
        self.0
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Bricks<const R: usize, const C: usize> {
    pub bricks: [[Brick; C]; R],
}

impl<const R: usize, const C: usize> Bricks<R, C> {
    pub const fn new() -> Self {
        Self {
            bricks: [[Brick(true); C]; R],
        }
    }
    pub const fn rows(&self) -> usize {
        R
    }
    pub const fn cols(&self) -> usize {
        C
    }
    #[inline]
    pub fn brick_width(&self, screen_width: f32, spacing: f32) -> f32 {
        (screen_width - (spacing * (C + 1) as f32)) / C as f32
    }

    pub fn brick_hit_by_ball(&self, ball: &Ball<R>, brick_w: f32) -> Option<(usize, usize)> {
        let mut y = Brick::SPACING;
        for row in 0..R {
            let mut x = Brick::SPACING;
            for col in 0..C {
                let brick = Rect::new(x, y, brick_w, Brick::HEIGHT);
                if self.bricks[row][col].0 {
                    if brick.contains(ball.pos)
                        || brick.contains(ball.pos - ball.radius())
                        || brick.contains(ball.pos + ball.radius())
                    {
                        return Some((row, col));
                    }
                }
                x += brick_w + Brick::SPACING;
            }
            y += Brick::HEIGHT + Brick::SPACING;
        }

        None
    }
}
