use crate::game::Game;
use macroquad::prelude::*;

pub struct Pong {
    icon: RenderTarget
}

impl Game for Pong {
    fn init() -> Self {
        Self {
            icon: {
                let (w, h) = (500.0, 250.0);
                let target = render_target(w as u32, h as u32);

                let camera = Camera2D {
                    render_target: Some(target),
                    target: vec2(w / 2., h / 2.),
                    zoom: vec2(1. / w, 1. / h),
                    ..Default::default()
                };
                set_camera(&camera);

                clear_background(BLACK);
                draw_circle(w / 2.0, h / 2.0, 50.0, WHITE);

                set_default_camera();

                camera.render_target.unwrap()
            }
        }
    }
    fn icon(&self) -> Option<&Texture2D> {
        Some(&self.icon.texture)
    }
    fn title(&self) -> &'static str {
        "Pong"
    }
    fn draw(&self) {
        
    }
    fn update(&mut self) {
        
    }
}