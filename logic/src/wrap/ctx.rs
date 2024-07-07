use macroquad::{camera::Camera, prelude as mq};
use super::prelude::*;

pub trait Context {
    // Get information
    fn screen_width(&self) -> f32;
    fn screen_height(&self) -> f32;
    fn get_frame_time(&self) -> f32;

    // Input
    fn mouse_position(&self) -> (f32, f32);
    fn mouse_delta_position(&self) -> Vec2;
    fn is_key_down(&self, key: KeyCode) -> bool;
    fn is_key_pressed(&self, key: KeyCode) -> bool;
    fn is_key_released(&self, key: KeyCode) -> bool;
    fn is_mouse_button_down(&self, mb: MouseButton) -> bool;
    fn is_mouse_button_pressed(&self, mb: MouseButton) -> bool;
    fn is_mouse_button_released(&self, mb: MouseButton) -> bool;

    // Drawing
    fn clear_background(&self, color: Color);
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color);
    fn draw_rectangle_ex(&self, x: f32, y: f32, w: f32, h: f32, params: DrawRectangleParams);
    fn draw_circle(&self, x: f32, y: f32, radius: f32, color: Color);
    fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, thick: f32, color: Color);
    
    // Camera
    fn set_default_camera(&self);
    fn set_camera(&self, camera: &dyn Camera);

    // Textures
    fn render_target(&self, w: u32, h: u32) -> RenderTarget;
    fn empty_texture(&self) -> Texture2D;

    // Text
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color);
    fn measure_text(&self, text: &str, font: Option<&mq::Font>, font_size: u16, font_scale: f32) -> TextDimensions;
}