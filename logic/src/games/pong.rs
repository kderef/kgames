use crate::game::Game;
use crate::wrap::ctx::Context;
use crate::wrap::prelude::*;

use super::ball::Ball;

#[derive(Default)]
pub struct Pong {
    icon: Option<RenderTarget>,
    ball: Ball<5>,
    exit: bool,
}

impl Pong {
    fn new(ctx: &dyn Context) -> Self {
        Self {
            ball: Ball::new(ctx.screen_width() / 2., ctx.screen_height() /2., 50.0, 50.0),
            ..Default::default()
        }
    }
}

impl Game for Pong {
    fn init(ctx: &dyn Context) -> Self {
        let mut s = Self::new(ctx);
        s.icon = {
            let (w, h) = (500.0, 250.0);
            let (cx, cy) = (w/2., h/2.);
            let target = ctx.render_target(w as u32, h as u32);

            let camera = Camera2D {
                render_target: Some(target),
                target: vec2(cx, cy),
                zoom: vec2(1.7 / w, 1.7 / h),
                ..Default::default()
            };
            ctx.set_camera(&camera);

            ctx.clear_background(BLACK);
            ctx.draw_line(cx, 0.0, cx, h, 5.0, GRAY);
            ctx.draw_circle(cx, cy, 15.0, WHITE);

            // Draw paddles
            let paddle_h = cy;
            let paddle_w = paddle_h / 5.0;
            let pad = 3.0;

            ctx.draw_rectangle(pad, cy / 2.0, paddle_w, paddle_h, WHITE);
            ctx.draw_rectangle(w - pad, h - cy, paddle_w, paddle_h, WHITE);

            ctx.set_default_camera();

            Some(camera.render_target.unwrap())
        };
        s
    }
    fn icon(&self) -> Option<&Texture2D> {
        self.icon.as_ref().map(|i| &i.texture)
    }
    fn title(&self) -> &'static str {
        "Pong"
    }
    fn draw(&mut self, ctx: &dyn Context) {
        const BG: Color = BLACK;
        const BALL: Color = WHITE;
        const PADDLE: Color = WHITE;

        ctx.clear_background(BG);
    }
    fn update(&mut self, ctx: &dyn Context) {
        
    }
    fn requested_exit(&self) -> bool {
        self.exit
    }
    fn reset(&mut self, ctx: &dyn Context) {
        *self = Self::new(ctx);
    }
}