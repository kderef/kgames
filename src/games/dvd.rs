use crate::game::Game;
use macroquad::{color, prelude::*};

pub struct DvD {
    exit: bool,
    logo: Texture2D,
    logo_size: Vec2,
    // State
    pos: Vec2,
    vel: Vec2,
    inverted: bool,
    rainbow: bool,
    paused: bool,
    passed_time: f32,
}

impl Game for DvD {
    fn title(&self) -> &str {
        "DvD bouncy"
    }
    fn icon(&self) -> Option<&Texture2D> {
        None
    }
    fn init() -> Self
    where
        Self: Sized,
    {
        let logo = Texture2D::from_file_with_format(
            include_bytes!("../../res/DVD_logo.png"),
            Some(ImageFormat::Png),
        );
        let logo_size = logo.size();

        Self {
            vel: Vec2::new(250., 250.),
            exit: false,
            pos: Vec2::new(100., 100.),
            logo,
            logo_size,
            inverted: true,
            rainbow: true,
            paused: false,
            passed_time: 0.,
        }
    }
    fn update(&mut self) {
        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Escape => {
                    self.exit = true;
                }
                KeyCode::Space => {
                    self.paused = !self.paused;
                }
                KeyCode::I => {
                    self.inverted = !self.inverted;
                }
                KeyCode::C => {
                    self.rainbow = !self.rainbow;
                }
                _ => {}
            }
        }
        let dt = get_frame_time();
        let screen_size = vec2(screen_width(), screen_height());
        let (screen_w, screen_h) = screen_size.into();

        self.passed_time += dt;
        if self.passed_time > 36. {
            self.passed_time = 0.;
        }

        if self.paused {
            return; // Do not update position
        }

        // TODO: DvD clips into sides
        // Update pos
        self.pos = (self.vel * dt) + self.pos.clamp(Vec2::ZERO, screen_size - self.logo_size);

        if self.pos.x + self.logo_size.x >= screen_w || self.pos.x <= 0. {
            self.vel.x *= -1.;
        }
        if self.pos.y + self.logo_size.y >= screen_h || self.pos.y <= 0. {
            self.vel.y *= -1.;
        }
    }
    fn reset(&mut self) {
        *self = Self::init();
    }
    fn draw(&mut self) {
        let tint = if self.rainbow {
            let v = self.passed_time / 36.;
            if self.inverted {
                color::hsl_to_rgb(v, 5., v.max(0.5))
            } else {
                color::hsl_to_rgb(self.passed_time / 5., 0.6, 1.0)
            }
        } else {
            if self.inverted {
                WHITE
            } else {
                BLACK
            }
        };

        clear_background(if self.inverted { BLACK } else { WHITE });

        if self.paused {
            // TODO: draw paused text
        }

        draw_texture(&self.logo, self.pos.x, self.pos.y, tint);
    }
    fn requested_exit(&self) -> bool {
        self.exit
    }
}
