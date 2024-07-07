use crate::wrap::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Ball<const R: usize> {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl<const R: usize> Ball<R> {
    pub const fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Self {
            pos: vec2(x, y),
            vel: vec2(vx, vy)
        }
    }
    pub const fn radius(&self) -> f32 {
        R as f32
    }
    pub fn draw(&self, color: Color) {
        draw_circle(self.pos.x, self.pos.y, R as f32, color);
    }

    pub fn right(&self) -> f32 {
        self.pos.x + self.radius()
    }
    pub fn left(&self) -> f32 {
        self.pos.x - self.radius()
    }
    pub fn up(&self) -> f32 {
        self.pos.y - self.radius()
    }
    pub fn down(&self) -> f32 {
        self.pos.y + self.radius()
    }
}